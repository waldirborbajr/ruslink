use std::path::Path;
use std::process::Command;
use anyhow::Result;
use tracing::{debug, info};

use crate::config::Config;

pub fn auto_git_commit(package_path: &Path, config: &Config) -> Result<()> {
    // Check if it's a git repository
    if !package_path.join(".git").exists() {
        debug!("Not a git repository. Skipping auto commit.");
        return Ok(());
    }

    let status = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(package_path)
        .output()
        .map_err(|e| anyhow::anyhow!("failed to get git status: {}", e))?;

    if status.stdout.is_empty() {
        debug!("No changes to commit.");
        return Ok(());
    }

    debug!("Git changes detected, adding files...");

    // Add all changes
    let add_status = Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(package_path)
        .status()
        .map_err(|e| anyhow::anyhow!("failed to run git add: {}", e))?;

    if !add_status.success() {
        anyhow::bail!("git add failed");
    }

    debug!("Git add succeeded, committing changes...");

    // Commit
    let message = config.commit_message.clone().unwrap_or_else(|| {
        format!("Update {} configuration - {}", 
            config.package, 
            chrono::Local::now().format("%Y-%m-%d %H:%M")
        )
    });

    debug!("Commit message: {}", message);

    let commit_status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&message)
        .current_dir(package_path)
        .status()
        .map_err(|e| anyhow::anyhow!("failed to run git commit: {}", e))?;

    if commit_status.success() {
        info!("✓ Changes committed successfully!");
    } else {
        debug!("Commit failed or was empty.");
    }

    Ok(())
}

pub fn auto_git_push(package_path: &Path, _config: &Config) -> Result<()> {
    // Check if it's a git repository
    if !package_path.join(".git").exists() {
        debug!("Not a git repository. Skipping push.");
        return Ok(());
    }

    debug!("Pushing changes to remote...");

    // Push to remote
    let push_status = Command::new("git")
        .arg("push")
        .current_dir(package_path)
        .status()
        .map_err(|e| anyhow::anyhow!("failed to run git push: {}", e))?;

    if push_status.success() {
        info!("✓ Changes pushed successfully!");
    } else {
        anyhow::bail!("git push failed");
    }

    Ok(())
}
