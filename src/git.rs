// src/git.rs
use std::path::Path;
use std::process::Command;
use anyhow::Result;
use tracing::{debug, info, warn};

use crate::config::Config;

/// Check if there are uncommitted changes in the git repository
pub fn has_git_changes(package_path: &Path) -> Result<bool> {
    if !package_path.join(".git").exists() {
        return Ok(false);
    }

    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(package_path)
        .output()?;

    Ok(!output.stdout.is_empty())
}

/// Silent auto-commit (used when --git is not explicitly passed)
pub fn auto_git_commit_silent(package_path: &Path, package_name: &str) -> Result<()> {
    if !package_path.join(".git").exists() {
        debug!("Not a git repository. Skipping auto-commit.");
        return Ok(());
    }

    debug!("Auto-committing changes for package: {}", package_name);

    // Add changes
    let add_status = Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(package_path)
        .status()?;

    if !add_status.success() {
        anyhow::bail!("git add failed");
    }

    // Check if there are actually changes to commit
    let diff_status = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .arg("--quiet")
        .current_dir(package_path)
        .status()?;

    if diff_status.success() {
        debug!("No changes to commit after git add.");
        return Ok(());
    }

    let message = format!(
        "chore({}): auto-update configuration ({})",
        package_name,
        chrono::Local::now().format("%Y-%m-%d %H:%M")
    );

    let commit_status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&message)
        .current_dir(package_path)
        .status()?;

    if commit_status.success() {
        info!("✓ Auto-commit successful: {}", message);
    } else {
        debug!("Commit returned non-zero (possibly empty).");
    }

    Ok(())
}

/// Manual git commit (used with --git flag)
pub fn auto_git_commit(package_path: &Path, config: &Config) -> Result<()> {
    if !package_path.join(".git").exists() {
        debug!("Not a git repository.");
        return Ok(());
    }

    if !has_git_changes(package_path)? {
        info!("No changes to commit.");
        return Ok(());
    }

    // git add
    Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(package_path)
        .status()?;

    let message = config.commit_message.clone().unwrap_or_else(|| {
        format!("Update {} configuration - {}", 
            config.package, 
            chrono::Local::now().format("%Y-%m-%d %H:%M")
        )
    });

    let status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&message)
        .current_dir(package_path)
        .status()?;

    if status.success() {
        info!("✓ Changes committed successfully!");
    } else {
        warn!("Commit failed or no changes were staged.");
    }

    Ok(())
}

/// Push changes to remote
pub fn auto_git_push(package_path: &Path, _config: &Config) -> Result<()> {
    if !package_path.join(".git").exists() {
        debug!("Not a git repository. Skipping push.");
        return Ok(());
    }

    info!("Pushing changes to remote...");

    let status = Command::new("git")
        .arg("push")
        .current_dir(package_path)
        .status()?;

    if status.success() {
        info!("✓ Successfully pushed to remote!");
    } else {
        anyhow::bail!("git push failed");
    }

    Ok(())
}
