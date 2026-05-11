use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub package: String,
    pub stow_dir: PathBuf,
    pub target_dir: PathBuf,
    pub delete: bool,
    pub restow: bool,
    pub dry_run: bool,
    pub verbose: bool,
    pub auto_git: bool,
    pub git_push: bool,
    pub force: bool,
    pub backup: bool,
    pub adopt: bool,
    pub commit_message: Option<String>,
}
