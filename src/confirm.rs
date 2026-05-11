// src/confirm.rs
use std::io::{self, Write};
use tracing::debug;

use crate::config::Config;
use crate::output::{warning, error};

/// Solicita confirmação do usuário para ações destrutivas
pub fn confirm_action(action: &str, config: &Config) -> bool {
    if config.yes {
        debug!("Skipping confirmation due to --yes flag");
        return true;
    }

    println!();
    warning(&format!("⚠️  This action will {} the package '{}'", action, config.package));
    
    if config.force {
        warning("   --force mode is enabled: existing files may be overwritten!");
    }
    if config.delete || config.restow {
        warning("   This will remove symlinks and potentially lose manual changes.");
    }

    print!("\nAre you sure you want to continue? [y/N]: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        error(&format!("Failed to read user input: {}", e));
        return false;
    }

    let response = input.trim().to_lowercase();

    matches!(response.as_str(), "y" | "yes")
}

/// Versão simplificada para confirmações genéricas
pub fn confirm(message: &str, config: &Config) -> bool {
    if config.yes {
        return true;
    }

    println!();
    warning(message);
    print!("\nContinue? [y/N]: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);

    let response = input.trim().to_lowercase();
    matches!(response.as_str(), "y" | "yes")
}
