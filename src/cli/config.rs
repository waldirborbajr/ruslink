// src/cli/config.rs
#![allow(clippy::struct_excessive_bools)]

use std::path::PathBuf;

use crate::stow::MergeConfig;

#[derive(Debug, Clone)]
pub struct Config {
    pub package: String,
    pub stow_dir: PathBuf,
    pub target_dir: PathBuf,

    // New commands
    pub list: bool,
    pub status: bool,
    pub clean: bool,

    // Operation modes
    pub delete: bool,
    pub restow: bool,
    pub dry_run: bool,

    // Output
    pub verbose: bool,

    // Git
    pub auto_git: bool,
    pub git_push: bool,
    pub commit_message: Option<String>,

    // Conflict handling
    pub force: bool,
    pub backup: bool,
    pub adopt: bool,

    // Confirmation
    pub yes: bool,

    // Merge mode
    pub merge: bool,
    pub merge_settings: MergeConfig,
    pub show_merge_history: bool,

    // Dotfiles mode
    pub dotfiles: bool,
}

impl Config {
    #[must_use]
    pub const fn is_destructive(&self) -> bool {
        self.delete || self.restow || self.force || self.adopt
    }

    #[must_use]
    pub const fn is_merge_enabled(&self) -> bool {
        self.merge && self.merge_settings.enabled
    }
}