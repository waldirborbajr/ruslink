// src/stow/stowmanager.rs

use std::fs;
use std::path::{Component, Path, PathBuf};
use std::time::Instant;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use super::merge::{MergeAction, MergeHandler};
use crate::cli::Config;
use crate::utils::should_ignore;

// ====================== OPERATION LOGGER ======================

#[derive(Debug, Serialize, Deserialize)]
pub struct FileOperation {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user: String,
    pub package: String,
    pub operation: String,
    pub source: Option<String>,
    pub destination: String,
    pub success: bool,
    pub details: Option<String>,
}

pub struct OperationLogger {
    package: String,
}

impl OperationLogger {
    pub const fn new(package: String) -> Self {
        Self { package }
    }

    fn get_current_user() -> String {
        std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "unknown".to_string())
    }

    pub fn log(
        &self,
        operation: &str,
        destination: &Path,
        source: Option<&Path>,
        success: bool,
        details: Option<String>,
    ) {
        let entry = FileOperation {
            timestamp: chrono::Utc::now(),
            user: Self::get_current_user(),
            package: self.package.clone(),
            operation: operation.to_string(),
            source: source.map(|p| p.display().to_string()),
            destination: destination.display().to_string(),
            success,
            details,
        };

        if success {
            info!(
                operation = %operation,
                dest = %destination.display(),
                src = ?source.map(|p| p.display()),
                "File operation completed"
            );
        } else {
            tracing::error!(
                operation = %operation,
                dest = %destination.display(),
                "File operation failed"
            );
        }

        debug!(entry = ?entry, "Operation logged as JSON");
    }
}

// ====================== RECURSION CONTEXT ======================

struct VisitContext<'a> {
    root: &'a Path,
    target_base: &'a Path,
    config: &'a Config,
    ignores: &'a [regex::Regex],
    merge_handler: Option<&'a MergeHandler>,
    logger: &'a OperationLogger,
    stats: &'a mut StowStats,
}

// ====================== PATH CONTAINMENT (SECURITY) ======================

/// Returns true if `path` contains components that can escape a base directory
/// (`..`, root, or Windows prefix). Package-relative paths must never include these.
fn path_has_escape(path: &Path) -> bool {
    path.components().any(|c| {
        matches!(
            c,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    })
}

/// Lexically normalize a path (resolve `.` and `..` without touching the filesystem).
/// Used so containment checks work even when the destination does not exist yet.
fn normalize_path(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();

    for component in path.components() {
        match component {
            Component::Prefix(prefix) => result.push(prefix.as_os_str()),
            Component::RootDir => result.push(component.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                // Do not pop past the root / prefix
                if result.as_os_str().is_empty() {
                    // Relative path that starts with .. — keep marker for starts_with failure
                    result.push("..");
                } else {
                    let last = result.components().next_back();
                    match last {
                        Some(Component::Normal(_)) => {
                            result.pop();
                        }
                        _ => {
                            result.push("..");
                        }
                    }
                }
            }
            Component::Normal(name) => result.push(name),
        }
    }

    result
}

/// Resolve `path` to an absolute path without requiring the final component to exist.
fn absolute_path(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    }
}

/// Ensure `destination` stays strictly under `target_base` (no path escape).
///
/// Checks:
/// 1. Lexical components of `destination` must not introduce escapes relative to target
/// 2. After normalization, the destination path must start with the normalized target
///
/// Call this before any create / remove / merge on a destination path.
fn ensure_under_target(destination: &Path, target_base: &Path) -> Result<()> {
    if path_has_escape(destination) && !destination.is_absolute() {
        // Relative destination still carrying `..` is always unsafe
        anyhow::bail!(
            "Refusing path with escape components (`..` or absolute): {} \
             (must stay under target {})",
            destination.display(),
            target_base.display()
        );
    }

    let target_abs = absolute_path(target_base);
    let dest_abs = absolute_path(destination);

    let target_norm = normalize_path(&target_abs);
    let dest_norm = normalize_path(&dest_abs);

    // Destination must equal target or be a strict child of target
    if dest_norm != target_norm && !dest_norm.starts_with(&target_norm) {
        anyhow::bail!(
            "Refusing path outside target directory: {} (resolved: {}) — \
             target is {} (resolved: {})",
            destination.display(),
            dest_norm.display(),
            target_base.display(),
            target_norm.display()
        );
    }

    // Extra guard: if dest_norm somehow still contains ParentDir, reject.
    if path_has_escape(&dest_norm) {
        anyhow::bail!(
            "Refusing unresolved escape in destination path: {}",
            dest_norm.display()
        );
    }

    Ok(())
}

