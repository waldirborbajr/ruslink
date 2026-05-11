use std::io;
use std::path::Path;
use std::process::Command;

use crate::config::Config;

pub fn auto_git_commit(package_path: &Path, config: &Config) -> io::Result<()> {
    // Check if it's a git repository
    if !package_path.join(".git").exists() {
        println!("  Not a git repository. Skipping auto commit.");
        return Ok(());
    }

    let status = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(package_path)
        .output()?;

    if status.stdout.is_empty() {
        println!("  No changes to commit.");
        return Ok(());
    }

    // Add all changes
    let add_status = Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(package_path)
        .status()?;

    if !add_status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "git add failed"));
    }

    // Commit
    let message = config.commit_message.clone().unwrap_or_else(|| {
        format!("Update {} configuration - {}", 
            config.package, 
            chrono::Local::now().format("%Y-%m-%d %H:%M")
        )
    });

    let commit_status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .current_dir(package_path)
        .status()?;

    if commit_status.success() {
        println!("  ✓ Changes committed successfully!");
    } else {
        println!("  ⚠ Commit failed or was empty.");
    }

    Ok(())
}
