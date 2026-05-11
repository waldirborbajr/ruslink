// rustow.rs - Rust Stow with .gitignore + Auto Git Commit
mod args;
mod config;
mod git;
mod ignore;
mod stow;

use std::process;

use args::parse_args;
use git::auto_git_commit;
use ignore::load_all_ignore_patterns;
use stow::{stow_package, unstow_package};

fn main() {
    let mut config = parse_args();
    let package_path = config.stow_dir.join(&config.package);

    if !package_path.exists() {
        eprintln!(
            "Error: Package '{}' not found in {:?}",
            config.package, config.stow_dir
        );
        process::exit(1);
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
        let _ = unstow_package(&package_path, &config.target_dir, &config, &ignore_regexes);
    }

    // Stow phase
    if !config.delete {
        println!("Stowing package '{}'...", config.package);
        let _ = stow_package(&package_path, &config.target_dir, &config, &ignore_regexes);
    }

    // Auto Git Commit
    if config.auto_git && !config.dry_run && !config.delete {
        println!("\nGit: Checking for changes...");
        if let Err(e) = auto_git_commit(&package_path, &config) {
            eprintln!("Git warning: {}", e);
        }
    }

    if config.dry_run {
        println!("\nDry run completed. No changes were made.");
    } else {
        println!("\nDone!");
    }
}
