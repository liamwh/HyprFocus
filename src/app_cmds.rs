use clap::{crate_version, Arg, Command};

pub const CRATE_VERSION: &str = crate_version!();

pub fn app_cmd() -> Command {
    Command::new("hyprfocus")
        .version(CRATE_VERSION)
        .author("Liam Woodleigh-Hardinge")
        .about("Focuses windows and launches applications in Hyprland")
        .arg(
            Arg::new("client")
                .short('c')
                .long("client")
                .help("The client application class name, found using 'hyprctl clients'"),
        )
        .arg(
            Arg::new("launcher").short('l').long("launcher").help(
                "The command to launch the client (application) if it is not already running",
            ),
        )
}
