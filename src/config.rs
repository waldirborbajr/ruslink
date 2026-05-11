// src/config.rs
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub package: String,
    pub stow_dir: PathBuf,
    pub target_dir: PathBuf,

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
}

impl Config {
    /// Retorna se alguma operação destrutiva será realizada
    pub fn is_destructive(&self) -> bool {
        self.delete || self.restow || self.force || self.adopt
    }
}
