// src/stow/stowmanager.rs

use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use anyhow::Result;
use pathdiff::diff_paths;
use tracing::{debug, info};

use super::merge::{MergeAction, MergeHandler};
use crate::cli::Config;
use crate::utils::should_ignore;

// ====================== DOTFILES HANDLING ======================

/// Transforma paths com prefixo 'dot-' em paths com prefixo '.'
/// Exemplo: "dot-bashrc" → ".bashrc"
///          "dot-config/fish" → ".config/fish"
fn transform_dot_prefix(path: &Path) -> PathBuf {
    let mut components = Vec::new();

    for comp in path.components() {
        match comp {
            std::path::Component::Normal(os_str) => {
                let name = os_str.to_string_lossy();
                if let Some(stripped) = name.strip_prefix("dot-") {
                    components.push(format!(".{stripped}"));
                } else {
                    components.push(name.into_owned());
                }
            }
            std::path::Component::ParentDir => {
                components.push("..".to_string());
            }
            // Ignore current dir and handle the rest by re-emitting
            std::path::Component::CurDir => {}
            _ => {
                // RootDir, Prefix, etc.
                components.push(comp.as_os_str().to_string_lossy().into_owned());
            }
        }
    }

    components.iter().collect()
}

#[derive(Debug, Default)]
pub struct StowStats {
    pub files_linked: usize,
    pub files_removed: usize,
    pub dirs_created: usize,
    pub files_ignored: usize,
}

impl StowStats {
    pub fn print_summary(&self, operation: &str, elapsed: std::time::Duration) {
        info!(
            "✅ {} completed: {} files | {} dirs | {} ignored | {:.2?}",
            operation, self.files_linked, self.dirs_created, self.files_ignored, elapsed
        );
    }
}

// ====================== PUBLIC API ======================

pub fn stow_package(
    source: &Path,
    target: &Path,
    config: &Config,
    ignores: &[regex::Regex],
) -> Result<StowStats> {
    if !source.is_dir() {
        anyhow::bail!("Source package must be a directory: {}", source.display());
    }

    let start = Instant::now();

    info!("Stowing from {} → {}", source.display(), target.display());

    let merge_handler = if config.is_merge_enabled() {
        Some(MergeHandler::new(source, config.package.clone()))
    } else {
        None
    };

    let mut stats = StowStats::default();

    visit_source(
        source,
        source,
        target,
        config,
        ignores,
        merge_handler.as_ref(),
        &mut stats,
    )?;

    let elapsed = start.elapsed();

    stats.print_summary("Stow", elapsed);

    if config.show_merge_history {
        if let Some(handler) = &merge_handler {
            handler.show_merge_history();
        }
    }

    Ok(stats)
}

pub fn unstow_package(
    source: &Path,
    target: &Path,
    config: &Config,
    ignores: &[regex::Regex],
) -> Result<StowStats> {
    if !source.is_dir() {
        anyhow::bail!("Source package must be a directory: {}", source.display());
    }

    let start = Instant::now();

    info!("Unstowing from {} → {}", source.display(), target.display());

    let mut stats = StowStats::default();

    visit_unstow(source, source, target, config, ignores, &mut stats)?;

    let elapsed = start.elapsed();

    stats.print_summary("Unstow", elapsed);

    Ok(stats)
}

// ====================== STOW ======================

fn visit_source(
    root: &Path,
    current: &Path,
    target_base: &Path,
    config: &Config,
    ignores: &[regex::Regex],
    merge_handler: Option<&MergeHandler>,
    stats: &mut StowStats,
) -> Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();

        let rel_path = path.strip_prefix(root).unwrap_or(&path);

        if should_ignore(rel_path, ignores) {
            stats.files_ignored += 1;

            debug!("Ignored: {}", rel_path.display());

            continue;
        }

        // Aplicar transformação dot- se dotfiles mode está habilitado
        let destination_rel_path = if config.dotfiles {
            transform_dot_prefix(rel_path)
        } else {
            rel_path.to_path_buf()
        };

        let destination = target_base.join(&destination_rel_path);

        if entry.file_type()?.is_dir() {
            if !config.dry_run {
                fs::create_dir_all(&destination)?;
                stats.dirs_created += 1;
            }

            visit_source(
                root,
                &path,
                target_base,
                config,
                ignores,
                merge_handler,
                stats,
            )?;
        } else if stow_item(&path, &destination, config, merge_handler)? {
            stats.files_linked += 1;
        }
    }

    Ok(())
}

