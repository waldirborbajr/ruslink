// ruslink.rs - Rust Stow with .gitignore + Auto Git Commit
mod args;
mod config;
mod git;
mod ignore;
mod stow;

use anyhow::Result;

use args::parse_args;
use git::{auto_git_commit, auto_git_push};
use ignore::load_all_ignore_patterns;
use stow::{stow_package, unstow_package};

fn main() -> Result<()> {
    let mut config = parse_args();
    let package_path = config.stow_dir.join(&config.package);

    if !package_path.exists() {
        anyhow::bail!(
            "Error: Package '{}' not found in {:?}",
            config.package, config.stow_dir
        );
    }

    println!("Package: {}", config.package);
    println!("Stow dir: {:?}", config.stow_dir);
    println!("Target dir: {:?}", config.target_dir);

    if config.dry_run {
        println!("*** DRY RUN MODE ***");
        config.auto_git = false; // Disable git in dry-run
    }

    let ignore_regexes = load_all_ignore_patterns(&package_path);

    // Unstow phase
    if config.restow || config.delete {
        println!("Unstowing package '{}'...", config.package);
        unstow_package(&package_path, &config.target_dir, &config, &ignore_regexes)?;
    }

    // Stow phase
    if !config.delete {
        println!("Stowing package '{}'...", config.package);
        stow_package(&package_path, &config.target_dir, &config, &ignore_regexes)?;
    }

    // Auto Git Commit
    if config.auto_git && !config.dry_run && !config.delete {
        println!("\nGit: Checking for changes...");
        if let Err(e) = auto_git_commit(&package_path, &config) {
            eprintln!("Git warning: {}", e);
        }

        // Auto Git Push
        if config.git_push {
            println!("\nGit: Pushing changes...");
            if let Err(e) = auto_git_push(&package_path, &config) {
                eprintln!("Git push error: {}", e);
            }
        }
    }

    if config.dry_run {
        println!("\nDry run completed. No changes were made.");
    } else {
        println!("\nDone!");
    }

    Ok(())
}
