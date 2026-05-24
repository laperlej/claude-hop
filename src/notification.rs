use anyhow::Result;
use std::process::Command;

#[cfg(target_os = "macos")]
pub fn notify(title: &str, body: &str) -> Result<()> {
    let script = format!(
        "display notification \"{}\" with title \"{}\"",
        body.replace('\\', "\\\\").replace('"', "\\\""),
        title.replace('\\', "\\\\").replace('"', "\\\""),
    );
    Command::new("osascript").args(["-e", &script]).output()?;
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn notify(title: &str, body: &str) -> Result<()> {
    Command::new("notify-send").args([title, body]).output()?;
    Ok(())
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub fn notify(_title: &str, _body: &str) -> Result<()> {
    Ok(())
}
