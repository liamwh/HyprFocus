use clap::{crate_version, Arg, ArgAction, Command};

pub const CRATE_VERSION: &str = crate_version!();

pub fn app_cmd() -> Command {
    Command::new("hyprfocus")
        .version(CRATE_VERSION)
        .author("Liam Woodleigh-Hardinge")
        .about("Focuses windows and launches applications in Hyprland")
        .subcommand(get_cmd())
        .arg(
            Arg::new("class")
                .short('c')
                .long("class")
                .required(false)
                .help("The client application class name"),
        )
        .arg(
            Arg::new("launcher")
                .short('l')
                .long("launcher")
                .help("The command to launch the client (application) if it is not already running")
                .required(false),
        )
}

pub fn get_cmd() -> Command {
    Command::new("get")
        .about("Gets information about something")
        .subcommand(class_cmd())
}

fn class_cmd() -> Command {
    Command::new("class")
        .about("Gets information about the clients that have this class")
        .arg(
            Arg::new("class")
                .index(1)
                .help("The client application class name")
                .required(false),
        )
        .arg(
            Arg::new("json")
                .long("json")
                .short('j')
                .action(ArgAction::SetTrue)
                .help("Output in JSON format"),
        )
}
