mod cli;
mod event;
mod multiplexer;
mod notification;
mod picker;
mod session;
mod setup;
mod status;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Pick => picker::run(),
        Command::Jump => {
            let instances = session::discover()?;
            let current_pane_pid = multiplexer::current_pane_pid();
            if let Some(inst) = instances.iter().find(|i| {
                i.status == session::Status::Idle
                    && !current_pane_pid
                        .map(|p| multiplexer::pid_in_pane(p, i.pid))
                        .unwrap_or(false)
            }) {
                multiplexer::jump_to_pid(inst.pid)?;
            } else {
                eprintln!("no idle instances to jump to");
            }
            Ok(())
        }
        Command::Event { kind } => event::handle(&kind),
        Command::Status => status::run(),
        Command::Setup => setup::run(),
    }
}
