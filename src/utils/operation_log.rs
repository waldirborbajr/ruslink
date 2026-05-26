// src/utils/operation_log.rs
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{debug, info};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileOperation {
    pub timestamp: DateTime<Utc>,
    pub user: String,
    pub package: String,
    pub operation: String, // "create_symlink", "remove_file", "backup", "append_content", etc.
    pub source: Option<String>,
    pub destination: String,
    pub success: bool,
    pub details: Option<String>,
}

pub struct OperationLogger {
    package: String,
}

impl OperationLogger {
    pub fn new(package: String) -> Self {
        Self { package }
    }

    fn get_current_user() -> String {
        std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "unknown".to_string())
    }

    pub fn log_operation(
        &self,
        operation: &str,
        destination: &Path,
        source: Option<&Path>,
        success: bool,
        details: Option<String>,
    ) {
        let log_entry = FileOperation {
            timestamp: Utc::now(),
            user: Self::get_current_user(),
            package: self.package.clone(),
            operation: operation.to_string(),
            source: source.map(|p| p.display().to_string()),
            destination: destination.display().to_string(),
            success,
            details,
        };

        // Structured log (visible with -v)
        if success {
            info!(
                operation = %operation,
                destination = %destination.display(),
                source = ?source.map(|p| p.display()),
                "File operation completed"
            );
        } else {
            tracing::error!(
                operation = %operation,
                destination = %destination.display(),
                "File operation failed"
            );
        }

        // TODO: Write to JSON file (in next step)
        debug!(entry = ?log_entry, "Operation logged");
    }
}