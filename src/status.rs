use crate::multiplexer;
use crate::session::{self, Status};
use anyhow::Result;

pub fn run() -> Result<()> {
    let instances = session::discover()?;
    let current_pane_pid = multiplexer::current_pane_pid();

    let ready: Vec<_> = instances
        .iter()
        .filter(|i| matches!(i.status, Status::Idle | Status::Asking | Status::Waiting))
        .filter(|i| {
            !current_pane_pid
                .map(|p| multiplexer::pid_in_pane(p, i.pid))
                .unwrap_or(false)
        })
        .collect();

    if ready.is_empty() {
        return Ok(());
    }

    if ready.len() == 1 {
        let label = ready[0]
            .name
            .as_deref()
            .unwrap_or_else(|| ready[0].cwd.rsplit('/').next().unwrap_or("claude"));
        print!("⏎ {label}");
    } else {
        print!("⏎ {} ready", ready.len());
    }

    Ok(())
}
