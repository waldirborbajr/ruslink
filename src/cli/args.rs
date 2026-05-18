
// src/cli/args.rs
#![allow(clippy::struct_excessive_bools)]

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
  ruslink list
  ruslink status
  ruslink clean --dry-run

MERGE MODE:
  ruslink base --target ~
  ruslink dev --target ~ --merge --merge-append
  ruslink gui --target ~ --merge --merge-append --merge-extensions .bashrc,.zshrc

DOTFILES MODE:
  ruslink bash --dotfiles
  ruslink nvim --dotfiles --merge --merge-append
  ruslink shell --dotfiles --dry-run -v")]
struct Args {
    /// Package name (optional for list/status/clean)
    package: Option<String>,

    /// Stow directory
    #[arg(short = 'd', long)]
    dir: Option<PathBuf>,

    /// Target directory
    #[arg(short = 't', long)]
    target: Option<PathBuf>,

    /// List available packages
    #[arg(long)]
    list: bool,

    /// Show detailed status
    #[arg(long)]
    status: bool,

    /// Clean broken symlinks, empty dirs and old backups
    #[arg(long)]
    clean: bool,

    /// Delete/unstow only
    #[arg(short = 'D', long)]
    delete: bool,

    /// Restow
    #[arg(short = 'R', long)]
    restow: bool,

    /// Dry run
    #[arg(short = 'n', long)]
    dry_run: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Enable git integration
    #[arg(short, long)]
    git: bool,

    /// Push after commit
    #[arg(long)]
    git_push: bool,

    /// Force overwrite
    #[arg(long)]
    force: bool,

    /// Backup existing files
    #[arg(long)]
    backup: bool,

    /// Adopt existing files
    #[arg(long)]
    adopt: bool,

    /// Commit message
    #[arg(short = 'm', long)]
    message: Option<String>,

    /// Auto-confirm prompts
    #[arg(short = 'y', long = "yes")]
    yes: bool,

    /// Enable merge mode
    #[arg(long)]
    merge: bool,

    /// Append mergeable files
    #[arg(long)]
    merge_append: bool,

    /// Merge extensions
    #[arg(long)]
    merge_extensions: Option<String>,

    /// Show merge history
    #[arg(long)]
    show_merge_history: bool,

    /// Enable dotfiles mode
    #[arg(long)]
    dotfiles: bool,
}

pub fn parse_args() -> Config {
    let args = Args::parse();

    let stow_dir = args
        .dir
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current dir"));

    let target_dir = args.target.unwrap_or_else(|| {
        stow_dir.parent().map_or_else(|| PathBuf::from("/"), PathBuf::from)
    });

    let mut merge_settings = crate::stow::MergeConfig::default();

    if args.merge || args.merge_append {
        merge_settings.enabled = true;
    }

    if let Some(exts) = args.merge_extensions {
        merge_settings.append_extensions = exts.split(',').map(|e| e.trim().to_string()).collect();
    }

    Config {
        package: args.package.unwrap_or_default(),
        stow_dir,
        target_dir,

        list: args.list,
        status: args.status,
        clean: args.clean,

        delete: args.delete,
        restow: args.restow,
        dry_run: args.dry_run,

        verbose: args.verbose,

        auto_git: args.git,
        git_push: args.git_push,
        commit_message: args.message,

        force: args.force,
        backup: args.backup,
        adopt: args.adopt,

        yes: args.yes,

        merge: args.merge || args.merge_append,
        merge_settings,

        show_merge_history: args.show_merge_history,
        dotfiles: args.dotfiles,
    }
}