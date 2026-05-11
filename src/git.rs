// src/git.rs
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use anyhow::Result;
use tracing::{debug, info, warn};

use crate::config::Config;

/// Representa um repositório Git gerenciado pelo ruslink
#[derive(Debug)]
pub struct GitRepository {
    path: PathBuf,
}

impl GitRepository {
    pub fn new<P: AsRef<Path>>(package_path: P) -> Self {
        Self {
            path: package_path.as_ref().to_path_buf(),
        }
    }

    /// Verifica se o Git está instalado no sistema
    pub fn ensure_git_installed() -> Result<()> {
        debug!("Checking if git is installed...");

        let output = Command::new("git")
            .arg("--version")
            .output();

        match output {
            Ok(o) if o.status.success() => {
                debug!("Git found: {}", String::from_utf8_lossy(&o.stdout).trim());
                Ok(())
            }
            _ => {
                anyhow::bail!("Git is not installed or not found in PATH. Please install Git first.")
            }
        }
    }

    pub fn is_git_repo(&self) -> bool {
        self.path.join(".git").exists()
    }

    pub fn has_changes(&self) -> Result<bool> {
        if !self.is_git_repo() {
            return Ok(false);
        }

        let output = self.run_git(&["status", "--porcelain"])?;
        Ok(!output.stdout.is_empty())
    }

    pub fn auto_commit_silent(&self, package_name: &str) -> Result<()> {
        if !self.is_git_repo() {
            debug!("Not a git repository. Skipping auto-commit.");
            return Ok(());
        }

        debug!("Auto-committing changes for package: {}", package_name);

        self.git_add()?;

        if self.has_staged_changes()? {
            debug!("No changes to commit after git add.");
            return Ok(());
        }

        let message = format!(
            "chore({}): auto-update configuration ({})",
            package_name,
            chrono::Local::now().format("%Y-%m-%d %H:%M")
        );

        self.git_commit(&message)?;
        Ok(())
    }

    pub fn commit(&self, config: &Config) -> Result<()> {
        if !self.is_git_repo() {
            debug!("Not a git repository.");
            return Ok(());
        }

        if !self.has_changes()? {
            info!("No changes to commit.");
            return Ok(());
        }

        self.git_add()?;

        let message = config.commit_message.clone().unwrap_or_else(|| {
            format!(
                "Update {} configuration - {}",
                config.package,
                chrono::Local::now().format("%Y-%m-%d %H:%M")
            )
        });

        self.git_commit(&message)?;
        info!("✓ Changes committed successfully!");
        Ok(())
    }

    pub fn push(&self) -> Result<()> {
        if !self.is_git_repo() {
            debug!("Not a git repository. Skipping push.");
            return Ok(());
        }

        info!("Pushing changes to remote...");

        self.run_git(&["push"])?;
        info!("✓ Successfully pushed to remote!");

        Ok(())
    }

    // ====================== Git Commands ======================

    fn git_add(&self) -> Result<()> {
        self.run_git(&["add", "."])?;
        Ok(())
    }

    fn has_staged_changes(&self) -> Result<bool> {
        let output = self.run_git(&["diff", "--cached", "--quiet"])?;
        Ok(output.status.success())
    }

    fn git_commit(&self, message: &str) -> Result<()> {
        self.run_git(&["commit", "-m", message])?;
        Ok(())
    }

    // ====================== Central Command Executor ======================

    fn run_git(&self, args: &[&str]) -> Result<Output> {
        debug!("git {}", args.join(" "));

        let output = Command::new("git")
            .current_dir(&self.path)
            .args(args)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if !stderr.trim().is_empty() {
                warn!("git {}: {}", args.join(" "), stderr.trim());
            }
        } else {
            debug!("git {} succeeded", args.join(" "));
        }

        Ok(output)
    }
}
