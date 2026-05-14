// // src/stow/merge.rs

// use std::fs;
// use std::path::Path;
// use anyhow::Result;

// pub struct MergeHandler {
//     track_file: PathBuf,
// }

// impl MergeHandler {
//     pub fn new(package_path: &Path) -> Self {
//         Self {
//             track_file: package_path.parent().unwrap().join(".ruslink-merge-log"),
//         }
//     }

//     /// Resolve conflito inteligentemente
//     pub fn resolve_conflict(
//         &self,
//         destination: &Path,
//         source: &Path,
//         config: &MergeConfig,
//     ) -> Result<MergeAction> {
//         if !destination.exists() {
//             return Ok(MergeAction::CreateLink);
//         }

//         // Verificar se é extensão appendable
//         if let Some(ext) = destination.extension().and_then(|e| e.to_str()) {
//             if config.append_extensions.iter().any(|e| e.trim_start_matches('.') == ext) {
//                 return Ok(MergeAction::AppendContent);
//             }
//         }

//         // Verificar se é diretório que pode ser merged
//         if destination.is_dir() && source.is_dir() {
//             return Ok(MergeAction::MergeDirectories);
//         }

//         // Conflito verdadeiro
//         Ok(MergeAction::Conflict)
//     }

//     /// Implementar append de conteúdo (para .bashrc, .zshrc, etc)
//     pub fn append_content(
//         &self,
//         destination: &Path,
//         source: &Path,
//         package_name: &str,
//     ) -> Result<()> {
//         let source_content = fs::read_to_string(source)?;
//         let mut dest_content = fs::read_to_string(destination).unwrap_or_default();

//         // Verificar se já foi appendido
//         let marker = format!("# === ruslink {} ===", package_name);
//         if dest_content.contains(&marker) {
//             info!("Content from {} already merged", package_name);
//             return Ok(());
//         }

//         // Fazer append com marcador
//         dest_content.push_str(&format!("\n\n{}\n", marker));
//         dest_content.push_str(&source_content);
//         dest_content.push_str(&format!("\n# === end ruslink {} ===\n", package_name));

//         fs::write(destination, dest_content)?;
//         self.log_merge(destination, package_name)?;

//         info!("Merged content from {} into {:?}", package_name, destination);
//         Ok(())
//     }

//     /// Log de merges para tracking
//     fn log_merge(&self, file: &Path, package: &str) -> Result<()> {
//         let entry = format!("{}: {}\n", chrono::Local::now(), file.display());
//         let mut log = fs::read_to_string(&self.track_file).unwrap_or_default();
//         log.push_str(&entry);
//         fs::write(&self.track_file, log)?;
//         Ok(())
//     }
// }

// #[derive(Debug)]
// pub enum MergeAction {
//     CreateLink,
//     AppendContent,
//     MergeDirectories,
//     Conflict,
// }

// src/stow/merge.rs
// Implementação do Merge Mode - Feature indispensável para ruslink

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// Ações possíveis quando há conflito entre packages
#[derive(Debug, Clone, Copy)]
pub enum MergeAction {
    /// Criar symlink normalmente
    CreateLink,
    /// Fazer append de conteúdo (para .bashrc, .zshrc, etc)
    AppendContent,
    /// Mesclar diretórios (continuar stowing dentro)
    MergeDirectories,
    /// Conflito não resolvível
    Conflict,
}

/// Configuração de merge para suportar múltiplos packages
#[derive(Debug, Clone)]
pub struct MergeConfig {
    /// Ativar merge mode
    pub enabled: bool,

    /// Extensões que devem ser appendidas (ex: [".bashrc", ".zshrc"])
    pub append_extensions: Vec<String>,

    /// Se true, cria .ruslink-merge-log para tracking de merges
    pub track_merges: bool,
}

impl Default for MergeConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            append_extensions: vec![
                ".bashrc".to_string(),
                ".bash_profile".to_string(),
                ".zshrc".to_string(),
                ".profile".to_string(),
                ".fishrc".to_string(),
            ],
            track_merges: true,
        }
    }
}

/// Gerenciador de merges entre packages
pub struct MergeHandler {
    /// Arquivo de log dos merges realizados
    track_file: PathBuf,
    /// Package que está sendo stowed
    package_name: String,
}

impl MergeHandler {
    pub fn new(package_path: &Path, package_name: String) -> Self {
        let track_file =
            package_path.parent().unwrap_or_else(|| Path::new(".")).join(".ruslink-merge-log");

        Self { track_file, package_name }
    }

    /// Resolve conflito entre arquivo de source e destination
    ///
    /// Retorna MergeAction indicando como proceder
    pub fn resolve_conflict(
        &self,
        destination: &Path,
        source: &Path,
        config: &MergeConfig,
    ) -> Result<MergeAction> {
        if !destination.exists() && destination.symlink_metadata().is_err() {
            debug!("No conflict: destination {:?} doesn't exist", destination);
            return Ok(MergeAction::CreateLink);
        }

        // Se destination é symlink apontando para source, OK
if let Ok(link) = fs::read_link(destination) {
    let same_path = link == source;

    let same_canonical = match (link.canonicalize(), source.canonicalize()) {
        (Ok(link_path), Ok(source_path)) => link_path == source_path,
        _ => false,
    };

    if same_path || same_canonical {
        debug!("Symlink already correct: {:?}", destination);
        return Ok(MergeAction::CreateLink);
    }
}

        // Verificar se é uma extensão que pode ser appendida
        if self.should_append(destination, config) {
            debug!("File eligible for append: {:?}", destination);
            return Ok(MergeAction::AppendContent);
        }

        // Se ambos são diretórios, pode fazer merge
        if destination.is_dir() && source.is_dir() {
            debug!("Both are directories, can merge recursively: {:?}", destination);
            return Ok(MergeAction::MergeDirectories);
        }

        // Conflito: arquivo regular conflitando com arquivo regular ou diretório
        warn!("Conflict detected at {:?}: file exists and is not mergeable", destination);
        Ok(MergeAction::Conflict)
    }

