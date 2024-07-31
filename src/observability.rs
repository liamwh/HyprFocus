use anyhow::{Context, Result};
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{prelude::*, EnvFilter};

pub fn setup_tracing() -> Result<WorkerGuard> {
    let file_appender = rolling::daily("/tmp", "HyprFocus.jsonl");
    let (non_blocking_log_writer, guard) = tracing_appender::non_blocking(file_appender);

    let filter = EnvFilter::from_default_env();
    let formatting_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(non_blocking_log_writer);
    let subscriber = tracing_subscriber::Registry::default()
        .with(filter)
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber).context("Failed to set subscriber")?;
    Ok(guard)
}