/// Reject package-relative paths that contain `..`, root, or prefix components.
fn reject_package_escape(rel_path: &Path) -> Result<()> {
    if path_has_escape(rel_path) {
        anyhow::bail!(
            "Refusing package path with escape components (`..` or absolute): {}. \
             Package entries must stay within the package directory.",
            rel_path.display()
        );
    }
    Ok(())
}

// ====================== DOTFILES HANDLING ======================

/// Transform `dot-` prefixes to leading dots (e.g. `dot-bashrc` → `.bashrc`).
///
/// Security: rejects any resulting path that contains `..`, root, or prefix
/// components. A crafted name such as `dot-..` would otherwise become `..`
/// and escape the target directory.
fn transform_dot_prefix(path: &Path) -> Result<PathBuf> {
    let mut components = Vec::new();

    for comp in path.components() {
        match comp {
            Component::Normal(os_str) => {
                let name = os_str.to_string_lossy();
                if let Some(stripped) = name.strip_prefix("dot-") {
                    // Reject empty strip or escape-like results
                    if stripped.is_empty() || stripped == ".." || stripped == "." {
                        anyhow::bail!(
                            "Refusing unsafe dotfiles transform of component '{}': \
                             would produce an escape or empty name",
                            name
                        );
                    }
                    // If stripped still looks like a path escape when parsed
                    let candidate = Path::new(stripped);
                    if path_has_escape(candidate) {
                        anyhow::bail!(
                            "Refusing unsafe dotfiles transform of component '{}': \
                             result contains escape components",
                            name
                        );
                    }
                    components.push(format!(".{stripped}"));
                } else {
                    if name == ".." || name == "." {
                        anyhow::bail!(
                            "Refusing package component '{}': escape or current-dir name",
                            name
                        );
                    }
                    components.push(name.into_owned());
                }
            }
            Component::ParentDir => {
                anyhow::bail!(
                    "Refusing path with parent-directory component (`..`): {}",
                    path.display()
                );
            }
            Component::CurDir => {}
            Component::RootDir | Component::Prefix(_) => {
                anyhow::bail!(
                    "Refusing absolute or prefixed path component in package entry: {}",
                    path.display()
                );
            }
        }
    }

    let result: PathBuf = components.iter().collect();

    // Final safety check on the assembled path
    reject_package_escape(&result)?;

    Ok(result)
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
    let logger = OperationLogger::new(config.package.clone());

    info!("Stowing from {} → {}", source.display(), target.display());

    let merge_handler = if config.is_merge_enabled() {
        Some(MergeHandler::new(source, config.package.clone()))
    } else {
        None
    };

    let mut stats = StowStats::default();

    let ctx = VisitContext {
        root: source,
        target_base: target,
        config,
        ignores,
        merge_handler: merge_handler.as_ref(),
        logger: &logger,
        stats: &mut stats,
    };

    visit_source(ctx)?;

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
    let logger = OperationLogger::new(config.package.clone());

    info!("Unstowing from {} → {}", source.display(), target.display());

    let mut stats = StowStats::default();

    let ctx = VisitContext {
        root: source,
        target_base: target,
        config,
        ignores,
        merge_handler: None,
        logger: &logger,
        stats: &mut stats,
    };

    visit_unstow(ctx)?;

    let elapsed = start.elapsed();
    stats.print_summary("Unstow", elapsed);

    Ok(stats)
}

