// src/git/mod.rs
mod gitmanager;
mod operations; // was: mod git;

pub use operations::handle_git_operations;
