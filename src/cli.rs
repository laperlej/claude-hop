use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "claude-hop", about = "Fuzzy picker for jumping between parallel Claude Code instances")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Open fuzzy picker to select and jump to a Claude instance
    Pick,
    /// Jump to the most recently idle instance (no UI)
    Jump,
    /// Hook target — receives Claude hook events, updates state and notifies
    Event {
        /// Event kind: idle, permission, start, end
        kind: String,
    },
    /// One-line status for tmux status bar
    Status,
    /// Print hook config and keybinding setup instructions
    Setup,
}
