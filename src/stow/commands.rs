// src/stow/commands.rs
use anyhow::Result;
use std::path::Path;
use tracing::info;
use walkdir::WalkDir;

use crate::cli::Config;

/// Lista todos os pacotes disponíveis no diretório stow
pub fn list_packages(stow_dir: &Path) -> Result<()> {
    info!("📋 Listing packages in {}", stow_dir.display());

    let mut count = 0;
    println!("Packages available in {}:\n", stow_dir.display());

    for entry in std::fs::read_dir(stow_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let name = entry.file_name();
            if let Some(name_str) = name.to_str() {
                if !name_str.starts_with('.') {
                    println!("  • {name_str}");
                    count += 1;
                }
            }
        }
    }

    println!("\nTotal: {count} package(s)");
    Ok(())
}

/// Mostra status dos pacotes
pub fn show_status(stow_dir: &Path, target_dir: &Path, config: &Config) {
    info!("🔍 Showing status");

    println!("Target  : {}", target_dir.display());
    println!("Stow dir: {}", stow_dir.display());

    if config.package.is_empty() {
        println!("\nUse `ruslink status <package>` for specific package details.");
    } else {
        let package_path = stow_dir.join(&config.package);
        if package_path.exists() {
            println!("\nPackage '{}' exists", config.package);
        } else {
            println!("\nPackage '{}' not found", config.package);
        }
    }
}

/// Limpa broken symlinks, diretórios vazios e backups antigos
#[allow(clippy::unnecessary_wraps)]
pub fn clean_target(target_dir: &Path, config: &Config) -> Result<()> {
    if config.dry_run {
        info!("🧹 DRY RUN: Cleaning {}", target_dir.display());
    } else {
        info!("🧹 Cleaning {}", target_dir.display());
    }

    let mut broken_symlinks = 0;
    let mut empty_dirs = 0;

    for entry in WalkDir::new(target_dir)
        .follow_links(false)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();

        // Broken symlink
        if path
            .symlink_metadata()
            .is_ok_and(|m| m.file_type().is_symlink())
            && !path.exists()
        {
            if !config.dry_run {
                let _ = std::fs::remove_file(path);
            }
            broken_symlinks += 1;
            println!("  🗑  Removed broken symlink: {}", path.display());
        }

        // Diretório vazio
        if path.is_dir() && !config.dry_run {
            if let Ok(entries) = std::fs::read_dir(path) {
                if entries.count() == 0 && std::fs::remove_dir(path).is_ok() {
                    empty_dirs += 1;
                    println!("  🗑  Removed empty directory: {}", path.display());
                }
            }
        }
    }

    println!("\n✅ Clean summary:");
    println!("   • {broken_symlinks} broken symlinks removed");
    println!("   • {empty_dirs} empty directories removed");

    if config.dry_run {
        println!("   (DRY RUN - nenhuma alteração foi feita)");
    }

    Ok(())
}
