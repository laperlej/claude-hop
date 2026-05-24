use anyhow::Result;
use std::env;
use std::process::Command;

#[derive(Debug)]
pub enum Multiplexer {
    Tmux,
    Zellij,
    None,
}

pub fn detect() -> Multiplexer {
    if env::var("TMUX").is_ok() {
        Multiplexer::Tmux
    } else if env::var("ZELLIJ").is_ok() {
        Multiplexer::Zellij
    } else {
        Multiplexer::None
    }
}

pub fn current_pane_pid() -> Option<u32> {
    match detect() {
        Multiplexer::Tmux => {
            let output = Command::new("tmux")
                .args(["display-message", "-p", "#{pane_pid}"])
                .output()
                .ok()?;
            String::from_utf8_lossy(&output.stdout).trim().parse().ok()
        }
        _ => None,
    }
}

pub fn pid_in_pane(pane_pid: u32, target_pid: u32) -> bool {
    is_ancestor_of(pane_pid, target_pid)
}

pub fn jump_to_pid(pid: u32) -> Result<()> {
    match detect() {
        Multiplexer::Tmux => tmux_jump(pid),
        Multiplexer::Zellij => todo!("zellij jump"),
        Multiplexer::None => {
            eprintln!("no multiplexer detected, cannot jump (pid: {pid})");
            Ok(())
        }
    }
}

fn tmux_jump(target_pid: u32) -> Result<()> {
    let output = Command::new("tmux")
        .args(["list-panes", "-a", "-F", "#{pane_pid} #{session_name}:#{window_index}.#{pane_index}"])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        let mut parts = line.splitn(2, ' ');
        let Some(pane_pid_str) = parts.next() else { continue };
        let Some(target) = parts.next() else { continue };
        let Ok(pane_pid) = pane_pid_str.parse::<u32>() else { continue };

        if is_ancestor_of(pane_pid, target_pid) {
            Command::new("tmux")
                .args(["switch-client", "-t", target])
                .status()?;
            return Ok(());
        }
    }

    eprintln!("could not find tmux pane for pid {target_pid}");
    Ok(())
}

fn is_ancestor_of(ancestor: u32, mut pid: u32) -> bool {
    for _ in 0..30 {
        if pid == ancestor {
            return true;
        }
        if pid <= 1 {
            return false;
        }
        match parent_pid(pid) {
            Some(ppid) if ppid != pid => pid = ppid,
            _ => return false,
        }
    }
    false
}

#[cfg(target_os = "macos")]
fn parent_pid(pid: u32) -> Option<u32> {
    let output = std::process::Command::new("ps")
        .args(["-o", "ppid=", "-p", &pid.to_string()])
        .output()
        .ok()?;
    let s = String::from_utf8_lossy(&output.stdout);
    s.trim().parse().ok()
}

#[cfg(target_os = "linux")]
fn parent_pid(pid: u32) -> Option<u32> {
    let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    stat.split_whitespace().nth(3)?.parse().ok()
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn parent_pid(_pid: u32) -> Option<u32> {
    None
}
