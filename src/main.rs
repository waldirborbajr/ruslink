// ruslink.rs - Rust Stow with .gitignore + Auto Git Commit
mod args;
mod config;
mod git;
mod ignore;
mod stow;

use anyhow::Result;
use tracing::{debug, info, warn};
use tracing_subscriber::EnvFilter;

use args::parse_args;
use git::{auto_git_commit, auto_git_push, has_git_changes, auto_git_commit_silent};
use ignore::load_all_ignore_patterns;
use stow::{stow_package, unstow_package};

fn main() -> Result<()> {
    // Setup human-panic for better error messages on crashes
    human_panic::setup_panic!();

    let config = parse_args();

    // Setup tracing based on verbose flag
    setup_tracing(config.verbose);

    let mut config = config;
    let package_path = config.stow_dir.join(&config.package);

    if !package_path.exists() {
        anyhow::bail!(
            "Error: Package '{}' not found in {:?}",
            config.package, config.stow_dir
        );
    }

    info!("Package: {}", config.package);
    info!("Stow dir: {:?}", config.stow_dir);
    info!("Target dir: {:?}", config.target_dir);
    debug!("Config: {:?}", config);

    if config.dry_run {
        info!("*** DRY RUN MODE ***");
        config.auto_git = false; // Disable git in dry-run
    }

    let ignore_regexes = load_all_ignore_patterns(&package_path);
    debug!("Loaded {} ignore patterns", ignore_regexes.len());

    // Unstow phase
    if config.restow || config.delete {
        info!("Unstowing package '{}'...", config.package);
        unstow_package(&package_path, &config.target_dir, &config, &ignore_regexes)?;
    }

    // Stow phase
    if !config.delete {
        info!("Stowing package '{}'...", config.package);
        stow_package(&package_path, &config.target_dir, &config, &ignore_regexes)?;
    }

    // Git handling - at the end of execution
    if !config.dry_run && !config.delete {
        if config.auto_git {
            // Manual git mode: user explicitly requested git operations
            info!("Git: Checking for changes...");
            if let Err(e) = auto_git_commit(&package_path, &config) {
                warn!("Git warning: {}", e);
            }

            // Manual Git Push
            if config.git_push {
                info!("Git: Pushing changes...");
                if let Err(e) = auto_git_push(&package_path, &config) {
                    warn!("Git push error: {}", e);
                }
            }
        } else {
            // Automatic git mode: check for changes and auto-commit if there are any
            match has_git_changes(&package_path) {
                Ok(true) => {
                    info!("Changes detected in git repository. Running automatic commit...");
                    if let Err(e) = auto_git_commit_silent(&package_path, &config.package) {
                        debug!("Automatic commit failed: {}", e);
                    }
                }
                Ok(false) => {
                    debug!("No changes in git repository.");
                }
                Err(e) => {
                    debug!("Failed to check git status: {}", e);
                }
            }
        }
    }

    if config.dry_run {
        info!("Dry run completed. No changes were made.");
    } else {
        info!("Done!");
    }

    Ok(())
}

fn setup_tracing(verbose: bool) {
    let filter_level = if verbose {
        EnvFilter::new("ruslink=debug")
    } else {
        EnvFilter::new("ruslink=info")
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter_level)
        .with_writer(std::io::stdout)
        .init();
}
