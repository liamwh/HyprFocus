use anyhow::Result;

use tracing::instrument;

mod app_cmds;
mod focus;
mod io;
mod observability;

use focus::{focus_most_recent_window, focus_window};
use hyprland::{
    data::{Client, Clients},
    shared::{HyprData, HyprDataActiveOptional},
};
use io::launch_application;

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
fn focus_or_launch_app(client_name: &str, launcher_command: &str) -> Result<()> {
    let clients: Clients = Clients::get()?;
    let active_windows_for_client_name = clients
        .iter()
        .filter(|client| client.class == client_name)
        .collect::<Vec<&Client>>();

    if active_windows_for_client_name.is_empty() {
        tracing::debug!("No windows open for client '{client_name}'");
        if launcher_command.is_empty() {
            tracing::warn!("No launcher command provided");
            return Ok(());
        }
        return launch_application(launcher_command);
    }

    focus_application(active_windows_for_client_name)
}

#[instrument(ret, err)]
fn focus_application(open_windows_for_this_client: Vec<&Client>) -> anyhow::Result<()> {
    let active_client =
        Client::get_active()?.expect("expected to be able to get the active client");

    match open_windows_for_this_client.len() {
        1 => focus_window(
            &open_windows_for_this_client[0].address,
            &active_client.address,
        )?,
        _ => focus_most_recent_window(active_client.address)?,
    }

    Ok(())
}
