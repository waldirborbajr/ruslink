use anyhow::Result;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tracing::{debug, info};

use crate::cli::Config;

/// Sanitiza mensagem de commit
fn sanitize_commit_message(message: &str) -> String {
    let mut sanitized = message.trim().to_string();

    sanitized = sanitized
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    if let Some(first_line) = sanitized.lines().next() {
        if first_line.len() > 100 {
            sanitized = first_line.chars().take(97).collect::<String>() + "...";
        }
    }

    if sanitized.trim().is_empty() {
        sanitized = "Update configuration".to_string();
    }

    sanitized
}

#[cfg(feature = "git")]
fn get_timestamp() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M").to_string()
}

#[cfg(not(feature = "git"))]
fn get_timestamp() -> String {
    "now".to_string()
}

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

    pub fn ensure_git_installed() -> Result<()> {
        debug!("Checking if git is installed...");

        let output = Command::new("git").arg("--version").output()?;

        if !output.status.success() {
            anyhow::bail!("Git is not installed or not found in PATH.\nPlease install Git first.");
        }

        debug!(
            "Git found: {}",
            String::from_utf8_lossy(&output.stdout).trim()
        );
        Ok(())
    }

    pub fn is_git_repo(&self) -> bool {
        self.run_git_quiet(&["rev-parse", "--git-dir"]).is_ok()
    }

    pub fn has_changes(&self) -> Result<bool> {
        if !self.is_git_repo() {
            return Ok(false);
        }
        let output = self.run_git(&["status", "--porcelain"])?;
        Ok(!output.stdout.is_empty())
    }

    // ====================== Operações Públicas ======================

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
            get_timestamp()
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

        let raw_message = config.commit_message.clone().unwrap_or_else(|| {
            format!(
                "Update {} configuration - {}",
                config.package,
                get_timestamp()
            )
        });

        self.git_commit(&raw_message)?;
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

    // ====================== Comandos Internos ======================

    fn git_add(&self) -> Result<()> {
        self.run_git(&["add", "-A"])?;
        Ok(())
    }

    fn has_staged_changes(&self) -> Result<bool> {
        let output = self.run_git(&["diff", "--cached", "--quiet"])?;
        Ok(output.status.success())
    }

    fn git_commit(&self, message: &str) -> Result<()> {
        let clean_message = sanitize_commit_message(message);
        debug!("Commit message: {}", clean_message.replace('\n', "\\n"));

        self.run_git(&["commit", "-m", &clean_message])?;
        Ok(())
    }

    // ====================== Executores ======================

    fn run_git(&self, args: &[&str]) -> Result<Output> {
        debug!("git {}", args.join(" "));

        let output = Command::new("git")
            .current_dir(&self.path)
            .args(args)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

            let mut error_msg = format!("git {} failed", args.join(" "));
            if !stderr.is_empty() {
                error_msg.push_str(&format!("\nstderr: {}", stderr));
            }
            if !stdout.is_empty() {
                error_msg.push_str(&format!("\nstdout: {}", stdout));
            }
            anyhow::bail!(error_msg);
        }

        debug!("git {} succeeded", args.join(" "));
        Ok(output)
    }

    fn run_git_quiet(&self, args: &[&str]) -> Result<Output> {
        let output = Command::new("git")
            .current_dir(&self.path)
            .args(args)
            .output()?;

        if !output.status.success() {
            debug!("git {} returned non-zero (expected)", args.join(" "));
            anyhow::bail!("command failed");
        }
        Ok(output)
    }
}
