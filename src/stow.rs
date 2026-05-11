use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Result;

use crate::config::Config;
use crate::ignore::should_ignore;

pub fn stow_package(
    source: &Path,
    target: &Path,
    config: &Config,
    ignores: &[regex::Regex],
) -> Result<()> {
    if !source.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "source package must be a directory",
        ));
    }

    visit_source(source, source, target, config, ignores)
}

pub fn unstow_package(
    source: &Path,
    target: &Path,
    config: &Config,
    ignores: &[regex::Regex],
) -> Result<()> {
    if !source.is_dir() {
        anyhow::bail!("source package must be a directory");
    }

    visit_unstow(source, source, target, config, ignores)
}

fn visit_source(
    root: &Path,
    current: &Path,
    target: &Path,
    config: &Config,
    ignores: &[regex::Regex],
) -> Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let rel = path.strip_prefix(root).unwrap();

        if should_ignore(rel, ignores) {
            continue;
        }

        let destination = target.join(rel);

        if entry.file_type()?.is_dir() {
            fs::create_dir_all(&destination)?;
            visit_source(root, &path, target, config, ignores)?;
        } else {
            stow_item(&path, &destination, config)?;
        }
    }
    Ok(())
}

fn stow_item(source: &Path, destination: &Path, config: &Config) -> Result<()> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }

    if destination.exists() || destination.symlink_metadata().map(|m| m.file_type().is_symlink()).unwrap_or(false) {
        if config.adopt {
            // Adopt mode: replace without backup
            remove_existing(destination)?;
        } else if config.force {
            // Force mode: replace with optional backup
            if config.backup {
                backup_existing(destination)?;
            }
            remove_existing(destination)?;
        } else {
            eprintln!("Skipping existing destination: {:?}", destination);
            return Ok(());
        }
    }

    if config.dry_run {
        println!("DRY RUN: link {:?} -> {:?}", destination, source);
        return Ok(());
    }

    create_symlink(source, destination)
}

fn visit_unstow(
    root: &Path,
    current: &Path,
    target: &Path,
    config: &Config,
    ignores: &[regex::Regex],
) -> Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let rel = path.strip_prefix(root).unwrap();

        if should_ignore(rel, ignores) {
            continue;
        }

        let destination = target.join(rel);

        if entry.file_type()?.is_dir() {
            visit_unstow(root, &path, target, config, ignores)?;
        }

        if destination.exists() {
            if destination.symlink_metadata()?.file_type().is_symlink() {
                if should_remove_link(&destination, &path) {
                    if config.backup {
                        backup_existing(&destination)?;
                    }
                    if config.dry_run {
                        println!("DRY RUN: remove {:?}", destination);
                    } else {
                        fs::remove_file(&destination)?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn should_remove_link(destination: &Path, source: &Path) -> bool {
    if let Ok(link_target) = fs::read_link(destination) {
        let abs_link = if link_target.is_absolute() {
            link_target
        } else {
            destination.parent().unwrap_or_else(|| Path::new(".")).join(link_target)
        };
        if let Ok(link_canon) = abs_link.canonicalize() {
            if let Ok(src_canon) = source.canonicalize() {
                return link_canon == src_canon;
            }
        }
    }
    false
}

fn backup_existing(path: &Path) -> Result<()> {
    let mut backup = path.with_extension("bak");
    let mut counter = 1;
    while backup.exists() {
        backup = path.with_extension(format!("bak{}", counter));
        counter += 1;
    }
    fs::rename(path, backup)
}

fn remove_existing(path: &Path) -> Result<()> {
    let metadata = path.symlink_metadata()?;
    if metadata.file_type().is_dir() && !metadata.file_type().is_symlink() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

#[cfg(unix)]
fn create_symlink(source: &Path, destination: &Path) -> Result<()> {
    std::os::unix::fs::symlink(source, destination)
        .map_err(|e| anyhow::anyhow!("failed to create symlink: {}", e))
}

#[cfg(windows)]
fn create_symlink(source: &Path, destination: &Path) -> Result<()> {
    if source.is_dir() {
        std::os::windows::fs::symlink_dir(source, destination)
            .map_err(|e| anyhow::anyhow!("failed to create symlink_dir: {}", e))
    } else {
        std::os::windows::fs::symlink_file(source, destination)
            .map_err(|e| anyhow::anyhow!("failed to create symlink_file: {}", e))
    }
}