fn stow_item(
    source: &Path,
    destination: &Path,
    config: &Config,
    merge_handler: Option<&MergeHandler>,
) -> Result<bool> {
    if let Some(parent) = destination.parent() {
        if !config.dry_run {
            fs::create_dir_all(parent)?;
        }
    }

    if destination.exists() || destination.symlink_metadata().is_ok() {
        if let Some(merge) = merge_handler {
            match MergeHandler::resolve_conflict(destination, source, &config.merge_settings) {
                MergeAction::CreateLink => {
                    if !config.dry_run {
                        remove_existing(destination)?;
                    }
                }

                MergeAction::AppendContent => {
                    if config.dry_run {
                        info!(
                            "DRY RUN: would append content from {} to {}",
                            source.display(),
                            destination.display()
                        );
                    } else {
                        merge.append_content(destination, source, &config.merge_settings)?;
                    }

                    return Ok(true);
                }

                MergeAction::MergeDirectories => {
                    debug!(
                        "Both are directories, continuing recursion: {}",
                        destination.display()
                    );

                    return Ok(true);
                }

                MergeAction::Conflict => {
                    if !config.force && !config.adopt {
                        anyhow::bail!(
                            "Conflict: {} already exists \
(use --force, --adopt or --merge)",
                            destination.display()
                        );
                    }

                    handle_existing_destination(destination, config)?;
                }
            }
        } else {
            handle_existing_destination(destination, config)?;
        }
    }

    if config.dry_run {
        info!(
            "DRY RUN: would link {} → {}",
            destination.display(),
            source.display()
        );

        return Ok(true);
    }

    let relative = make_relative(source, destination);

    create_symlink(&relative, destination)?;

    info!("Linked: {} → {}", destination.display(), relative.display());

    Ok(true)
}

// ====================== UNSTOW ======================

fn visit_unstow(
    root: &Path,
    current: &Path,
    target_base: &Path,
    config: &Config,
    ignores: &[regex::Regex],
    stats: &mut StowStats,
) -> Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();

        let rel_path = path.strip_prefix(root).unwrap_or(&path);

        if should_ignore(rel_path, ignores) {
            continue;
        }

        // Aplicar transformação dot- se dotfiles mode está habilitado
        let destination_rel_path = if config.dotfiles {
            transform_dot_prefix(rel_path)
        } else {
            rel_path.to_path_buf()
        };

        let destination = target_base.join(&destination_rel_path);

        if entry.file_type()?.is_dir() {
            visit_unstow(root, &path, target_base, config, ignores, stats)?;

            if !config.dry_run && destination.exists() {
                let _ = fs::remove_dir(&destination);
            }
        } else if is_managed_symlink(&destination, &path) {
            if config.backup {
                backup_existing(&destination)?;
            }

            if config.dry_run {
                info!("DRY RUN: would remove {}", destination.display());
            } else {
                fs::remove_file(&destination)?;

                info!("Removed: {}", destination.display());

                stats.files_removed += 1;
            }
        }
    }

    Ok(())
}

// ====================== HELPERS ======================

fn handle_existing_destination(destination: &Path, config: &Config) -> Result<()> {
    if destination
        .symlink_metadata()
        .is_ok_and(|m| m.file_type().is_symlink())
    {
        if !config.dry_run {
            fs::remove_file(destination)?;
        }

        return Ok(());
    }

    if config.adopt {
        debug!("Adopting existing file: {}", destination.display());

        remove_existing(destination)?;
    } else if config.force {
        if config.backup {
            backup_existing(destination)?;
        }

        remove_existing(destination)?;
    } else {
        anyhow::bail!(
            "Conflict: {} already exists \
(use --force or --adopt)",
            destination.display()
        );
    }

    Ok(())
}

fn make_relative(source: &Path, destination: &Path) -> PathBuf {
    diff_paths(source, destination.parent().unwrap_or(destination))
        .unwrap_or_else(|| source.to_path_buf())
}

fn backup_existing(path: &Path) -> Result<()> {
    let mut backup = path.with_extension("bak");
    let mut counter = 1;

    while backup.exists() {
        backup = path.with_extension(format!("bak{counter}"));
        counter += 1;
    }

    fs::rename(path, &backup)?;

    info!("Backed up: {} → {}", path.display(), backup.display());

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

fn is_managed_symlink(destination: &Path, source: &Path) -> bool {
    if let Ok(link) = fs::read_link(destination) {
        let abs_link = if link.is_absolute() {
            link
        } else {
            destination
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .join(link)
        };

        if let (Ok(a), Ok(b)) = (abs_link.canonicalize(), source.canonicalize()) {
            return a == b;
        }
    }

    false
}

#[cfg(unix)]
fn create_symlink(source: &Path, destination: &Path) -> Result<()> {
    std::os::unix::fs::symlink(source, destination).map_err(|e| {
        anyhow::anyhow!(
            "Failed to create symlink {} -> {}: {}",
            destination.display(),
            source.display(),
            e
        )
    })
}

#[cfg(windows)]
fn create_symlink(source: &Path, destination: &Path) -> Result<()> {
    if source.is_dir() {
        std::os::windows::fs::symlink_dir(source, destination)
    } else {
        std::os::windows::fs::symlink_file(source, destination)
    }
    .map_err(|e| {
        anyhow::anyhow!(
            "Failed to create symlink {} -> {}: {}",
            destination.display(),
            source.display(),
            e
        )
    })
}