// src/utils/tracing.rs

use std::path::PathBuf;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn setup_tracing(verbose: bool, package_name: &str) {
    let filter = if verbose {
        EnvFilter::new("ruslink=debug")
    } else {
        EnvFilter::from_default_env().add_directive("ruslink=info".parse().unwrap())
    };

    // Console layer (human readable)
    let console_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(filter);

    // JSON file logging with daily rotation
    let log_dir = std::env::var("RUSLINK_LOG_DIR").map_or_else(
        |_| {
            dirs::data_dir()
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
                .join("ruslink/logs")
        },
        PathBuf::from,
    );

    // Ensure log directory exists
    let _ = std::fs::create_dir_all(&log_dir);

    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        log_dir,
        format!("ruslink-{package_name}.json"),
    );

    let file_layer = fmt::layer()
        .json()
        .with_writer(file_appender)
        .with_filter(EnvFilter::new("ruslink=info"));

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();
}
