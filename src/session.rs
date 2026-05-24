use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Instance {
    pub pid: u32,
    pub session_id: String,
    pub cwd: String,
    pub status: Status,
    pub name: Option<String>,
    pub started_at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Busy,
    Idle,
    Asking,
    Waiting,
    Unknown,
}

impl Status {
    pub fn label(&self) -> &'static str {
        match self {
            Status::Busy => "[busy]",
            Status::Idle => "[idle]",
            Status::Asking => "[ask]",
            Status::Waiting => "[wait]",
            Status::Unknown => "[??]",
        }
    }
}

#[derive(Deserialize)]
struct SessionFile {
    pid: u32,
    #[serde(rename = "sessionId")]
    session_id: String,
    cwd: String,
    status: Option<String>,
    name: Option<String>,
    #[serde(rename = "startedAt")]
    started_at: Option<u64>,
}

fn sessions_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".claude").join("sessions"))
}

pub fn discover() -> Result<Vec<Instance>> {
    let dir = sessions_dir().context("could not determine home directory")?;
    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut instances = Vec::new();

    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }

        let contents = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let sf: SessionFile = match serde_json::from_str(&contents) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let status = match sf.status.as_deref() {
            Some("busy") => Status::Busy,
            Some("idle") => Status::Idle,
            Some("asking") => Status::Asking,
            Some("waiting") => Status::Waiting,
            _ => Status::Unknown,
        };

        instances.push(Instance {
            pid: sf.pid,
            session_id: sf.session_id,
            cwd: sf.cwd,
            status,
            name: sf.name,
            started_at: sf.started_at.unwrap_or(0),
        });
    }

    // idle instances first, then by start time descending
    instances.sort_by(|a, b| {
        let idle_ord = (a.status != Status::Idle).cmp(&(b.status != Status::Idle));
        idle_ord.then(b.started_at.cmp(&a.started_at))
    });

    Ok(instances)
}
