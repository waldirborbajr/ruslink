use std::io;
use std::path::Path;

use crate::config::Config;

pub fn stow_package(
    source: &Path,
    target: &Path,
    config: &Config,
    ignores: &[regex::Regex],
) -> io::Result<()> {
    // TODO: Implement stow package functionality
    unimplemented!("Stow package implementation needed")
}

pub fn unstow_package(
    source: &Path,
    target: &Path,
    config: &Config,
    ignores: &[regex::Regex],
) -> io::Result<()> {
    // TODO: Implement unstow package functionality
    unimplemented!("Unstow package implementation needed")
}