// ====================== STOW ======================

// Allow is needed because we pass by value to simplify recursion with mutable stats
#[allow(clippy::needless_pass_by_value)]
fn visit_source(ctx: VisitContext) -> Result<()> {
    for entry in fs::read_dir(ctx.root)? {
        let entry = entry?;
        let path = entry.path();

        let rel_path = path.strip_prefix(ctx.root).unwrap_or(&path);

        if should_ignore(rel_path, ctx.ignores) {
            ctx.stats.files_ignored += 1;
            debug!("Ignored: {}", rel_path.display());
            continue;
        }

        // Reject escape components in the package-relative path before any transform
        reject_package_escape(rel_path)?;

        let destination_rel_path = if ctx.config.dotfiles {
            transform_dot_prefix(rel_path)?
        } else {
            rel_path.to_path_buf()
        };

        // Reject escape after transform as well
        reject_package_escape(&destination_rel_path)?;

        let destination = ctx.target_base.join(&destination_rel_path);

        // Containment: destination must stay under target_base
        ensure_under_target(&destination, ctx.target_base)?;

        if entry.file_type()?.is_dir() {
            if !ctx.config.dry_run {
                fs::create_dir_all(&destination)?;
                ctx.logger
                    .log("create_directory", &destination, None, true, None);
                ctx.stats.dirs_created += 1;
            }

            let sub_ctx = VisitContext {
                root: &path,
                target_base: ctx.target_base,
                config: ctx.config,
                ignores: ctx.ignores,
                merge_handler: ctx.merge_handler,
                logger: ctx.logger,
                stats: ctx.stats,
            };
            visit_source(sub_ctx)?;
        } else if stow_item(
            &path,
            &destination,
            ctx.target_base,
            ctx.config,
            ctx.merge_handler,
            ctx.logger,
        )? {
            ctx.stats.files_linked += 1;
        }
    }

    Ok(())
}

fn stow_item(
    source: &Path,
    destination: &Path,
    target_base: &Path,
    config: &Config,
    merge_handler: Option<&MergeHandler>,
    logger: &OperationLogger,
) -> Result<bool> {
    // Containment check before any mutation
    ensure_under_target(destination, target_base)?;

    if let Some(parent) = destination.parent() {
        ensure_under_target(parent, target_base)?;
        if !config.dry_run {
            fs::create_dir_all(parent)?;
        }
    }

    if !config.dry_run {
        validate_symlink_target(source, destination)?;
        detect_circular_symlink(source, destination)?;
    }

    if destination.exists() || destination.symlink_metadata().is_ok() {
        if let Some(merge) = merge_handler {
            match MergeHandler::resolve_conflict(destination, source, &config.merge_settings) {
                MergeAction::CreateLink => {
                    if !config.dry_run {
                        remove_existing(destination, target_base, logger)?;
                    }
                }

                MergeAction::AppendContent => {
                    // Re-check containment before writing merged content
                    ensure_under_target(destination, target_base)?;

                    if config.dry_run {
                        info!(
                            "DRY RUN: would append content from {} to {}",
                            source.display(),
                            destination.display()
                        );
                    } else {
                        merge.append_content(
                            destination,
                            source,
                            &config.merge_settings,
                            logger,
                        )?;
                    }
                    logger.log("append_content", destination, Some(source), true, None);
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
                            "Conflict: {} already exists (use --force, --adopt or --merge)",
                            destination.display()
                        );
                    }
                    handle_existing_destination(destination, target_base, config, logger)?;
                }
            }
        } else {
            handle_existing_destination(destination, target_base, config, logger)?;
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

    logger.log("create_symlink", destination, Some(source), true, None);
    info!("Linked: {} → {}", destination.display(), relative.display());

    Ok(true)
}

