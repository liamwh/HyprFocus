use anyhow::{Context, Result};

use hyprland::{
    data::{Client, Clients},
    dispatch::{Dispatch, DispatchType, WindowIdentifier},
    shared::{Address, HyprData, HyprDataActiveOptional},
};
use tracing::instrument;

use crate::io::launch_application;

#[instrument(ret, err)]
pub fn focus_window<T>(window_address_to_focus: &T, active_address: &T) -> Result<()>
where
    T: ToString + std::fmt::Debug + PartialEq + std::fmt::Display,
{
    if window_address_to_focus != active_address {
        focus_window_by_address(Address::new(window_address_to_focus))?;
    }
    Ok(())
}

#[instrument(level = "trace", ret, err)]
pub fn focus_window_by_address<T>(window_address: T) -> Result<()>
where
    T: ToString + std::fmt::Debug,
{
    let window_address = Address::new(window_address);
    Ok(Dispatch::call(DispatchType::FocusWindow(
        WindowIdentifier::Address(window_address),
    ))?)
}

#[instrument(level = "trace", ret, err)]
pub fn focus_most_recent_window(
    open_windows_for_this_client: Vec<&Client>,
    active_address: Address,
) -> Result<()> {
    if let Some(most_recent_window_different_to_active_window) = open_windows_for_this_client
        .iter()
        .filter(|client| client.address != active_address)
        .min_by_key(|client| client.focus_history_id)
    {
        focus_window_by_address(
            most_recent_window_different_to_active_window
                .address
                .clone(),
        )?;
    }
    Ok(())
}

#[instrument(ret, err)]
pub fn focus_or_launch_app(client_name: &str, launcher_command: &str) -> Result<()> {
    let clients: Clients = Clients::get().context("expected to be able to get clients")?;
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
pub fn focus_application(open_windows_for_this_client: Vec<&Client>) -> anyhow::Result<()> {
    let active_client = Client::get_active()
        .context("expected to be able to get the active client")?
        .expect("Expected active client to exist");

    match open_windows_for_this_client.len() {
        1 => focus_window(
            &open_windows_for_this_client
                .first()
                .expect("expected to have at least one window")
                .address,
            &active_client.address,
        )?,
        _ => focus_most_recent_window(open_windows_for_this_client, active_client.address)?,
    }

    Ok(())
}
