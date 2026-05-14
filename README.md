# ruslink

<p align="center">
  <img width="256" height="256" src="./.assets/logo.png" />
</p>

[![CodeQL](https://github.com/waldirborbajr/ruslink/actions/workflows/github-code-scanning/codeql/badge.svg)](https://github.com/waldirborbajr/ruslink/actions/workflows/github-code-scanning/codeql)
[![Publish to crates.io](https://github.com/waldirborbajr/ruslink/actions/workflows/crates.yaml/badge.svg)](https://github.com/waldirborbajr/ruslink/actions/workflows/crates.yaml)
[![Release](https://github.com/waldirborbajr/ruslink/actions/workflows/release.yaml/badge.svg)](https://github.com/waldirborbajr/ruslink/actions/workflows/release.yaml)
[![Rust CI](https://github.com/waldirborbajr/ruslink/actions/workflows/ci.yaml/badge.svg)](https://github.com/waldirborbajr/ruslink/actions/workflows/ci.yaml)


`ruslink` is a lightweight Rust-based stow utility for managing dotfiles and package-style deployments with support for ignore patterns, dry-run mode, auto git commit, force overwrite, backup, and friendly error messages.

## Features

- Stows a package directory into a target location using symlinks
- Supports uninstalling packages and restowing
- Respects `.gitignore` and `.ruslink.ignore` patterns
- Auto commit changes in a git repository with optional push
- Force overwrite existing destination files
- Backup existing files before overwriting/removing them
- Adopt mode to replace existing files with symlinks
- Dry-run mode for safe previews
- Structured logging with tracing (use `-v` flag for debug logs)
- Human-friendly error messages on crashes

## Installation

Build with Cargo:

```bash
cargo build --release
```

Build with just

```bash
=== Development ===
 just / just help            → show this help
 just build / b              → watch + build (default features)
 just run / r                → watch + run

=== Feature Builds ===
 just build-minimal          → Build without git and colors
 just build-no-git           → Build with colors but no git
 just build-no-colors        → Build with git but no colors
 just build-release          → Release build (default features)
 just build-release-minimal  → Smallest possible binary

=== Quality ===
 just lint                   → fmt + fmt --check + clippy
 just test                   → run tests

=== Maintenance ===
 just release                → build release + install locally
 just update                 → update deps + clear cache
 just cache                  → remove cargo cache
 just clean                  → cargo clean
 just size                   → show binary sizes
```

The binary will be available at `target/release/ruslink`.

## Usage

```bash
ruslink <package> [OPTIONS]
```

### Example

```bash
ruslink home --dir ~/.dotfiles --target ~
```

This command will stow the `home` package from `~/.dotfiles/home` into the home directory.

## Options

- `-d, --dir <DIR>`: Stow directory (default: current working directory)
- `-t, --target <DIR>`: Target directory (default: parent of `--dir`)
- `-D, --delete`: Only unstow the package
- `-R, --restow`: Unstow then stow the package
- `-n, --dry-run`: Simulate actions without making changes
- `-v, --verbose`: Enable verbose (debug) output with structured logging
- `-g, --git`: Auto commit changes in the package git repository
- `--git-push`: Push changes to git remote after commit
- `--force`: Overwrite existing destination files
- `--backup`: Backup existing files before modifying them
- `--adopt`: Adopt existing files (replace with symlink, no backup)
- `-m, --message <MSG>`: Custom git commit message

## Logging

Use the `-v` or `--verbose` flag to enable debug logging:

```bash
ruslink home --dir ~/.dotfiles --target ~ -v

# Teste seguro com dry-run
ruslink nvim --dir ~/.dotfiles --target ~ --dry-run -v

# Se tudo parecer certo, execute de verdade
ruslink nvim --dir ~/.dotfiles --target ~

# Com git auto-commit
ruslink nvim --dir ~/.dotfiles --target ~ --git --message "Setup nvim config"

# Setup básico
ruslink base --target ~

# Adicionar dev tools (alguns arquivos conflitam)
ruslink dev --target ~ --merge --merge-append \
  --merge-extensions=".bashrc,.zshrc,.config/fish/config.fish"

# Resultado:
# ~/.bashrc contém:
#   - Conteúdo de base/.bashrc
#   - # === ruslink dev ===
#   - Conteúdo de dev/.bashrc (appendido)
#   - # === end ruslink dev ===
```

This will show detailed information about what ruslink is doing internally, including:
- Configuration details
- Symlink creation operations
- File adoption/backup operations
- Git operations

## Error Handling

`ruslink` provides human-friendly error messages on crashes and panics. If something goes wrong, you'll see a clear, actionable error message instead of a cryptic panic traceback.

## Ignore files

`ruslink` loads ignore patterns from:

- `.gitignore`
- `.ruslink.ignore`

The ignore loader also skips common metadata files such as `.git`, `.gitmodules`, `README*`, `LICENSE*`, `COPYING*`, `.DS_Store`, and temporary backups.

## Notes

- When `--dry-run` is enabled, git auto-commit is disabled automatically.
- `--force` allows destination files to be replaced when they already exist.
- `--backup` renames an existing path with a `.bak` suffix before removal or overwrite.

## Project structure

- `src/main.rs`: application entry point
- `src/config.rs`: configuration struct
- `src/args.rs`: command-line parsing with Clap
- `src/git.rs`: git auto-commit helper
- `src/ignore.rs`: ignore pattern loading and matching
- `src/stow.rs`: stow/unstow implementation

## License

This project is released under the MIT License - see the [LICENSE](LICENSE) file for details.

