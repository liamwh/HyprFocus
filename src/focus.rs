use anyhow::{Context, Result};

use serde_json::Value;
use tracing::instrument;

#[instrument(ret, err)]
pub fn focus_window(window_to_focus: &Value, active_address: &str) -> Result<()> {
    let window_address = window_to_focus["address"]
        .as_str()
        .context("Window address not found")?;
    if window_address != active_address {
        focus_window_by_address(window_address)?;
    }
    Ok(())
}

#[instrument(level = "trace", ret, err)]
pub fn focus_window_by_address(window_address: &str) -> anyhow::Result<()> {
    crate::execute_hyprctl(&["dispatch", &format!("focuswindow address:{window_address}")])
}

#[instrument(level = "trace", ret, err)]
pub fn focus_most_recent_window(windows: &[Value], active_address: &str) -> Result<()> {
    tracing::info!(?windows);
    if let Some(most_recent_window_different_to_active_window) = windows
        .iter()
        .filter(|window| window["address"].as_str() != Some(active_address))
        .min_by_key(|window| {
            window["focusHistoryID"]
                .as_i64()
                .expect("expected focusHistoryID to be an i64")
        })
    {
        let window_address = most_recent_window_different_to_active_window["address"]
            .as_str()
            .expect("expected to be able to convert the most recent window address to a string");
        focus_window_by_address(window_address)?;
    }
    Ok(())
}
