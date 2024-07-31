use anyhow::{Context, Result};

use serde_json::Value;
use std::process::Command;
use tracing::instrument;

mod app_cmds;
mod focus;
mod io;
mod observability;

use focus::{focus_most_recent_window, focus_window};
use io::{execute_hyprctl, launch_application};

#[tokio::main]
async fn main() -> Result<()> {
    let _non_blocking_writer_guard = observability::setup_tracing()?;
    let arg_matches = app_cmds::app_cmd().get_matches();
    let client_name = arg_matches
        .get_one::<String>("client")
        .expect("expected to valid 'client name' arg");

    let launcher_command = arg_matches
        .get_one::<String>("launcher")
        .expect("expected to valid 'launcher' arg");

    focus_or_launch_app(client_name, launcher_command)?;
    Ok(())
}

#[instrument(ret, err)]
fn get_hyprland_clients_json() -> Result<Value> {
    let clients_json = Command::new("hyprctl")
        .args(["clients", "-j"])
        .output()
        .context("Failed to get client list from hyprctl")?
        .stdout;

    serde_json::from_slice(&clients_json).context("Failed to parse clients JSON")
}

#[instrument(ret, err)]
fn focus_or_launch_app(client_name: &str, launcher_command: &str) -> Result<()> {
    let clients: Value = get_hyprland_clients_json()?;
    let open_windows_for_this_client: Vec<Value> = filter_clients_by_name(&clients, client_name);

    if open_windows_for_this_client.is_empty() {
        tracing::debug!("No windows open for client '{client_name}'");
        if launcher_command.is_empty() {
            tracing::warn!("No launcher command provided");
            return Ok(());
        }
        return launch_application(launcher_command);
    }

    focus_application(&open_windows_for_this_client)
}

#[instrument(ret, err)]
fn focus_application(open_windows_for_this_client: &[Value]) -> anyhow::Result<()> {
    let active_window = get_active_window_json()?;
    let active_address = active_window["address"]
        .as_str()
        .expect("expected to be able to determine the active window address");

    match open_windows_for_this_client.len() {
        1 => focus_window(&open_windows_for_this_client[0], active_address)?,
        _ => focus_most_recent_window(open_windows_for_this_client, active_address)?,
    }

    Ok(())
}

#[instrument(level = "debug", ret, err)]
fn get_active_window_json() -> Result<Value, anyhow::Error> {
    let active_window_address = Command::new("hyprctl")
        .args(["activewindow", "-j"])
        .output()
        .context("Failed to get active window from hyprctl")?
        .stdout;
    let active_window: Value = serde_json::from_slice(&active_window_address)
        .context("Failed to parse active window JSON")?;
    Ok(active_window)
}

#[instrument(level = "trace", ret)]
fn filter_clients_by_name(clients: &Value, client_name: &str) -> Vec<Value> {
    let open_windows_for_this_client: Vec<Value> = clients
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter(|client| client["class"] == client_name)
        .cloned()
        .collect();
    open_windows_for_this_client
}
