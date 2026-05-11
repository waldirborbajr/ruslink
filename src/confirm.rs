// src/confirm.rs
use std::io::{self, Write};
use crate::output::{warning, error};

/// Ask for confirmation on destructive actions
pub fn confirm_action(action: &str, config: &crate::config::Config) -> bool {
    if config.yes {
        return true;
    }

    warning(&format!("This will {} the package '{}'.", action, config.package));
    print!("Are you sure? [y/N] ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_lowercase();
    matches!(input.as_str(), "y" | "yes")
}
