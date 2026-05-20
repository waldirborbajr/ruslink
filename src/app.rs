// src/app.rs

use anyhow::Result;
use tracing::{debug, info};

use crate::cli::parse_args;
use crate::git::handle_git_operations;
use crate::stow::{clean_target, list_packages, show_status, stow_package, unstow_package};
use crate::utils::{
    confirm_action, error, load_all_ignore_patterns, setup_tracing, success, warning,
};

pub fn run() -> Result<()> {
    human_panic::setup_panic!(human_panic::metadata!());

    let config = parse_args();

    // Setup tracing with package name for operation logging
    setup_tracing(config.verbose, &config.package);

    // ======================
    // NEW COMMANDS
    // ======================
    if config.list {
        return list_packages(&config.stow_dir);
    }

    if config.status {
        show_status(&config.stow_dir, &config.target_dir, &config);
        return Ok(());
    }

    if config.clean {
        return clean_target(&config.target_dir, &config);
    }

    // ======================
    // CLASSIC STOW OPERATIONS
    // ======================
    let package_path = config.stow_dir.join(&config.package);

    if !package_path.exists() {
        error(&format!(
            "Package '{}' not found in {}",
            config.package,
            config.stow_dir.display()
        ));
        std::process::exit(1);
    }

    info!("Package     : {}", config.package);
    info!("Stow dir    : {}", config.stow_dir.display());
    info!("Target dir  : {}", config.target_dir.display());

    if config.dotfiles {
        info!("Dotfiles    : enabled (dot- prefix → .)");
    }

    if config.dry_run {
        warning("*** DRY RUN MODE ENABLED ***");
    }

    let ignore_regexes = load_all_ignore_patterns(&package_path);
    debug!("Loaded {} ignore patterns", ignore_regexes.len());

    // Confirm destructive actions
    if !config.yes && !config.dry_run && config.is_destructive() {
        if (config.delete || config.restow) && !confirm_action("DELETE / UNSTOW", &config) {
            warning("Operation cancelled by user.");
            std::process::exit(0);
        }

        if (config.force || config.adopt)
            && !config.delete
            && !confirm_action("FORCE / ADOPT", &config)
        {
            warning("Operation cancelled by user.");
            std::process::exit(0);
        }
    }

    // We still collect stats for potential future use / logging
    let mut _total_stats = crate::stow::StowStats::default();

    // UNSTOW
    if config.restow || config.delete {
        info!("Unstowing package '{}'...", config.package);
        let unstow_stats =
            unstow_package(&package_path, &config.target_dir, &config, &ignore_regexes)?;

        _total_stats.files_removed = unstow_stats.files_removed;
    }

    // STOW
    if !config.delete {
        info!("Stowing package '{}'...", config.package);
        let stow_stats = stow_package(&package_path, &config.target_dir, &config, &ignore_regexes)?;

        _total_stats.files_linked = stow_stats.files_linked;
        _total_stats.dirs_created = stow_stats.dirs_created;
        _total_stats.files_ignored = stow_stats.files_ignored;
    }

    // GIT OPERATIONS
    if !config.dry_run && !config.delete {
        handle_git_operations(&package_path, &config);
    }

    // FINAL SUMMARY
    if config.dry_run {
        warning("Dry run completed. No changes were made.");
    } else {
        success("✅ Done!");

        // Optional: You can re-enable detailed summary if desired
        // info!(
        //     "Summary → Linked: {} | Removed: {} | Dirs: {} | Ignored: {}",
        //     _total_stats.files_linked,
        //     _total_stats.files_removed,
        //     _total_stats.dirs_created,
        //     _total_stats.files_ignored
        // );
    }

    Ok(())
}
