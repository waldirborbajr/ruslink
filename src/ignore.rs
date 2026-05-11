use std::fs;
use std::io::BufRead;
use std::path::Path;

pub fn load_all_ignore_patterns(package_path: &Path) -> Vec<regex::Regex> {
    let mut regexes = Vec::new();

    for pat in [
        r"^\.git$", r"^\.gitmodules$", r"^\.gitignore$", r"^\.ruslink\.ignore$",
        r"^README.*$", r"^LICENSE.*$", r"^COPYING.*$", r".*\.bak$", r".*\.tmp$", r"^\.DS_Store$",
    ] {
        if let Ok(re) = regex::Regex::new(pat) { regexes.push(re); }
    }

    if let Some(git) = load_gitignore(package_path) { regexes.extend(git); }
    if let Some(ruslink) = load_ruslink_ignore(package_path) { regexes.extend(ruslink); }

    regexes
}

pub fn load_gitignore(base: &Path) -> Option<Vec<regex::Regex>> {
    let path = base.join(".gitignore");
    if !path.exists() { return None; }
    
    let mut regexes = Vec::new();
    if let Ok(file) = fs::File::open(path) {
        for line in std::io::BufReader::new(file).lines().flatten() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            if let Some(re) = git_pattern_to_regex(line) {
                regexes.push(re);
            }
        }
    }
    Some(regexes)
}

pub fn load_ruslink_ignore(base: &Path) -> Option<Vec<regex::Regex>> {
    let path = base.join(".ruslink.ignore");
    if !path.exists() { return None; }
    
    let mut regexes = Vec::new();
    if let Ok(file) = fs::File::open(path) {
        for line in std::io::BufReader::new(file).lines().flatten() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            let pat = if line.starts_with('^') { line.to_string() } else { glob_to_regex(line) };
            if let Ok(re) = regex::Regex::new(&pat) { regexes.push(re); }
        }
    }
    Some(regexes)
}

pub fn git_pattern_to_regex(pattern: &str) -> Option<regex::Regex> {
    let p = pattern.trim_start_matches('/').trim_end_matches('/');
    let re_str = p.replace('.', r"\.").replace('*', ".*").replace('?', ".");
    let final_re = format!(r"(^|/).*{}(/|$)", re_str);
    regex::Regex::new(&final_re).ok()
}

pub fn glob_to_regex(p: &str) -> String {
    let re = p.replace('.', r"\.").replace('*', ".*").replace('?', ".");
    format!("^.*{}$", re)
}

pub fn should_ignore(path: &Path, regexes: &[regex::Regex]) -> bool {
    let path_str = path.to_string_lossy().replace('\\', "/");
    regexes.iter().any(|re| re.is_match(&path_str))
}
