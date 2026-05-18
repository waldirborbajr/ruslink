// src/stow/mod.rs

pub mod merge;
mod stowmanager;
pub mod commands;

pub use merge::MergeConfig;
pub use stowmanager::{stow_package, unstow_package, StowStats};
pub use commands::{list_packages, show_status, clean_target};