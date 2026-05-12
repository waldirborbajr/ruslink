// src/main.rs
mod app;
mod cli;
mod git;
mod stow;
mod utils;

use anyhow::Result;

fn main() -> Result<()> {
    app::run()
}