    /// Verifica se arquivo deve ser appendido
    fn should_append(&self, destination: &Path, config: &MergeConfig) -> bool {
        // Verificar por nome exato do arquivo
        if let Some(file_name) = destination.file_name().and_then(|n| n.to_str()) {
            if config
                .append_extensions
                .iter()
                .any(|ext| file_name == ext.trim_start_matches('.') || file_name == ext)
            {
                return true;
            }
        }

        // Verificar por extensão
        if let Some(ext) = destination.extension().and_then(|e| e.to_str()) {
            return config.append_extensions.iter().any(|e| e.trim_start_matches('.') == ext);
        }

        false
    }

    /// Fazer append de conteúdo com marcadores
    ///
    /// Adiciona conteúdo de source no final de destination com marcadores
    /// de qual package fez o merge para rastrear originalmente
    pub fn append_content(&self, destination: &Path, source: &Path) -> Result<()> {
        let source_content = match fs::read_to_string(source) {
            Ok(content) => content,
            Err(e) => anyhow::bail!("Failed to read source file {:?}: {}", source, e),
        };

        let mut dest_content = fs::read_to_string(destination).unwrap_or_default();

        // Marcadores para tracking
        let start_marker = format!("# === ruslink [{}] ===", self.package_name);
        let end_marker = format!("# === ruslink [{}] (end) ===", self.package_name);

        // Verificar se já foi appendido (evitar duplicatas)
        if dest_content.contains(&start_marker) {
            debug!(
                "Content from {} already merged in {:?}, skipping",
                self.package_name, destination
            );
            return Ok(());
        }

        // Fazer append com marcadores
        if !dest_content.ends_with('\n') && !dest_content.is_empty() {
            dest_content.push('\n');
        }

        dest_content.push('\n');
        dest_content.push_str(&start_marker);
        dest_content.push('\n');
        dest_content.push_str(&source_content);
        if !source_content.ends_with('\n') {
            dest_content.push('\n');
        }
        dest_content.push_str(&end_marker);
        dest_content.push('\n');

        // Escrever arquivo merged
        fs::write(destination, &dest_content)
            .map_err(|e| anyhow::anyhow!("Failed to write merged file {:?}: {}", destination, e))?;

        // Logar merge se ativado
        self.log_merge(destination)?;

        info!("✓ Merged content from {} into {:?}", self.package_name, destination);

        Ok(())
    }

    /// Log de merges para auditoria
    fn log_merge(&self, file: &Path) -> Result<()> {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let entry =
            format!("[{}] Package: {} | File: {}\n", timestamp, self.package_name, file.display());

        let mut log = fs::read_to_string(&self.track_file).unwrap_or_default();
        log.push_str(&entry);

        fs::write(&self.track_file, log)
            .map_err(|e| anyhow::anyhow!("Failed to write merge log: {}", e))?;

        debug!("Logged merge: {}", entry.trim());
        Ok(())
    }

    /// Exibir histórico de merges
    pub fn show_merge_history(&self) -> Result<()> {
        match fs::read_to_string(&self.track_file) {
            Ok(log) => {
                println!("\n📋 Merge History ({}):", self.track_file.display());
                println!("{}", log);
                Ok(())
            },
            Err(e) => {
                warn!("No merge history found: {}", e);
                Ok(())
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_should_append_by_name() {
        let temp = TempDir::new().unwrap();
        let handler = MergeHandler::new(temp.path(), "test".to_string());
        let config = MergeConfig::default();

        let bashrc = temp.path().join(".bashrc");
        assert!(handler.should_append(&bashrc, &config));
    }

    #[test]
    fn test_should_append_by_extension() {
        let temp = TempDir::new().unwrap();
        let handler = MergeHandler::new(temp.path(), "test".to_string());
        let config = MergeConfig::default();

        let file = temp.path().join("config.zshrc");
        assert!(handler.should_append(&file, &config));
    }

    #[test]
    fn test_append_content_basic() -> Result<()> {
        let temp = TempDir::new().unwrap();
        let handler = MergeHandler::new(temp.path(), "dev".to_string());

        // Criar arquivo base
        let dest = temp.path().join("test.bashrc");
        fs::write(&dest, "# Base config\necho 'base'")?;

        // Criar arquivo source
        let src = temp.path().join("source.bashrc");
        fs::write(&src, "echo 'dev addon'")?;

        // Fazer merge
        handler.append_content(&dest, &src)?;

        // Verificar resultado
        let content = fs::read_to_string(&dest)?;
        assert!(content.contains("Base config"));
        assert!(content.contains("dev addon"));
        assert!(content.contains("# === ruslink [dev] ==="));

        Ok(())
    }

    #[test]
    fn test_no_duplicate_merge() -> Result<()> {
        let temp = TempDir::new().unwrap();
        let handler = MergeHandler::new(temp.path(), "dev".to_string());

        let dest = temp.path().join("test.bashrc");
        fs::write(&dest, "# Base\necho 'base'")?;

        let src = temp.path().join("source.bashrc");
        fs::write(&src, "echo 'dev'")?;

        // Primeiro merge
        handler.append_content(&dest, &src)?;
        let content1 = fs::read_to_string(&dest)?;

        // Segundo merge (não deve duplicar)
        handler.append_content(&dest, &src)?;
        let content2 = fs::read_to_string(&dest)?;

        // Devem ser idênticos (sem duplicatas)
        assert_eq!(content1, content2);

        Ok(())
    }
}
