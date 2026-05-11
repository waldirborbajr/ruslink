use std::env;
use std::path::PathBuf;
use std::process;

use crate::config::Config;

pub fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();
    let mut stow_dir = env::current_dir().unwrap();
    let mut target_dir = stow_dir.parent().map(PathBuf::from).unwrap_or_else(|| PathBuf::from("/"));
    let mut delete = false;
    let mut restow = false;
    let mut dry_run = false;
    let mut verbose = false;
    let mut auto_git = false;
    let mut commit_message = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-d" | "--dir" => { if i + 1 < args.len() { stow_dir = PathBuf::from(&args[i + 1]); i += 1; } }
            "-t" | "--target" => { if i + 1 < args.len() { target_dir = PathBuf::from(&args[i + 1]); i += 1; } }
            "--delete" | "-D" => delete = true,
            "--restow" | "-R" => restow = true,
            "--dry-run" | "-n" => dry_run = true,
            "--verbose" | "-v" => verbose = true,
            "--git" | "-g" => auto_git = true,
            "--message" | "-m" => {
                if i + 1 < args.len() {
                    commit_message = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    let package = match args.get(1) {
        Some(p) if !p.starts_with('-') => p.clone(),
        _ => {
            eprintln!("Usage: {} <package> [options]", args[0]);
            eprintln!("Options:");
            eprintln!("  --delete, -D     Unstow only");
            eprintln!("  --restow, -R     Unstow then stow");
            eprintln!("  --dry-run, -n    Simulate only");
            eprintln!("  --git, -g        Auto commit changes to git");
            eprintln!("  -m, --message    Custom commit message");
            process::exit(1);
        }
    };

    Config {
        package,
        stow_dir,
        target_dir,
        delete,
        restow,
        dry_run,
        verbose,
        auto_git,
        commit_message,
    }
}
