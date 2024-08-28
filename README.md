<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD041 -->

# HyprFocus

<p align="center">
    <p align="center">
        <a href="https://github.com/liamwh/HyprFocus/actions/workflows/check.yml">
            <img src="https://img.shields.io/github/actions/workflow/status/liamwh/hyprfocus/check.yml?style=flat-square" alt="GitHub Workflow Status">
        </a>
        <a href="https://blog.rust-lang.org/2024/05/02/Rust-1.78.0.html">
            <img src="https://img.shields.io/badge/rustc-1.78+-success.svg?style=flat-square" alt="Rustc version"></a>
        <a href="https://github.com/rust-secure-code/safety-dance/">
            <img src="https://img.shields.io/badge/unsafe-forbidden-success.svg?style=flat-square" alt="Unsafe forbidden"/>
        </a>
        <a href="https://crates.io/crates/hyprfocus">
            <img src="https://img.shields.io/badge/docs-latest-success.svg?style=flat-square" alt="crates.io documentation"/>
        </a>
        <a href="https://github.com/liamwh/hyprfocus/blob/main/README.md">
            <img src="https://img.shields.io/badge/License-MIT-success.svg?style=flat-square" alt="MIT License">
        </a>
        <a href="https://github.com/liamwh/hyprfocus/blob/main/README.md">
            <img src="https://img.shields.io/badge/License-Apache-success.svg?style=flat-square" alt="Apache License">
        </a>
    </p>
    <p align="center">
    <a href="https://github.com/liamwh/hyprfocus"><img src="https://res.cloudinary.com/dbhkh9bkw/image/upload/v1724846579/hyprfocus-logo.webp" alt="HyprFocus"
            width=50%></a>

HyprFocus is a CLI utility designed for [Hyprland](https://hyprland.org/) that enhances your workflow by managing application focus. It either launches an application or brings its most recently used window to the forefront. If the targeted application is already in focus, HyprFocus cleverly shifts attention to the next most recently used window for that application. This functionality ensures seamless access to your application windows with a single command, helping you maintain peak productivity without breaking your flow.

## Usage

To use the CLI directly, run the following:

```sh
hyprfocus --client "Notion" --launcher "notion-app"
```

To [bind a key](https://wiki.hyprland.org/Configuring/Binds/) in Hyprland to execute HyprFocus, modify your Hyprland configuration file similar to the following example:

```conf
bind = CONTROL, N, exec, hyprfocus --client "Notion" --launcher "notion-app"
```

This would bind the `CTRL+N` key combination to open Notion or focus the most recently focused window of Notion.

## To do

- [ ] Add support for identifying the launcher command of an application by the application name.

## Installation

Run the following command to install HyprFocus:

```sh
cargo install hyprfocus
```
