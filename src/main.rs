// src/main.rs
mod args;
mod config;
mod confirm;
mod git;
mod ignore;
mod output;
mod stow;

use anyhow::Result;
use tracing::{debug, info, warn};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use args::parse_args;
use confirm::confirm_action;
use git::GitRepository;
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
        handle_git_operations(&package_path, &config)?;
    }

    if config.dry_run {
        warning("Dry run completed. No changes were made.");
    } else {
        success("✅ Done!");
    }

    Ok(())
}

/// Gerencia todas as operações Git
fn handle_git_operations(package_path: &std::path::Path, config: &config::Config) -> Result<()> {
    // Verifica se o Git está instalado
    if let Err(e) = GitRepository::ensure_git_installed() {
        error(&format!("Git error: {}", e));
        std::process::exit(1);
    }

    let repo = GitRepository::new(package_path);

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

/// Configura tracing com suporte completo a RUST_LOG
fn setup_tracing(verbose: bool) {
    let filter = if verbose {
        // --verbose força debug
        EnvFilter::new("ruslink=debug")
    } else {
        // Permite controle via RUST_LOG (ex: RUST_LOG=debug ruslink ...)
        EnvFilter::from_default_env()
            .add_directive("ruslink=info".parse().unwrap())
    };

    fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stdout)
        .init();
}
