use anyhow::Result;

mod app_cmds;
mod focus;
mod io;
mod observability;

use focus::focus_or_launch_app;

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

    focus_or_launch_app(client_name, launcher_command)
}
