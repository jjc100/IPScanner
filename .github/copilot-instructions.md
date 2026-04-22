# Copilot instructions

This repository contains a Rust desktop application for Windows built with `eframe`/`egui`.

## Current repository state

- `Cargo.toml` and `Cargo.lock` define a Rust application crate.
- The main application code lives in `src\main.rs`.
- `README.md` documents the project and how to run it.
- `.github\workflows\ci.yml` runs `cargo test --locked` on Windows.

## Guidance for future Copilot sessions

- Re-scan the repository before making assumptions.
- Prefer Windows-oriented validation because the application uses Windows-specific APIs and commands.
- Use Cargo commands from the checked-in manifest instead of guessing:
  - `cargo test --locked`
  - `cargo run`
- Keep documentation aligned with the current app behavior and supported platform.
