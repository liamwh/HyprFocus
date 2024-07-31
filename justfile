#!/usr/bin/env -S just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.

set dotenv-load := true

# Show available commands
default:
    @just --list --justfile {{justfile()}}

test-notion:
 RUST_LOG="debug" cargo run -- --client "Notion" --launcher "notion-app"

test-brave:
 RUST_LOG="debug" cargo run -- --client "Brave-browser" --launcher "brave"

# run various auditing tools to assure we are legal and safe
audit:
    cargo deny check advisories bans licenses sources

# Show unused dependencies
udeps:
    cargo +nightly udeps

# Run rust tests with `cargo nextest`
nextest *FLAGS="--all":
    cargo nextest run {{ FLAGS }}
