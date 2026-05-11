// src/ignore.rs
use std::path::Path;
use std::sync::OnceLock;

use regex::Regex;

static GLOBAL_IGNORE_PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();

/// Carrega todos os padrões de ignore com cache global
pub fn load_all_ignore_patterns(package_path: &Path) -> Vec<Regex> {
    // Padrões globais (sempre os mesmos)
    let global_patterns = GLOBAL_IGNORE_PATTERNS.get_or_init(|| {
        let mut regexes = Vec::new();

        // Padrões padrão
        for pat in [
            r"^\.git$",
            r"^\.gitmodules$",
            r"^\.gitignore$",
            r"^\.ruslink\.ignore$",
            r"^README.*$",
            r"^LICENSE.*$",
            r"^COPYING.*$",
            r".*\.bak$",
            r".*\.tmp$",
            r"^\.DS_Store$",
        ] {
            if let Ok(re) = Regex::new(pat) {
                regexes.push(re);
            }
        }

        if let Some(git) = load_gitignore(package_path) {
            regexes.extend(git);
        }
        if let Some(ruslink) = load_ruslink_ignore(package_path) {
            regexes.extend(ruslink);
        }

        regexes
    });

    global_patterns.clone()
}

fn load_gitignore(base: &Path) -> Option<Vec<Regex>> {
    let path = base.join(".gitignore");
    if !path.exists() {
        return None;
    }

    let mut regexes = Vec::new();
    if let Ok(file) = std::fs::File::open(path) {
        for line in std::io::BufRead::lines(std::io::BufReader::new(file)).flatten() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(re) = git_pattern_to_regex(line) {
                regexes.push(re);
            }
        }
    }
    Some(regexes)
}

fn load_ruslink_ignore(base: &Path) -> Option<Vec<Regex>> {
    let path = base.join(".ruslink.ignore");
    if !path.exists() {
        return None;
    }

    let mut regexes = Vec::new();
    if let Ok(file) = std::fs::File::open(path) {
        for line in std::io::BufRead::lines(std::io::BufReader::new(file)).flatten() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let pattern = if line.starts_with('^') || line.contains(".*") {
                line.to_string()
            } else {
                glob_to_regex(line)
            };
            if let Ok(re) = Regex::new(&pattern) {
                regexes.push(re);
            }
        }
    }
    Some(regexes)
}

fn git_pattern_to_regex(pattern: &str) -> Option<Regex> {
    let p = pattern.trim_start_matches('/').trim_end_matches('/');
    let re_str = p.replace('.', r"\.").replace('*', ".*").replace('?', ".");
    let final_re = format!(r"(^|/).*{}(/|$)", re_str);
    Regex::new(&final_re).ok()
}

fn glob_to_regex(p: &str) -> String {
    let re = p.replace('.', r"\.").replace('*', ".*").replace('?', ".");
    format!("^.*{}$", re)
}

/// Verifica se um caminho deve ser ignorado
pub fn should_ignore(path: &Path, regexes: &[Regex]) -> bool {
    let path_str = path.to_string_lossy().replace('\\', "/");
    let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

    regexes.iter().any(|re| re.is_match(&path_str) || re.is_match(file_name))
}
