// src/utils/tracing.rs
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn setup_tracing(verbose: bool) {
    let filter = if verbose {
        EnvFilter::new("ruslink=debug")
    } else {
        EnvFilter::from_default_env()
            .add_directive("ruslink=info".parse().unwrap())
    };

    fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stdout)
        .init();
}