// ====================== UNSTOW ======================

#[allow(clippy::needless_pass_by_value)]
fn visit_unstow(ctx: VisitContext) -> Result<()> {
    for entry in fs::read_dir(ctx.root)? {
        let entry = entry?;
        let path = entry.path();

        let rel_path = path.strip_prefix(ctx.root).unwrap_or(&path);

        if should_ignore(rel_path, ctx.ignores) {
            continue;
        }

        reject_package_escape(rel_path)?;

        let destination_rel_path = if ctx.config.dotfiles {
            transform_dot_prefix(rel_path)?
        } else {
            rel_path.to_path_buf()
        };

        reject_package_escape(&destination_rel_path)?;

        let destination = ctx.target_base.join(&destination_rel_path);

        ensure_under_target(&destination, ctx.target_base)?;

        if entry.file_type()?.is_dir() {
            let sub_ctx = VisitContext {
                root: &path,
                target_base: ctx.target_base,
                config: ctx.config,
                ignores: ctx.ignores,
                merge_handler: ctx.merge_handler,
                logger: ctx.logger,
                stats: ctx.stats,
            };
            visit_unstow(sub_ctx)?;

            if !ctx.config.dry_run && destination.exists() {
                // Only remove empty dirs that are still under target
                ensure_under_target(&destination, ctx.target_base)?;
                let _ = fs::remove_dir(&destination);
                ctx.logger
                    .log("remove_directory", &destination, None, true, None);
            }
        } else if is_managed_symlink(&destination, &path) {
            if ctx.config.backup {
                backup_existing(&destination, ctx.logger)?;
            }

            if ctx.config.dry_run {
                info!("DRY RUN: would remove {}", destination.display());
            } else {
                ensure_under_target(&destination, ctx.target_base)?;
                fs::remove_file(&destination)?;
                ctx.logger
                    .log("remove_symlink", &destination, Some(&path), true, None);
                info!("Removed: {}", destination.display());
                ctx.stats.files_removed += 1;
            }
        }
    }

    Ok(())
}

// ====================== HELPERS ======================

fn handle_existing_destination(
    destination: &Path,
    target_base: &Path,
    config: &Config,
    logger: &OperationLogger,
) -> Result<()> {
    ensure_under_target(destination, target_base)?;

    if destination
        .symlink_metadata()
        .is_ok_and(|m| m.file_type().is_symlink())
    {
        if !config.dry_run {
            fs::remove_file(destination)?;
            logger.log("remove_existing_symlink", destination, None, true, None);
        }
        return Ok(());
    }

    if config.adopt {
        debug!("Adopting existing file: {}", destination.display());
        remove_existing(destination, target_base, logger)?;
    } else if config.force {
        if config.backup {
            backup_existing(destination, logger)?;
        }
        remove_existing(destination, target_base, logger)?;
    } else {
        anyhow::bail!(
            "Conflict: {} already exists (use --force or --adopt)",
            destination.display()
        );
    }

    Ok(())
}

fn validate_symlink_target(source: &Path, destination: &Path) -> Result<()> {
    if !source.exists() {
        anyhow::bail!(
            "Symlink target does not exist: {} (for destination {})",
            source.display(),
            destination.display()
        );
    }

    if source.is_symlink() {
        debug!(
            "Target is itself a symlink: {} -> {}",
            source.display(),
            fs::read_link(source)?.display()
        );
    }

    if make_relative(source, destination).components().count() == 0 {
        anyhow::bail!(
            "Failed to compute relative path from {} to {}",
            source.display(),
            destination.display()
        );
    }

    if let Ok(canonical_src) = source.canonicalize() {
        if let Ok(canonical_dst) = destination.canonicalize() {
            if canonical_src == canonical_dst {
                anyhow::bail!(
                    "Cannot create symlink pointing to itself: {}",
                    source.display()
                );
            }
        }
    }

    Ok(())
}

