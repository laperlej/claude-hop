use crate::multiplexer;
use crate::session::{self, Instance, Status};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::time::Duration;
use nucleo::Matcher;
use nucleo::pattern::{Atom, AtomKind, CaseMatching, Normalization};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::CrosstermBackend,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use std::io;

struct App {
    instances: Vec<Instance>,
    filtered: Vec<usize>,
    search: String,
    list_state: ListState,
}

impl App {
    fn new(instances: Vec<Instance>) -> Self {
        let filtered: Vec<usize> = (0..instances.len()).collect();
        let mut list_state = ListState::default();
        if !filtered.is_empty() {
            list_state.select(Some(0));
        }
        Self {
            instances,
            filtered,
            search: String::new(),
            list_state,
        }
    }

    fn update_filter(&mut self) {
        if self.search.is_empty() {
            self.filtered = (0..self.instances.len()).collect();
        } else {
            let mut matcher = Matcher::new(nucleo::Config::DEFAULT);
            let atom = Atom::new(
                &self.search,
                CaseMatching::Smart,
                Normalization::Smart,
                AtomKind::Fuzzy,
                false,
            );

            let mut scored: Vec<(usize, u16)> = self
                .instances
                .iter()
                .enumerate()
                .filter_map(|(i, inst)| {
                    let haystack = display_text(inst);
                    let mut buf = Vec::new();
                    let hay = nucleo::Utf32Str::new(&haystack, &mut buf);
                    atom.score(hay, &mut matcher).map(|s| (i, s))
                })
                .collect();

            scored.sort_by(|a, b| b.1.cmp(&a.1));
            self.filtered = scored.into_iter().map(|(i, _)| i).collect();
        }

        if self.filtered.is_empty() {
            self.list_state.select(None);
        } else {
            self.list_state.select(Some(0));
        }
    }

    fn move_down(&mut self) {
        if let Some(sel) = self.list_state.selected() {
            if sel + 1 < self.filtered.len() {
                self.list_state.select(Some(sel + 1));
            }
        }
    }

    fn move_up(&mut self) {
        if let Some(sel) = self.list_state.selected() {
            if sel > 0 {
                self.list_state.select(Some(sel - 1));
            }
        }
    }

    fn selected_instance(&self) -> Option<&Instance> {
        self.list_state
            .selected()
            .and_then(|sel| self.filtered.get(sel))
            .map(|&idx| &self.instances[idx])
    }

    fn refresh(&mut self) {
        let selected_pid = self.selected_instance().map(|i| i.pid);

        if let Ok(instances) = session::discover() {
            self.instances = instances;
            self.update_filter();

            if let Some(pid) = selected_pid {
                if let Some(pos) = self
                    .filtered
                    .iter()
                    .position(|&idx| self.instances[idx].pid == pid)
                {
                    self.list_state.select(Some(pos));
                }
            }
        }
    }
}

fn status_color(status: &Status) -> Color {
    match status {
        Status::Idle => Color::Green,
        Status::Busy => Color::Yellow,
        Status::Asking => Color::Cyan,
        Status::Waiting => Color::Magenta,
        Status::Unknown => Color::DarkGray,
    }
}

fn display_text(inst: &Instance) -> String {
    let label = inst
        .name
        .as_deref()
        .unwrap_or_else(|| inst.cwd.rsplit('/').next().unwrap_or(&inst.cwd));
    format!("{} {}", label, inst.cwd)
}

fn shorten_path(path: &str) -> String {
    if let Some(home) = dirs::home_dir() {
        if let Some(rest) = path.strip_prefix(home.to_str().unwrap_or("")) {
            return format!("~{rest}");
        }
    }
    path.to_string()
}

pub fn run() -> Result<()> {
    let instances = session::discover()?;
    if instances.is_empty() {
        eprintln!("no claude instances found");
        return Ok(());
    }

    let mut app = App::new(instances);

    terminal::enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let selected_pid = loop {
        terminal.draw(|f| render(f, &mut app))?;

        if event::poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Esc => break None,
                    KeyCode::Enter => break app.selected_instance().map(|i| i.pid),
                    KeyCode::Down => app.move_down(),
                    KeyCode::Up => app.move_up(),
                    KeyCode::Char('j')
                        if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                    {
                        app.move_down()
                    }
                    KeyCode::Char('k')
                        if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                    {
                        app.move_up()
                    }
                    KeyCode::Backspace => {
                        app.search.pop();
                        app.update_filter();
                    }
                    KeyCode::Char(c) => {
                        app.search.push(c);
                        app.update_filter();
                    }
                    _ => {}
                }
            }
        } else {
            app.refresh();
        }
    };

    terminal::disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    if let Some(pid) = selected_pid {
        multiplexer::jump_to_pid(pid)?;
    }

    Ok(())
}

fn render(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::vertical([Constraint::Min(1), Constraint::Length(3)]).split(f.area());

    render_list(f, app, chunks[0]);
    render_search(f, app, chunks[1]);
}

fn render_list(f: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .filtered
        .iter()
        .map(|&idx| {
            let inst = &app.instances[idx];
            let status = Span::styled(
                format!("{} ", inst.status.label()),
                Style::default().fg(status_color(&inst.status)),
            );
            let label = inst
                .name
                .as_deref()
                .unwrap_or_else(|| inst.cwd.rsplit('/').next().unwrap_or(&inst.cwd));
            let name = Span::styled(
                format!("{:<20}", label),
                Style::default().add_modifier(Modifier::BOLD),
            );
            let path = Span::styled(
                shorten_path(&inst.cwd),
                Style::default().fg(Color::DarkGray),
            );
            ListItem::new(Line::from(vec![status, name, path]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Claude Hop"))
        .highlight_style(Style::default().bg(Color::DarkGray));

    f.render_stateful_widget(list, area, &mut app.list_state);
}

fn render_search(f: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let input = Paragraph::new(format!("> {}", app.search))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(input, area);
}
