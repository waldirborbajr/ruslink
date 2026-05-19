# ruslink

<p align="center">
  <img width="256" height="256" src="./.assets/logo.png" />
</p>

[![CodeQL](https://github.com/waldirborbajr/ruslink/actions/workflows/github-code-scanning/codeql/badge.svg)](https://github.com/waldirborbajr/ruslink/actions/workflows/github-code-scanning/codeql)
[![Publish to crates.io](https://github.com/waldirborbajr/ruslink/actions/workflows/crates.yaml/badge.svg)](https://github.com/waldirborbajr/ruslink/actions/workflows/crates.yaml)
[![Release](https://github.com/waldirborbajr/ruslink/actions/workflows/release.yaml/badge.svg)](https://github.com/waldirborbajr/ruslink/actions/workflows/release.yaml)
[![Rust CI](https://github.com/waldirborbajr/ruslink/actions/workflows/rust-ci.yaml/badge.svg)](https://github.com/waldirborbajr/ruslink/actions/workflows/rust-ci.yaml)
[![Security Audit](https://github.com/waldirborbajr/ruslink/actions/workflows/security-audit.yaml/badge.svg)](https://github.com/waldirborbajr/ruslink/actions/workflows/security-audit.yaml)

**A fast, modern Rust reimplementation of GNU Stow** with powerful features like intelligent merging, native Git integration, intelligent conflict resolution and excellent UX.

---

## Why ruslink?

### Comprehensive Feature Comparison

| Feature | GNU Stow | Chezmoi | **ruslink** |
|---------|----------|---------|------------|
| **Language & Performance** | Perl (slow) | Go (fast) | **Rust** (fastest, smallest) |
| **Merge Mode** | ❌ Not supported | ⚠️ Limited | ✅ **Intelligent append + markers** |
| **Git Integration** | ❌ None | ✅ Yes | ✅ **Native auto-commit & push** |
| **Dotfiles Mode** | ❌ Manual / hacky | ✅ Yes | ✅ **Built-in & seamless** |
| **Conflict Handling** | ⚠️ Fail only / `--adopt` | ✅ Multiple strategies | ✅ **Force, Adopt, Backup, Merge** |
| **Dry Run & Safety** | ✅ Basic | ✅ Yes | ✅ **Excellent with detailed logs** |
| **User Experience** | ❌ Outdated | ✅ Modern | ✅ **Modern, colorful, clear messages** |
| **Windows Support** | ⚠️ Poor (WSL mainly) | ✅ Native | ✅ **Native symlinks support** |
| **Binary Size** | Perl dependency | ~10-15 MB | **~3–8 MB** (static) |
| **Installation** | System package manager | Go required | **Single static binary** |
| **Learning Curve** | Moderate | Steep | ⭐ **Minimal - just like Stow** |
| **Active Development** | ⚠️ Minimal | ✅ Active | ✅ **Active development** |
| **Built-in Commands** | `stow` / `unstow` | `init` / `add` / `apply` | **stow, unstow, list, status, clean** |
| **Configuration File** | ❌ No | ✅ TOML/YAML | ⏳ Planned |
| **Hooks/Scripts** | ⚠️ Limited | ✅ Advanced | ⏳ Planned |

**Key Takeaway:** 
- **Choose GNU Stow** if you want simplicity and minimal dependencies
- **Choose Chezmoi** if you need advanced templating and encryption
- **Choose ruslink** if you want Stow's simplicity + modern features (merge, git, dotfiles) + blazing speed

---

## Features

### Core Functionality ✅
- Symlink-based package management (just like GNU Stow)
- Full support for `.gitignore` and `.ruslink.ignore`
- Safe operations with `--dry-run` previews
- Dotfiles mode: `dot-bashrc` → `.bashrc`, `dot-config/nvim` → `.config/nvim`
- Conflict resolution: `--force`, `--backup`, `--adopt`, `--merge`
- Package introspection: `list` and `status` commands
- Clean up broken symlinks and empty directories

### Advanced Features 🚀
- **Merge Mode** — intelligently merge shell configs (`.bashrc`, `.zshrc`, `.fish/config.fish`, etc.)
- **Git Integration** — automatic commit and optional push after stowing
- **Interactive Confirmations** — user prompts for destructive actions
- **Structured Logging** — debug-level insights with `-v` flag
- **Human-Friendly Errors** — clear, actionable error messages
- **Optional Colors** — beautiful output with conditional color support
- **Feature Flags** — build minimal binaries for constrained environments

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Complete Command Reference](#complete-command-reference)
- [Usage Guide](#usage-guide)
  - [Basic Operations](#basic-operations)
  - [Merge Mode](#merge-mode)
  - [Git Integration](#git-integration)
  - [Conflict Resolution](#conflict-resolution)
  - [Dry Run & Safety](#dry-run--safety)
- [Configuration](#configuration)
- [Error Handling & Logging](#error-handling--logging)
- [Project Structure](#project-structure)
- [Build Options](#build-options)
- [Examples](#examples)
- [License](#license)

## Installation

### Build with Cargo

```bash
cargo build --release
```

The binary will be available at `target/release/ruslink`.

### Build with just

We provide a `justfile` for convenient development and building:

```bash
# Show all available commands
just help

# Development
just build          # Watch + build (default features)
just run            # Watch + run

# Feature Builds
just build-minimal          # Build without git and colors
just build-no-git           # Build with colors but no git
just build-no-colors        # Build with git but no colors
just build-release          # Release build (all features)
just build-release-minimal  # Smallest possible binary

# Quality Assurance
just lint           # fmt + fmt --check + clippy
just test           # Run tests

# Maintenance
just release        # Build release + install locally
just update         # Update deps + clear cache
just clean          # Cargo clean
just size           # Show binary sizes
```

## Quick Start

### Basic Setup

```bash
# List available packages
ruslink list --dir ~/.dotfiles

# Preview stowing
ruslink home --dir ~/.dotfiles --target ~ --dry-run -v

# Stow a package
ruslink home --dir ~/.dotfiles --target ~

# Unstow a package
ruslink home --delete --dir ~/.dotfiles --target ~

# Restow (unstow then stow)
ruslink home --restow --dir ~/.dotfiles --target ~

# Show package status
ruslink status home --dir ~/.dotfiles --target ~
```

### With Git Integration

```bash
# Auto-commit changes after stowing
ruslink nvim --dir ~/.dotfiles --target ~ --git

# With custom commit message
ruslink nvim --dir ~/.dotfiles --target ~ --git --message "Setup nvim config"

# Push to remote after commit
ruslink nvim --dir ~/.dotfiles --target ~ --git --git-push
```

### Merge Mode (Multiple Packages)

```bash
# Apply base package
ruslink base --target ~

# Add developer tools with intelligent merge
ruslink dev --target ~ --merge --merge-append \
  --merge-extensions ".bashrc,.zshrc,.config/fish/config.fish"

# Add GUI configuration
ruslink gui --target ~ --merge --merge-append

# View merge history
ruslink gui --target ~ --show-merge-history
```

### Dotfiles Mode

```bash
# Transform dot-prefixed files
ruslink bash --dotfiles
# dot-bashrc → .bashrc, dot-config/nvim → .config/nvim

# Combine with merge mode
ruslink nvim --dotfiles --merge --merge-append
```

---

## Complete Command Reference

### Positional Arguments

```
PACKAGE_NAME (optional)  Package to stow (required for stow/unstow/status)
                         Omit for list/clean/status
```

### Essential Options

| Flag | Short | Type | Description |
|------|-------|------|-------------|
| `--dir` | `-d` | `PATH` | Stow directory (default: current directory) |
| `--target` | `-t` | `PATH` | Target directory (default: parent of stow dir) |
| `--dry-run` | `-n` | bool | Preview changes without applying them |
| `--verbose` | `-v` | bool | Enable debug output (structured logging) |
| `--yes` | `-y` | bool | Auto-confirm all prompts |

### Commands

| Flag | Short | Type | Description |
|------|-------|------|-------------|
| `--list` | - | bool | List all available packages |
| `--status` | - | bool | Show detailed status of packages/links |
| `--clean` | - | bool | Remove broken symlinks and empty directories |

### Stow Operations

| Flag | Short | Type | Description |
|------|-------|------|-------------|
| `--delete` | `-D` | bool | Delete/unstow only (remove symlinks) |
| `--restow` | `-R` | bool | Restow (unstow then stow again) |

### Conflict Resolution

| Flag | Type | Description |
|------|------|-------------|
| `--force` | bool | Overwrite existing destination files |
| `--backup` | bool | Create `*.bak` backups before modifying files |
| `--adopt` | bool | Replace existing files with symlinks (adopt mode) |

### Merge Mode

| Flag | Type | Description |
|------|------|-------------|
| `--merge` | bool | Enable merge mode for multiple packages |
| `--merge-append` | bool | Append content to mergeable files (shell configs) |
| `--merge-extensions` | `LIST` | Comma-separated extensions to merge (e.g., `.bashrc,.zshrc`) |
| `--show-merge-history` | bool | Display merge operation audit log |

### Git Integration

| Flag | Type | Description |
|------|------|-------------|
| `--git` | `-g` | bool | Auto-commit changes in git repository |
| `--git-push` | bool | Push to remote after commit |
| `--message` | `-m` | `STRING` | Custom git commit message |

### Dotfiles Mode

| Flag | Type | Description |
|------|------|-------------|
| `--dotfiles` | bool | Transform `dot-` prefixed files to `.` (dot-bashrc → .bashrc) |

---

## Usage Guide

### Basic Operations

#### List Packages

```bash
# Show all available packages
ruslink list --dir ~/.dotfiles

# Output:
# Packages available in ~/.dotfiles:
#   • nvim
#   • bash
#   • tmux
# Total: 3 package(s)
```

#### Show Status

```bash
# Check if package is installed
ruslink status nvim --dir ~/.dotfiles --target ~

# Show summary (without package name)
ruslink status --dir ~/.dotfiles --target ~
```

#### Stow

```bash
# Default stow (dir: current directory, target: parent)
ruslink nvim

# Explicit paths
ruslink nvim --dir ~/.dotfiles --target ~
```

#### Unstow

```bash
# Remove symlinks for a package
ruslink nvim --delete --dir ~/.dotfiles --target ~
```

#### Restow

```bash
# Unstow then stow again (useful for updates)
ruslink nvim --restow --dir ~/.dotfiles --target ~
```

#### Clean

```bash
# Remove broken symlinks and empty directories
ruslink clean --dir ~/.dotfiles --target ~ --dry-run -v

# Actually clean (after previewing)
ruslink clean --dir ~/.dotfiles --target ~
```

### Merge Mode

Merge mode allows combining multiple packages with intelligent conflict resolution:

```bash
ruslink package --merge --merge-append \
  --merge-extensions ".bashrc,.zshrc,.config/fish/config.fish"
```

**How it works:**
- Files matching `--merge-extensions` are **appended** with markers
- Directories are **merged** recursively
- Conflicts require `--force`, `--adopt`, or explicit merge settings
- All merges are logged to `.ruslink-merge-log` for audit

**Default merge extensions:**
- `.bashrc`, `.bash_profile`
- `.zshrc`, `.profile`
- `.fishrc`

**Example merge result:**

After merging multiple packages, `~/.bashrc` will contain:

```bash
# Original base/.bashrc content

# === ruslink [dev] ===
# Content from dev/.bashrc (appended)
# === ruslink [dev] (end) ===

# === ruslink [gui] ===
# Content from gui/.bashrc (appended)
# === ruslink [gui] (end) ===
```

View history:

```bash
ruslink gui --show-merge-history --target ~
# Output: ~/.ruslink-merge-log with timestamps and packages
```

### Git Integration

```bash
# Auto-commit changes in the package directory
ruslink config --git

# With custom message
ruslink config --git --message "Update configuration (feature-xyz)"

# Push to remote after committing
ruslink config --git --git-push
```

**Features:**
- Automatic `git add -A` before commit
- Sanitized commit messages (max 100 chars first line)
- Timestamps in automatic messages
- Remote push support
- Auto-detects git repository (skips if not in git repo)

**How it works:**
1. If `--git` flag is set: uses custom message or default
2. If `--git` not set but changes detected: creates silent auto-commit
3. Commit message format (auto): `chore(package): auto-update configuration (YYYY-MM-DD HH:MM)`

### Conflict Resolution

When stowing encounters existing files:

#### `--force` (Overwrite)
```bash
ruslink nvim --force --dir ~/.dotfiles --target ~
# Overwrites all existing files with symlinks
# Add --backup to keep backups
```

#### `--adopt` (Adopt Mode)
```bash
ruslink nvim --adopt --dir ~/.dotfiles --target ~
# Replaces existing files with symlinks (adopts them)
# Useful when you have manual configs that should be managed
```

#### `--backup` (Safe Overwrite)
```bash
ruslink nvim --force --backup --dir ~/.dotfiles --target ~
# Creates *.bak files before overwriting
# Backup numbering: file.bak, file.bak1, file.bak2, ...
```

#### `--merge` (Intelligent Merge)
```bash
ruslink nvim --merge --merge-append --dir ~/.dotfiles --target ~
# Merges compatible files (shell configs) instead of overwriting
# Fails on incompatible conflicts unless --force or --adopt used
```

### Dry Run & Safety

Always preview before destructive operations:

```bash
# Preview stowing
ruslink nvim --dry-run -v --dir ~/.dotfiles --target ~

# Preview unstowing
ruslink nvim --delete --dry-run -v --dir ~/.dotfiles --target ~

# Preview cleanup
ruslink clean --dry-run -v --dir ~/.dotfiles --target ~

# Preview merge with details
ruslink dev --merge --merge-append --dry-run -v --target ~
```

**Dry-run behavior:**
- Shows all operations that would be performed
- No files are created, modified, or deleted
- Git operations are skipped (for safety)
- Interactive confirmations are still shown

---

## Configuration

### Ignore Patterns

ruslink automatically respects ignore patterns from:

1. **`.gitignore`** — Standard git ignore file (in package directory)
2. **`.ruslink.ignore`** — Custom ruslink-specific ignore patterns

#### Default Ignored Patterns

The following are always skipped:

```
.git                    # Git metadata
.gitmodules             # Git submodules
.gitignore              # Ignore files themselves
.ruslink.ignore         # ruslink ignore files
README*                 # Documentation
LICENSE*                # License files
COPYING*                # Copyright notices
.DS_Store               # macOS metadata
*.bak                   # Backup files
*.tmp                   # Temporary files
```

#### Custom Ignore File

Create `.ruslink.ignore` in your package directory:

```
# Comments start with #
node_modules
*.swp
.vscode/temp

# Glob patterns supported
test/**/*.log
tmp/*
```

### Merge Configuration

Merge behavior is controlled via command-line flags (see [Merge Mode](#merge-mode)):

```bash
# Enable merge mode with default extensions
ruslink package --merge --merge-append

# Customize which extensions to merge
ruslink package --merge --merge-append \
  --merge-extensions ".bashrc,.zshrc,.config/fish/config.fish"
```

The merge history is logged to `.ruslink-merge-log` in the stow directory.

---

## Error Handling & Logging

### Verbose Mode

Enable detailed logging with `-v` or `--verbose`:

```bash
ruslink nvim -v --dir ~/.dotfiles --target ~
```

Output includes:
- Configuration details
- Symlink creation operations
- File adoption/backup operations
- Git operations and status
- Pattern matching and ignore decisions
- Merge operations with markers

### Human-Friendly Errors

Instead of cryptic panic messages, ruslink provides clear, actionable errors:

```
Error: Conflict: ~/.bashrc already exists (use --force, --adopt, or --merge)

Error: Package 'nvim' not found in ~/.dotfiles

Error: Git is not installed or not found in PATH. Please install Git first.
```

### Color Support

Colors can be toggled at compile time via feature flags:

```bash
# Build without colors
cargo build --release --no-default-features --features git

# Build with colors only
cargo build --release --no-default-features --features colors

# Minimal (no colors, no git)
cargo build --release --no-default-features
```

---

## Project Structure

```
src/
├── main.rs              # Application entry point
├── app.rs               # Main application logic and orchestration
├── cli/
│   ├── mod.rs           # CLI module exports
│   ├── args.rs          # Clap-based argument parsing
│   └── config.rs        # Configuration struct
├── git/
│   ├── mod.rs           # Git module exports
│   ├── gitmanager.rs    # GitRepository with low-level operations
│   └── operations.rs    # High-level git orchestration
├── stow/
│   ├── mod.rs           # Stow module exports
│   ├── stowmanager.rs   # Stow/unstow implementation
│   ├── commands.rs      # List/status/clean commands
│   └── merge.rs         # Merge mode and conflict resolution
└── utils/
    ├── mod.rs           # Utils module exports
    ├── confirm.rs       # Interactive user confirmation
    ├── ignore.rs        # Ignore pattern loading and matching
    ├── output.rs        # Colored output helpers
    └── tracing.rs       # Structured logging setup
```

### Key Modules

- **`app.rs`** — Orchestrates the stow/unstow/list/status/clean workflow with logging
- **`gitmanager.rs`** — Low-level git operations (add, commit, push)
- **`stowmanager.rs`** — Core symlink creation and file management
- **`merge.rs`** — Conflict detection and intelligent merge strategies
- **`ignore.rs`** — Regex-based pattern matching with caching
- **`commands.rs`** — Introspection commands (list, status, clean)

---

## Build Options

ruslink supports feature flags for minimal binary sizes:

### Default Build (All Features)
```bash
cargo build --release
# Includes: git, colors
# Size: ~5-8 MB
```

### Minimal Build
```bash
cargo build --release --no-default-features
# Size: ~3-4 MB
# No git, colors, or advanced tracing
```

### No Git
```bash
cargo build --release --no-default-features --features colors
# For systems without git requirements
# Size: ~4-5 MB
```

### No Colors
```bash
cargo build --release --no-default-features --features git
# Lighter output, no colored terminal text
# Size: ~4-5 MB
```

---

## Examples

### Complete Dotfiles Setup

```bash
# 1. Safe preview
ruslink base --target ~ --dry-run -v

# 2. Apply base configuration
ruslink base --target ~ --git --message "Init: base config"

# 3. Add development tools
ruslink dev --target ~ --merge --merge-append \
  --merge-extensions ".bashrc,.zshrc,.config/fish/config.fish"

# 4. Add GUI configuration
ruslink gui --target ~ --merge --merge-append

# 5. View what was merged
ruslink gui --target ~ --show-merge-history
```

### Migrating Between Machines

```bash
# On source machine: stow and commit
ruslink config --dir ~/.dotfiles --target ~ --git --git-push

# On target machine: clone repo and stow
git clone <repo> ~/.dotfiles
ruslink config --dir ~/.dotfiles --target ~ --git
```

### Safe Update with Backup

```bash
# Update with backup of existing files
ruslink updated-package --restow --force --backup
```

### Troubleshooting & Recovery

```bash
# Preview cleanup of broken links
ruslink clean --dry-run -v --dir ~/.dotfiles --target ~

# Actually clean (removes broken symlinks and empty dirs)
ruslink clean --dir ~/.dotfiles --target ~

# View what would happen (verbose mode)
ruslink nvim --dry-run -v --force --dir ~/.dotfiles --target ~
```

---

## Notes & Best Practices

- **Dry-run First** — Always use `--dry-run -v` before destructive operations
- **Backup Important Files** — Use `--backup` when overwriting existing files
- **Git Integration** — Enable `--git` to track all changes
- **Merge Mode Clarity** — Use `--show-merge-history` to verify merges
- **Test Permissions** — Ensure proper read/write access to target directories
- **When Dry-Run is Enabled** — Git auto-commit is automatically disabled for safety
- **Multiple Packages** — Use merge mode to combine shell configs from different packages
- **Dotfiles Naming** — Start files with `dot-` to enable automatic `.` prefix transformation

---

## Troubleshooting

### Package Not Found
```
Error: Package 'nvim' not found in ~/.dotfiles
```
**Solution:** Verify the package directory exists: `ls -la ~/.dotfiles/nvim`

### Existing File Conflicts
```
Error: Conflict: ~/.bashrc already exists (use --force, --adopt, or --merge)
```
**Solution:** Choose a strategy:
- `--adopt` — Replace with symlink
- `--force --backup` — Overwrite and backup
- `--merge` — Merge compatible files (shell configs)

### Git Not Found
```
Error: Git is not installed or not found in PATH. Please install Git first.
```
**Solution:** Install Git: `sudo apt install git` (Linux) or `brew install git` (macOS)

### Permission Denied
```
Error: Failed to create symlink: Permission denied
```
**Solution:** Check directory permissions: `ls -la ~/.dotfiles`

### Broken Symlinks After Manual Deletions
```bash
# Use clean command to remove them
ruslink clean --dir ~/.dotfiles --target ~ --dry-run -v
ruslink clean --dir ~/.dotfiles --target ~
```

---

## Contributing

Contributions are welcome! Please ensure:
- Code passes `just lint`
- Tests pass with `just test`
- Documentation is updated
- All features are tested

---

## License

This project is released under the MIT License - see the [LICENSE](LICENSE) file for details.