fn make_relative(source: &Path, destination: &Path) -> PathBuf {
    let parent = destination.parent().unwrap_or(destination);
    pathdiff::diff_paths(source, parent).unwrap_or_else(|| {
        debug!("Failed to compute relative path, falling back to absolute");
        source.to_path_buf()
    })
}

fn backup_existing(path: &Path, logger: &OperationLogger) -> Result<()> {
    let mut backup = path.with_extension("bak");
    let mut counter = 1;

    while backup.exists() {
        backup = path.with_extension(format!("bak{counter}"));
        counter += 1;
    }

    fs::rename(path, &backup)?;
    logger.log("create_backup", &backup, Some(path), true, None);
    info!("Backed up: {} → {}", path.display(), backup.display());

    Ok(())
}

fn remove_existing(path: &Path, target_base: &Path, logger: &OperationLogger) -> Result<()> {
    // Final containment check before destructive removal
    ensure_under_target(path, target_base)?;

    let meta = path.symlink_metadata()?;

    if meta.is_dir() && !meta.file_type().is_symlink() {
        fs::remove_dir_all(path)?;
        logger.log("remove_directory_recursive", path, None, true, None);
    } else {
        fs::remove_file(path)?;
        logger.log("remove_file", path, None, true, None);
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

fn detect_circular_symlink(source: &Path, destination: &Path) -> Result<()> {
    if !destination.exists() && destination.symlink_metadata().is_err() {
        return Ok(());
    }

    let Ok(canonical_source) = source.canonicalize() else {
        return Ok(());
    };

    let Ok(canonical_dest) = destination.canonicalize() else {
        return Ok(());
    };

    if canonical_source == canonical_dest {
        anyhow::bail!(
            "Circular symlink detected: {} would point to itself (or existing loop)",
            destination.display()
        );
    }

    if let Ok(link_target) = fs::read_link(destination) {
        let abs_link = if link_target.is_absolute() {
            link_target
        } else {
            destination
                .parent()
                .unwrap_or(destination)
                .join(link_target)
        };

        if let Ok(canonical_link) = abs_link.canonicalize() {
            if canonical_link == canonical_source {
                anyhow::bail!(
                    "Would create circular symlink: {} → {} (already loops back)",
                    destination.display(),
                    source.display()
                );
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod path_containment_tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn rejects_parent_dir_component() {
        assert!(path_has_escape(Path::new("foo/../bar")));
        assert!(path_has_escape(Path::new("..")));
        assert!(path_has_escape(Path::new("../etc/passwd")));
        assert!(!path_has_escape(Path::new("foo/bar")));
        assert!(!path_has_escape(Path::new(".bashrc")));
    }

    #[test]
    fn transform_rejects_dot_dot() {
        assert!(transform_dot_prefix(Path::new("dot-..")).is_err());
        assert!(transform_dot_prefix(Path::new("dot-.")).is_err());
        assert!(transform_dot_prefix(Path::new("foo/..")).is_err());
    }

    #[test]
    fn transform_accepts_safe_dotfiles() {
        let result = transform_dot_prefix(Path::new("dot-bashrc")).unwrap();
        assert_eq!(result, PathBuf::from(".bashrc"));

        let result = transform_dot_prefix(Path::new("dot-config/nvim")).unwrap();
        assert_eq!(result, PathBuf::from(".config/nvim"));
    }

    #[test]
    fn ensure_under_target_rejects_escape() {
        let target = Path::new("/home/user/dotfiles-target");
        let dest = Path::new("/home/user/dotfiles-target/../../etc/passwd");
        let result = ensure_under_target(dest, target);
        assert!(result.is_err());
    }

    #[test]
    fn normalize_resolves_parent() {
        let p = normalize_path(Path::new("/home/user/foo/../bar"));
        assert_eq!(p, PathBuf::from("/home/user/bar"));
    }
}
