use anyhow::Result;
use tracing::{debug, info};

use crate::cli::parse_args;
use crate::git::handle_git_operations;
use crate::stow::{stow_package, unstow_package, StowStats};
use crate::utils::{
    confirm_action, error, load_all_ignore_patterns, setup_tracing, success, warning,
};

pub fn run() -> Result<()> {
    human_panic::setup_panic!(
        name: "ruslink",
        version: env!("CARGO_PKG_VERSION"),
        authors: "Waldir Borba Junior <wborbajr@gmail.com>",
    );

    let config = parse_args();
    setup_tracing(config.verbose);

    let package_path = config.stow_dir.join(&config.package);

    if !package_path.exists() {
        error(&format!(
            "Package '{}' not found in {:?}",
            config.package, config.stow_dir
        ));
        std::process::exit(1);
    }

    info!("Package     : {}", config.package);
    info!("Stow dir    : {:?}", config.stow_dir);
    info!("Target dir  : {:?}", config.target_dir);

    if config.dry_run {
        warning("*** DRY RUN MODE ENABLED ***");
    }

    let ignore_regexes = load_all_ignore_patterns(&package_path);
    debug!("Loaded {} ignore patterns", ignore_regexes.len());

    // ====================== CONFIRM DESTRUCTIVE ACTIONS ======================
    if !config.yes && !config.dry_run {
        if config.delete || config.restow {
            if !confirm_action("DELETE / UNSTOW", &config) {
                warning("Operation cancelled by user.");
                std::process::exit(0);
            }
        }

        if (config.force || config.adopt) && !config.delete {
            if !confirm_action("FORCE / ADOPT existing files", &config) {
                warning("Operation cancelled by user.");
                std::process::exit(0);
            }
        }
    }

    let mut total_stats = StowStats::default();

    // Unstow
    if config.restow || config.delete {
        info!("Unstowing package '{}'...", config.package);
        let unstow_stats =
            unstow_package(&package_path, &config.target_dir, &config, &ignore_regexes)?;
        total_stats.files_removed = unstow_stats.files_removed;
    }

    // Stow
    if !config.delete {
        info!("Stowing package '{}'...", config.package);
        let stow_stats =
            stow_package(&package_path, &config.target_dir, &config, &ignore_regexes)?;
        total_stats.files_linked = stow_stats.files_linked;
        total_stats.dirs_created = stow_stats.dirs_created;
        total_stats.files_ignored = stow_stats.files_ignored;
    }

    // Git Operations
    if !config.dry_run && !config.delete {
        handle_git_operations(&package_path, &config)?;
    }

    // Final Summary
    if config.dry_run {
        warning("Dry run completed. No changes were made.");
    } else {
        success("✅ Done!");

        if total_stats.files_linked > 0 || total_stats.files_removed > 0 {
            info!(
                "Summary → Linked: {} | Removed: {} | Dirs: {} | Ignored: {}",
                total_stats.files_linked,
                total_stats.files_removed,
                total_stats.dirs_created,
                total_stats.files_ignored
            );
        }
    }

    Ok(())
}