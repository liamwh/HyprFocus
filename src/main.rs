use anyhow::Result;

mod app_cmds;
mod focus;
mod get;
mod io;
mod observability;

use focus::focus_or_launch_app;

#[tokio::main]
async fn main() -> Result<()> {
    let _non_blocking_writer_guard = observability::setup_tracing()?;
    let matches = app_cmds::app_cmd().get_matches();
    if let Some(get_matches) = matches.subcommand_matches("get") {
        if let Some(class_matches) = get_matches.subcommand_matches("class") {
            if let Some(class_name) = class_matches.get_one::<String>("class") {
                let using_json_output = class_matches.get_flag("json");
                let clients_with_launcher_commands = get::get_launcher_cmd(class_name)?;
                if using_json_output {
                    let json = serde_json::to_string_pretty(&clients_with_launcher_commands)?;
                    println!("{json}");
                    return Ok(());
                }
                for client_with_launcher_command in clients_with_launcher_commands {
                    println!("{client_with_launcher_command}");
                }
            }
        }
    } else {
        let class_name = matches
            .get_one::<String>("class")
            .expect("expected a valid 'class' arg");

        let launcher_command = matches
            .get_one::<String>("launcher")
            .expect("expected a valid 'launcher' arg");

        return focus_or_launch_app(class_name, launcher_command);
    }
    Ok(())
}
