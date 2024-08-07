use std::{
    fmt::{self, Display, Formatter},
    fs,
};

use anyhow::{anyhow, Context, Result};
use hyprland::{
    data::{Client, Clients},
    shared::HyprData,
};
use serde::Serialize;

fn get_active_clients_for_class(client_class: &str) -> Result<Vec<Client>> {
    let clients: Clients = Clients::get().context("expected to be able to get clients")?;
    let active_clients_for_client_class = clients
        .into_iter()
        .filter(|client| client.class == client_class)
        .collect::<Vec<Client>>();
    Ok(active_clients_for_client_class)
}
#[tracing::instrument(level = "debug", ret, err)]
pub fn get_launcher_cmd(client_class: &str) -> Result<Vec<ClientWithLauncherCommand>> {
    let active_clients_for_class = get_active_clients_for_class(client_class)?;

    if active_clients_for_class.is_empty() {
        return Err(anyhow!(
            "No active clients found for class name '{client_class}'"
        ));
    };

    let mut clients_with_launcher_commands = Vec::new();
    for active_client in &active_clients_for_class {
        let client_with_launcher_command = get_launcher_command_for_client(active_client)?;
        clients_with_launcher_commands.push(client_with_launcher_command);
    }

    Ok(clients_with_launcher_commands)
}

#[derive(Debug, Serialize)]
pub struct ClientWithLauncherCommand {
    client: Client,
    launcher_cmd: String,
}

impl Display for ClientWithLauncherCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Class: {}, PID: {}, Launcher Command: {}",
            self.client.class, self.client.pid, self.launcher_cmd
        )
    }
}

impl ClientWithLauncherCommand {
    pub const fn new(client: Client, launcher_cmd: String) -> Self {
        Self {
            client,
            launcher_cmd,
        }
    }
}

#[tracing::instrument(level = "debug", ret, err)]
fn get_launcher_command_for_client(active_client: &Client) -> Result<ClientWithLauncherCommand> {
    let pid = active_client.pid;
    let class_name = &active_client.class;
    let cmdline_path = format!("/proc/{pid}/cmdline");
    let cmdline = fs::read_to_string(&cmdline_path).with_context(|| {
        format!(
            "Failed to read command line for client with class name '{class_name}' & PID '{pid}'"
        )
    })?;
    let cmdline_formatted = cmdline.replace('\0', " ");

    tracing::debug!("Class: {class_name}, PID: {pid}, Command Line: {cmdline_formatted}");
    Ok(ClientWithLauncherCommand::new(
        active_client.clone(),
        cmdline_formatted,
    ))
}
