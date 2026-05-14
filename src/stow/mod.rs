// src/stow/mod.rs
mod merge; // NOVO
mod stow;

pub use merge::MergeConfig; // NOVO: Exportar para uso em cli/
pub use stow::{stow_package, unstow_package, StowStats};
