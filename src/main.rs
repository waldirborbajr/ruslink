// src/main.rs

#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]

mod app;
mod cli;
mod git;
mod stow;
mod utils;

use anyhow::Result;

fn main() -> Result<()> {
    app::run()
}
