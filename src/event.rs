use crate::notification;
use anyhow::Result;
use serde::Deserialize;
use std::env;
use std::io::{self, Read};
use std::process::Command;

#[derive(Deserialize, Default)]
struct HookInput {
    cwd: Option<String>,
}

pub fn handle(kind: &str) -> Result<()> {
    let mut stdin = String::new();
    io::stdin().read_to_string(&mut stdin).ok();
    let input: HookInput = serde_json::from_str(&stdin).unwrap_or_default();

    match kind {
        "idle" | "asking" => notify_if_not_focused(kind, &input),
        _ => Ok(()),
    }
}

fn notify_if_not_focused(kind: &str, input: &HookInput) -> Result<()> {
    if is_focused_pane() {
        return Ok(());
    }

    let project = input
        .cwd
        .as_deref()
        .and_then(|p| p.rsplit('/').next())
        .unwrap_or("Claude");

    let body = match kind {
        "asking" => format!("{project} needs your input"),
        _ => format!("{project} is ready"),
    };

    notification::notify("Claude Hop", &body)
}

fn is_focused_pane() -> bool {
    if env::var("TMUX").is_err() {
        return false;
    }

    let hook_pane = match env::var("TMUX_PANE") {
        Ok(p) => p,
        Err(_) => return false,
    };

    let active_pane = Command::new("tmux")
        .args(["display-message", "-p", "#{pane_id}"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());

    match active_pane {
        Some(p) => p == hook_pane,
        None => false,
    }
}
