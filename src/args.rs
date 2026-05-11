use clap::Parser;
use std::path::PathBuf;

use crate::config::Config;

#[derive(Parser, Debug)]
#[command(name = "ruslink")]
#[command(about = "Rust Stow with .gitignore + Auto Git Commit", long_about = None)]
#[command(version)]
struct Args {
    /// Package to stow
    package: String,

    /// Stow directory
    #[arg(short, long)]
    dir: Option<PathBuf>,

    /// Target directory
    #[arg(short, long)]
    target: Option<PathBuf>,

    /// Unstow only
    #[arg(short = 'D', long)]
    delete: bool,

    /// Unstow then stow
    #[arg(short = 'R', long)]
    restow: bool,

    /// Simulate only
    #[arg(short = 'n', long = "dry-run")]
    dry_run: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Auto commit changes to git
    #[arg(short, long)]
    git: bool,

    /// Overwrite existing destination files if present
    #[arg(long)]
    force: bool,

    /// Backup existing files before modifying them
    #[arg(long)]
    backup: bool,

    /// Custom commit message
    #[arg(short, long)]
    message: Option<String>,
}

pub fn parse_args() -> Config {
    let args = Args::parse();

    let mut stow_dir = args
        .dir
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    let mut target_dir = args.target.unwrap_or_else(|| {
        stow_dir
            .parent()
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| std::path::PathBuf::from("/"))
    });

    Config {
        package: args.package,
        stow_dir,
        target_dir,
        delete: args.delete,
        restow: args.restow,
        dry_run: args.dry_run,
        verbose: args.verbose,
        auto_git: args.git,
        force: args.force,
        backup: args.backup,
        commit_message: args.message,
    }
}
