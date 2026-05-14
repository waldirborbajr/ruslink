// src/cli/args.rs
use clap::Parser;
use std::path::PathBuf;

use super::config::Config;

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(name = "ruslink")]
#[command(after_help = "Examples:
  ruslink nvim
  ruslink nvim --git --message \"Update neovim\"
  ruslink nvim --restow --force
  ruslink nvim --dry-run -v
  
MERGE MODE (múltiplos packages):
  ruslink base --target ~
  ruslink dev --target ~ --merge --merge-append
  ruslink gui --target ~ --merge --merge-append --merge-extensions .bashrc,.zshrc")]
struct Args {
    /// Package name to manage
    package: String,

    /// Stow directory (default: current directory)
    #[arg(short = 'd', long)]
    dir: Option<PathBuf>,

    /// Target directory (default: parent of stow dir)
    #[arg(short = 't', long)]
    target: Option<PathBuf>,

    /// Delete/unstow only
    #[arg(short = 'D', long)]
    delete: bool,

    /// Restow (unstow then stow)
    #[arg(short = 'R', long)]
    restow: bool,

    /// Dry run (simulate only)
    #[arg(short = 'n', long)]
    dry_run: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Enable git integration (commit changes)
    #[arg(short, long)]
    git: bool,

    /// Push to remote after commit
    #[arg(long)]
    git_push: bool,

    /// Force overwrite existing files
    #[arg(long)]
    force: bool,

    /// Create backup before overwriting
    #[arg(long)]
    backup: bool,

    /// Adopt existing files (replace with symlink)
    #[arg(long)]
    adopt: bool,

    /// Custom commit message
    #[arg(short = 'm', long)]
    message: Option<String>,

    /// Automatically answer yes to all prompts (skip confirmation)
    #[arg(short = 'y', long = "yes")]
    yes: bool,

    // ====================== NOVO: MERGE MODE ======================
    /// Enable merge mode: allow multiple packages to modify same files
    #[arg(long)]
    merge: bool,

    /// When merging, append content instead of conflicting
    #[arg(long)]
    merge_append: bool,

    /// File extensions to auto-append (comma-separated, e.g. ".bashrc,.zshrc")
    #[arg(long)]
    merge_extensions: Option<String>,

    /// Show merge history and exit
    #[arg(long)]
    show_merge_history: bool,
}

pub fn parse_args() -> Config {
    let args = Args::parse();

    let stow_dir =
        args.dir.unwrap_or_else(|| std::env::current_dir().expect("Failed to get current dir"));

    let target_dir = args.target.unwrap_or_else(|| {
        stow_dir.parent().map(PathBuf::from).unwrap_or_else(|| PathBuf::from("/"))
    });

    // ====================== NOVO: PARSE MERGE CONFIG ======================

    let mut merge_config = crate::stow::MergeConfig::default();

    if args.merge || args.merge_append {
        merge_config.enabled = true;
    }

    if let Some(exts) = args.merge_extensions {
        merge_config.append_extensions = exts.split(',').map(|e| e.trim().to_string()).collect();
    }

    Config {
        package: args.package,
        stow_dir,
        target_dir,
        delete: args.delete,
        restow: args.restow,
        dry_run: args.dry_run,
        verbose: args.verbose,
        auto_git: args.git,
        git_push: args.git_push,
        force: args.force,
        backup: args.backup,
        adopt: args.adopt,
        commit_message: args.message,
        yes: args.yes,

        // NOVO
        merge: args.merge || args.merge_append,
        merge_config,
        show_merge_history: args.show_merge_history,
    }
}
