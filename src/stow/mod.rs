// src/stow/mod.rs

pub mod merge;
mod stowmanager;

pub use merge::MergeConfig;
pub use stowmanager::{stow_package, unstow_package, StowStats};
