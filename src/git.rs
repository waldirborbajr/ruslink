// src/git.rs
use std::path::{Path, PathBuf};
use std::process::Command;
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

    pub fn is_git_repo(&self) -> bool {
        self.path.join(".git").exists()
    }

    pub fn has_changes(&self) -> Result<bool> {
        if !self.is_git_repo() {
            return Ok(false);
        }

        let output = self.git_command()
            .arg("status")
            .arg("--porcelain")
            .output()?;

        Ok(!output.stdout.is_empty())
    }

    /// Commit silencioso automático
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

    /// Commit com mensagem configurável (usado com --git)
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

    /// Push para o remote
    pub fn push(&self) -> Result<()> {
        if !self.is_git_repo() {
            debug!("Not a git repository. Skipping push.");
            return Ok(());
        }

        info!("Pushing changes to remote...");

        let status = self.git_command().arg("push").status()?;

        if status.success() {
            info!("✓ Successfully pushed to remote!");
        } else {
            anyhow::bail!("git push failed");
        }

        Ok(())
    }

    // ====================== Helpers ======================

    /// Retorna um Command já configurado com o diretório correto
    fn git_command(&self) -> Command {
        let mut cmd = Command::new("git");
        cmd.current_dir(&self.path);
        cmd
    }

    fn git_add(&self) -> Result<()> {
        let status = self.git_command().arg("add").arg(".").status()?;

        if !status.success() {
            anyhow::bail!("git add failed");
        }
        Ok(())
    }

    fn has_staged_changes(&self) -> Result<bool> {
        let status = self.git_command()
            .arg("diff")
            .arg("--cached")
            .arg("--quiet")
            .status()?;

        Ok(status.success()) // success significa "não há diferenças"
    }

    fn git_commit(&self, message: &str) -> Result<()> {
        let status = self.git_command()
            .arg("commit")
            .arg("-m")
            .arg(message)
            .status()?;

        if !status.success() {
            debug!("Commit returned non-zero (possibly no changes).");
        }
        Ok(())
    }
}
