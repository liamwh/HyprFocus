use anyhow::Result;

use hyprland::{
    data::Clients,
    dispatch::{Dispatch, DispatchType, WindowIdentifier},
    shared::{Address, HyprData},
};
use tracing::instrument;

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
pub fn focus_most_recent_window(active_address: Address) -> Result<()> {
    let clients = Clients::get()?;
    if let Some(most_recent_window_different_to_active_window) = clients
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
