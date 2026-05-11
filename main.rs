// rustow.rs - Rust Stow with .gitignore + .rustow.ignore support
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::process;

#[derive(Debug)]
struct Config {
    package: String,
    stow_dir: PathBuf,
    target_dir: PathBuf,
    delete: bool,
    restow: bool,
    dry_run: bool,
    verbose: bool,
}

fn main() {
    let config = parse_args();
    let package_path = config.stow_dir.join(&config.package);

    if !package_path.exists() {
        eprintln!("Error: Package '{}' not found in {:?}", config.package, config.stow_dir);
        process::exit(1);
    }

    println!("Package: {}", config.package);
    println!("Stow dir: {:?}", config.stow_dir);
    println!("Target dir: {:?}", config.target_dir);

    if config.dry_run {
        println!("*** DRY RUN MODE ***");
    }

    let ignore_regexes = load_all_ignore_patterns(&package_path);

    if config.restow || config.delete {
        println!("Unstowing package '{}'...", config.package);
        let _ = unstow_package(&package_path, &config.target_dir, &config, &ignore_regexes);
    }

    if !config.delete {
        println!("Stowing package '{}'...", config.package);
        let _ = stow_package(&package_path, &config.target_dir, &config, &ignore_regexes);
    }

    if config.dry_run {
        println!("\nDry run completed. No changes were made.");
    } else {
        println!("\nDone!");
    }
}

fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();
    let mut stow_dir = env::current_dir().unwrap();
    let mut target_dir = stow_dir.parent().map(PathBuf::from).unwrap_or_else(|| PathBuf::from("/"));
    let mut delete = false;
    let mut restow = false;
    let mut dry_run = false;
    let mut verbose = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-d" | "--dir" => { if i + 1 < args.len() { stow_dir = PathBuf::from(&args[i + 1]); i += 1; } }
            "-t" | "--target" => { if i + 1 < args.len() { target_dir = PathBuf::from(&args[i + 1]); i += 1; } }
            "--delete" | "-D" => delete = true,
            "--restow" | "-R" => restow = true,
            "--dry-run" | "-n" => dry_run = true,
            "--verbose" | "-v" => verbose = true,
            _ => {}
        }
        i += 1;
    }

    let package = args.get(1).cloned().unwrap_or_else(|| {
        eprintln!("Usage: {} <package> [options]", args[0]);
        eprintln!("Options: --delete, --restow, --dry-run, --verbose, -d <dir>, -t <target>");
        process::exit(1);
    });

    Config { package, stow_dir, target_dir, delete, restow, dry_run, verbose }
}

// ====================== IGNORE SYSTEM (.gitignore + .rustow.ignore) ======================

fn load_all_ignore_patterns(package_path: &Path) -> Vec<regex::Regex> {
    let mut regexes = Vec::new();

    // Default ignores
    for pat in [
        r"^\.git$",
        r"^\.gitmodules$",
        r"^\.gitignore$",
        r"^\.rustow\.ignore$",
        r"^README.*$",
        r"^LICENSE.*$",
        r"^COPYING.*$",
        r".*\.bak$",
        r".*\.tmp$",
        r"^\.DS_Store$",
    ] {
        if let Ok(re) = regex::Regex::new(pat) {
            regexes.push(re);
        }
    }

    // Load .gitignore
    if let Some(gitignore_patterns) = load_gitignore(package_path) {
        regexes.extend(gitignore_patterns);
    }

    // Load .rustow.ignore (higher priority)
    if let Some(rustow_patterns) = load_rustow_ignore(package_path) {
        regexes.extend(rustow_patterns);
    }

    regexes
}

