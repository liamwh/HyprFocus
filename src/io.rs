use anyhow::{Context, Result};

use std::process::Command;
use tracing::instrument;

#[instrument(level = "trace", ret, err)]
pub fn execute_hyprctl(args: &[&str]) -> Result<()> {
    Command::new("hyprctl")
        .args(args)
        .output()
        .context("Failed to execute hyprctl")?;
    Ok(())
}

#[instrument(level = "trace", ret, err)]
pub fn launch_application(launcher_command: &str) -> Result<(), anyhow::Error> {
    Command::new("sh")
        .arg("-c")
        .arg(launcher_command)
        .spawn()
        .context("Failed to launch the application")?;
    Ok(())
}
