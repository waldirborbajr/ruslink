// src/git/operations.rs
use std::path::Path;
use anyhow::Result;
use tracing::{debug, info, warn};

use crate::cli::Config;
use crate::utils::{error, warning};
use super::git::GitRepository;

pub fn handle_git_operations(package_path: &Path, config: &Config) -> Result<()> {
    if let Err(e) = GitRepository::ensure_git_installed() {
        error(&format!("Git error: {}", e));
        std::process::exit(1);
    }

    let repo = GitRepository::new(package_path);

    if config.auto_git {
        info!("Git: Checking for changes...");
        
        if let Err(e) = repo.commit(config) {
            warning(&format!("Git commit warning: {}", e));
        }

        if config.git_push {
            info!("Git: Pushing changes...");
            if let Err(e) = repo.push() {
                warning(&format!("Git push failed: {}", e));
            }
        }
    } else if let Ok(true) = repo.has_changes() {
        info!("Changes detected. Creating automatic commit...");
        if let Err(e) = repo.auto_commit_silent(&config.package) {
            debug!("Auto-commit failed: {}", e);
        }
    }
    Ok(())
}