fn load_gitignore(base: &Path) -> Option<Vec<regex::Regex>> {
    let gitignore_path = base.join(".gitignore");
    if !gitignore_path.exists() {
        return None;
    }

    let mut regexes = Vec::new();
    if let Ok(file) = fs::File::open(gitignore_path) {
        for line in io::BufReader::new(file).lines().flatten() {
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

fn load_rustow_ignore(base: &Path) -> Option<Vec<regex::Regex>> {
    let path = base.join(".rustow.ignore");
    if !path.exists() {
        return None;
    }

    let mut regexes = Vec::new();
    if let Ok(file) = fs::File::open(path) {
        for line in io::BufReader::new(file).lines().flatten() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let pattern = if line.starts_with('^') || line.contains(".*") {
                line.to_string()
            } else {
                glob_to_regex(line)
            };
            if let Ok(re) = regex::Regex::new(&pattern) {
                regexes.push(re);
            }
        }
    }
    Some(regexes)
}

fn git_pattern_to_regex(pattern: &str) -> Option<regex::Regex> {
    let mut p = pattern.trim_start_matches('/'); // remove leading slash

    if p.ends_with('/') {
        p = &p[..p.len() - 1];
    }

    let re_str = p
        .replace('.', r"\.")
        .replace('*', ".*")
        .replace('?', ".");

    let final_re = if pattern.starts_with('/') || pattern.starts_with("**/") {
        format!("^.*{}.*", re_str)
    } else {
        format!("^.*{}$|.*{}/.*", re_str, re_str)
    };

    regex::Regex::new(&final_re).ok()
}

fn glob_to_regex(pattern: &str) -> String {
    let mut re = pattern
        .replace('.', r"\.")
        .replace('*', ".*")
        .replace('?', ".");

    if pattern.ends_with('/') {
        format!("^{}.*", re.trim_end_matches('/'))
    } else if !pattern.starts_with('^') {
        format!("^.*{}$", re)
    } else {
        re
    }
}

fn should_ignore(path: &Path, regexes: &[regex::Regex]) -> bool {
    let path_str = path.to_string_lossy().replace('\\', "/");
    let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

    for re in regexes {
        if re.is_match(&path_str) || re.is_match(file_name) {
            return true;
        }
    }
    false
}

// ====================== STOW / UNSTOW ======================

fn stow_package(
    source: &Path,
    target: &Path,
    config: &Config,
    ignore_regexes: &[regex::Regex],
) -> io::Result<()> {
    if !target.exists() && !config.dry_run {
        fs::create_dir_all(target)?;
    }

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let entry_path = entry.path();

        if should_ignore(&entry_path, ignore_regexes) {
            if config.verbose {
                println!("  Ignored: {:?}", entry_path);
            }
            continue;
        }

        let target_path = target.join(entry.file_name());

        if entry_path.is_dir() {
            if !target_path.exists() && !config.dry_run {
                fs::create_dir_all(&target_path)?;
            }
            stow_package(&entry_path, &target_path, config, ignore_regexes)?;
        } else {
            handle_file_stow(&entry_path, &target_path, config)?;
        }
    }
    Ok(())
}

fn handle_file_stow(src: &Path, dst: &Path, config: &Config) -> io::Result<()> {
    if dst.exists() {
        if dst.is_symlink() {
            if !config.dry_run {
                fs::remove_file(dst)?;
            }
            println!("  Replaced: {:?}", dst);
        } else {
            eprintln!("  Conflict! Non-symlink exists: {:?}", dst);
            return Ok(());
        }
    } else {
        println!("  Linked: {:?}", dst);
    }

    if !config.dry_run {
        let relative = make_relative(src, dst);
        #[cfg(unix)]
        std::os::unix::fs::symlink(&relative, dst)?;
        #[cfg(windows)]
        std::os::windows::fs::symlink_file(src, dst)?;
    }
    Ok(())
}

fn unstow_package(
    source: &Path,
    target: &Path,
    config: &Config,
    ignore_regexes: &[regex::Regex],
) -> io::Result<()> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let entry_path = entry.path();

        if should_ignore(&entry_path, ignore_regexes) {
            continue;
        }

        let target_path = target.join(entry.file_name());

        if entry_path.is_dir() {
            unstow_package(&entry_path, &target_path, config, ignore_regexes)?;
            if target_path.exists() && !config.dry_run {
                let _ = fs::remove_dir(&target_path);
            }
        } else if target_path.is_symlink() {
            if !config.dry_run {
                fs::remove_file(&target_path)?;
            }
            println!("  Removed: {:?}", target_path);
        }
    }
    Ok(())
}

fn make_relative(src: &Path, _dst: &Path) -> PathBuf {
    src.canonicalize().unwrap_or_else(|_| src.to_path_buf())
}
