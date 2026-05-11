// src/main.rs
mod args;
mod config;
mod git;
mod ignore;
mod output;
mod stow;

use anyhow::Result;
use tracing::{debug, info, warn};

use args::parse_args;
use git::GitRepository;           // ← Import atualizado
use ignore::load_all_ignore_patterns;
use output::{success, error, warning};
use stow::{stow_package, unstow_package};

fn main() -> Result<()> {
    human_panic::setup_panic!();

    let config = parse_args();
    setup_tracing(config.verbose);

    let package_path = config.stow_dir.join(&config.package);

    if !package_path.exists() {
        error(&format!("Package '{}' not found in {:?}", config.package, config.stow_dir));
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

    // Unstow
    if config.restow || config.delete {
        info!("Unstowing package '{}'...", config.package);
        unstow_package(&package_path, &config.target_dir, &config, &ignore_regexes)?;
    }

    // Stow
    if !config.delete {
        info!("Stowing package '{}'...", config.package);
        stow_package(&package_path, &config.target_dir, &config, &ignore_regexes)?;
    }

    // Git Operations
    if !config.dry_run && !config.delete {
        let repo = GitRepository::new(&package_path);
        handle_git_operations(&repo, &config)?;
    }

    if config.dry_run {
        warning("Dry run completed. No changes were made.");
    } else {
        success("✅ Done!");
    }

    Ok(())
}

fn handle_git_operations(repo: &GitRepository, config: &config::Config) -> Result<()> {
    if config.auto_git {
        info!("Git: Checking for changes...");
        
        if let Err(e) = repo.commit(config) {
            warning(&format!("Git commit warning: {}", e));
        }

        if config.git_push {
            info!("Git: Pushing changes...");
            if let Err(e) = repo.push() {
                warning(&format!("Git push failed: {}", e));
            }
        }
    } else if let Ok(true) = repo.has_changes() {
        info!("Changes detected. Creating automatic commit...");
        if let Err(e) = repo.auto_commit_silent(&config.package) {
            debug!("Auto-commit failed: {}", e);
        }
    }
    Ok(())
}

fn setup_tracing(verbose: bool) {
    let level = if verbose { "debug" } else { "info" };

    tracing_subscriber::fmt()
        .with_env_filter(format!("ruslink={}", level))
        .with_writer(std::io::stdout)
        .init();
}
