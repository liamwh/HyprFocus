# HyprFocus

HyprFocus is a CLI utility for opening an application or focusing the most recent client (window) of that open application. This is useful for opening applications that are not in the foreground, or for focusing the most recently opened window of an application.

## Usage

To use the CLI directly, run the following:

```sh
hyprfocus --client "Notion" --launcher "notion-app"
```

To bind a key to execute HyprFocus, modify your Hyprland configuration file similar to the following example:

```conf
bind = CONTROL, B, exec, hyprfocus --client "Brave-browser" --launcher "brave"
```

This would bind the `CTRL+B` key combination to open Brave Browser or focus the most recently focused window of Brave.

## Installation

Run the following command to install HyprFocus:

```sh
cargo install hyprfocus
```
