// src/stow.rs
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use pathdiff::diff_paths;
use tracing::{debug, info};

use crate::config::Config;
use crate::ignore::should_ignore;

/// Stow a package (create symlinks)
pub fn stow_package(
    source: &Path,
    target: &Path,
    config: &Config,
    ignores: &[regex::Regex],
) -> Result<()> {
    if !source.is_dir() {
        anyhow::bail!("Source package must be a directory: {:?}", source);
    }

    info!("Stowing from {:?} → {:?}", source, target);
    visit_source(source, source, target, config, ignores)
}

/// Unstow a package (remove symlinks)
pub fn unstow_package(
    source: &Path,
    target: &Path,
    config: &Config,
    ignores: &[regex::Regex],
) -> Result<()> {
    if !source.is_dir() {
        anyhow::bail!("Source package must be a directory: {:?}", source);
    }

    info!("Unstowing from {:?} → {:?}", source, target);
    visit_unstow(source, source, target, config, ignores)
}

// ====================== STOW ======================

fn visit_source(
    root: &Path,
    current: &Path,
    target_base: &Path,
    config: &Config,
    ignores: &[regex::Regex],
) -> Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let rel_path = path.strip_prefix(root).unwrap_or(&path);

        if should_ignore(rel_path, ignores) {
            debug!("Ignored: {:?}", rel_path);
            continue;
        }

        let destination = target_base.join(rel_path);

        if entry.file_type()?.is_dir() {
            if !config.dry_run {
                fs::create_dir_all(&destination)?;
            }
            visit_source(root, &path, target_base, config, ignores)?;
        } else {
            stow_item(&path, &destination, config)?;
        }
    }
    Ok(())
}

fn stow_item(source: &Path, destination: &Path, config: &Config) -> Result<()> {
    if let Some(parent) = destination.parent() {
        if !config.dry_run {
            fs::create_dir_all(parent)?;
        }
    }

    // Handle existing destination
    if destination.exists() || destination.symlink_metadata().is_ok() {
        handle_existing_destination(destination, config)?;
    }

    if config.dry_run {
        info!("DRY RUN: would link {:?} → {:?}", destination, source);
        return Ok(());
    }

    let relative = make_relative(source, destination);
    create_symlink(&relative, destination)?;

    info!("Linked: {:?} → {:?}", destination, relative);
    Ok(())
}

fn handle_existing_destination(destination: &Path, config: &Config) -> Result<()> {
    if destination.symlink_metadata().map(|m| m.file_type().is_symlink()).unwrap_or(false) {
        if config.dry_run {
            debug!("Would remove existing symlink: {:?}", destination);
        } else {
            fs::remove_file(destination)?;
        }
        return Ok(());
    }

    // Real file or directory exists
    if config.adopt {
        debug!("Adopting existing file: {:?}", destination);
        remove_existing(destination)?;
    } else if config.force {
        if config.backup {
            backup_existing(destination)?;
        }
        remove_existing(destination)?;
    } else {
        anyhow::bail!("Conflict: {:?} already exists (use --force or --adopt)", destination);
    }
    Ok(())
}

// ====================== UNSTOW ======================

fn visit_unstow(
    root: &Path,
    current: &Path,
    target_base: &Path,
    config: &Config,
    ignores: &[regex::Regex],
) -> Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let rel_path = path.strip_prefix(root).unwrap_or(&path);

        if should_ignore(rel_path, ignores) {
            continue;
        }

        let destination = target_base.join(rel_path);

        if entry.file_type()?.is_dir() {
            visit_unstow(root, &path, target_base, config, ignores)?;
            // Try to remove empty directory
            if !config.dry_run && destination.exists() {
                let _ = fs::remove_dir(&destination);
            }
        } else if is_managed_symlink(&destination, &path) {
            if config.backup {
                backup_existing(&destination)?;
            }
            if config.dry_run {
                info!("DRY RUN: would remove {:?}", destination);
            } else {
                fs::remove_file(&destination)?;
                info!("Removed: {:?}", destination);
            }
        }
    }
    Ok(())
}

fn is_managed_symlink(destination: &Path, source: &Path) -> bool {
    if let Ok(link) = fs::read_link(destination) {
        let abs_link = if link.is_absolute() {
            link
        } else {
            destination.parent().unwrap_or_else(|| Path::new(".")).join(link)
        };
        if let (Ok(a), Ok(b)) = (abs_link.canonicalize(), source.canonicalize()) {
            return a == b;
        }
    }
    false
}

// ====================== HELPERS ======================

fn make_relative(source: &Path, destination: &Path) -> PathBuf {
    diff_paths(source, destination.parent().unwrap_or(destination))
        .unwrap_or_else(|| source.to_path_buf())
}

fn backup_existing(path: &Path) -> Result<()> {
    let mut backup = path.with_extension("bak");
    let mut counter = 1;
    while backup.exists() {
        backup = path.with_extension(format!("bak{}", counter));
        counter += 1;
    }
    fs::rename(path, &backup)?;
    info!("Backed up: {:?} → {:?}", path, backup);
    Ok(())
}

fn remove_existing(path: &Path) -> Result<()> {
    let meta = path.symlink_metadata()?;
    if meta.is_dir() && !meta.file_type().is_symlink() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }
    Ok(())
}

#[cfg(unix)]
fn create_symlink(source: &Path, destination: &Path) -> Result<()> {
    std::os::unix::fs::symlink(source, destination)
        .map_err(|e| anyhow::anyhow!("Failed to create symlink {} -> {}: {}", destination.display(), source.display(), e))
}

#[cfg(windows)]
fn create_symlink(source: &Path, destination: &Path) -> Result<()> {
    if source.is_dir() {
        std::os::windows::fs::symlink_dir(source, destination)
    } else {
        std::os::windows::fs::symlink_file(source, destination)
    }
    .map_err(|e| anyhow::anyhow!("Failed to create symlink on Windows: {}", e))
}
