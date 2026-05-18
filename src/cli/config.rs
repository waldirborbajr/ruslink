// src/cli/config.rs

#![allow(clippy::struct_excessive_bools)]

use std::path::PathBuf;

use crate::stow::MergeConfig;

#[derive(Debug, Clone)]
pub struct Config {
    pub package: String,
    pub stow_dir: PathBuf,
    pub target_dir: PathBuf,

    // ─── Operation modes ────────────────────────────────────────
    pub delete: bool,
    pub restow: bool,
    pub dry_run: bool,

    // ─── Output ────────────────────────────────────────────────
    pub verbose: bool,

    // ─── Git ───────────────────────────────────────────────────
    pub auto_git: bool,
    pub git_push: bool,
    pub commit_message: Option<String>,

    // ─── Conflict handling ─────────────────────────────────────
    pub force: bool,
    pub backup: bool,
    pub adopt: bool,

    // ─── Confirmation ──────────────────────────────────────────
    pub yes: bool,

    // ─── Merge mode ────────────────────────────────────────────
    /// Enable merge mode
    pub merge: bool,

    /// Merge settings
    pub merge_settings: MergeConfig,

    /// Show merge history
    pub show_merge_history: bool,

    // ─── Dotfiles mode ────────────────────────────────────────
    /// Enable dotfiles mode (dot- prefix → .)
    pub dotfiles: bool,
}

impl Config {
    /// Returns true if any destructive operation is enabled.
    #[must_use]
    pub const fn is_destructive(&self) -> bool {
        self.delete || self.restow || self.force || self.adopt
    }

    /// Returns true if merge mode is enabled.
    #[must_use]
    pub const fn is_merge_enabled(&self) -> bool {
        self.merge && self.merge_settings.enabled
    }
}