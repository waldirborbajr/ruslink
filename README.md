# ruslink

<p align="center">
  <img width="256" height="256" src="./.assets/logo.png" />
</p>

[![CodeQL](https://github.com/waldirborbajr/ruslink/actions/workflows/github-code-scanning/codeql/badge.svg)](https://github.com/waldirborbajr/ruslink/actions/workflows/github-code-scanning/codeql)
[![Publish to crates.io](https://github.com/waldirborbajr/ruslink/actions/workflows/crates.yaml/badge.svg)](https://github.com/waldirborbajr/ruslink/actions/workflows/crates.yaml)
[![Release](https://github.com/waldirborbajr/ruslink/actions/workflows/release.yaml/badge.svg)](https://github.com/waldirborbajr/ruslink/actions/workflows/release.yaml)
[![Rust CI](https://github.com/waldirborbajr/ruslink/actions/workflows/ci.yaml/badge.svg)](https://github.com/waldirborbajr/ruslink/actions/workflows/ci.yaml)
[![Security Audit](https://github.com/waldirborbajr/ruslink/actions/workflows/security-audit.yaml/badge.svg)](https://github.com/waldirborbajr/ruslink/actions/workflows/security-audit.yaml)

A lightweight, production-ready Rust-based stow utility for managing dotfiles and package-style deployments with advanced features like merge mode, git integration, and intelligent conflict resolution.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage Guide](#usage-guide)
  - [Basic Operations](#basic-operations)
  - [Merge Mode](#merge-mode)
  - [Git Integration](#git-integration)
  - [Conflict Resolution](#conflict-resolution)
  - [Dry Run & Safety](#dry-run--safety)
- [Command Reference](#command-reference)
- [Configuration](#configuration)
- [Error Handling & Logging](#error-handling--logging)
- [Project Structure](#project-structure)
- [Build Options](#build-options)
- [License](#license)

## Features

### Core Functionality
- ✅ Stow packages into target locations using symlinks
- ✅ Unstow and restow packages with a single command
- ✅ Respects `.gitignore` and `.ruslink.ignore` patterns
- ✅ Force overwrite existing destination files
- ✅ Backup existing files before overwriting/removing them
- ✅ Adopt mode to replace existing files with symlinks

### Advanced Features
- 🔀 **Merge Mode** - Intelligently merge multiple packages with conflict resolution
  - Content append for shell config files (`.bashrc`, `.zshrc`, `.fish/config.fish`)
  - Directory merging for recursive stowing
  - Merge history tracking with audit logs
- 🔧 **Git Integration** - Automatic commit and push after deployments
  - Auto-commit with custom messages
  - Optional git push to remote
  - Smart commit message sanitization
- 🎯 **Dry-run Mode** - Safe preview of all changes before execution
- 📊 **Structured Logging** - Debug-level insights with the `-v` flag
- ⚠️ **Interactive Confirmation** - User prompts for destructive actions
- 💾 **Human-friendly Errors** - Clear, actionable error messages on failures
- 🌈 **Optional Colors** - Beautiful output with conditional color support

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
# Stow a package into your home directory
ruslink home --dir ~/.dotfiles --target ~

# Preview changes without applying them
ruslink home --dir ~/.dotfiles --target ~ --dry-run -v

# Unstow a package
ruslink home --delete --dir ~/.dotfiles --target ~

# Restow (unstow then stow again)
ruslink home --restow --dir ~/.dotfiles --target ~
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

# Add GUI configuration, merging config files
ruslink gui --target ~ --merge --merge-append

# View merge history
ruslink gui --target ~ --show-merge-history
```

**Merge Result Example:**

After merging multiple packages, your `~/.bashrc` will contain:

```bash
# Original base/.bashrc content

# === ruslink [dev] ===
# Content from dev/.bashrc (appended)
# === ruslink [dev] (end) ===

# === ruslink [gui] ===
# Content from gui/.bashrc (appended)
# === ruslink [gui] (end) ===
```

## Usage Guide

### Basic Operations

```bash
ruslink <package> [OPTIONS]
```

#### Simple Stow

```bash
# Stow 'home' package from current directory's parent
ruslink home

# Stow with explicit directories
ruslink home --dir ~/.dotfiles --target ~
```

#### Unstow

```bash
# Remove symlinks created by ruslink
ruslink home --delete --dir ~/.dotfiles --target ~
```

#### Restow

```bash
# Reinstall a package (unstow then stow)
ruslink home --restow --dir ~/.dotfiles --target ~
```

### Merge Mode

Merge mode allows you to combine multiple packages intelligently:

```bash
ruslink package --merge --merge-append \
  --merge-extensions ".bashrc,.zshrc,.config/fish/config.fish"
```

**How it works:**
- Files matching `--merge-extensions` are **appended** with markers
- Directories are **merged** recursively
- Conflicts require `--force`, `--adopt`, or explicit merge settings
- All merges are logged for audit purposes

**Supported merge extensions (default):**
- `.bashrc`, `.bash_profile`
- `.zshrc`, `.profile`
- `.fishrc` and similar shell configs

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
- Sanitized commit messages (max 100 chars)
- Timestamps in automatic messages
- Remote push support
- Works only in actual git repositories

### Conflict Resolution

When files already exist at the destination, ruslink offers several strategies:

#### Default Behavior
```bash
# Fails with a clear error
ruslink package --dir ~/.dotfiles --target ~
# Error: Conflict: ~/.bashrc already exists (use --force or --adopt)
```

#### Force Overwrite
```bash
# Replace existing files (with optional backup)
ruslink package --force
ruslink package --force --backup  # Renames existing to *.bak
```

#### Adopt Mode
```bash
# Replace existing file with symlink (no backup)
ruslink package --adopt
```

#### Merge Mode
```bash
# Intelligently merge compatible files
ruslink package --merge --merge-append \
  --merge-extensions ".bashrc"
```

### Dry Run & Safety

```bash
# Preview all changes without applying them
ruslink package --dry-run -v

# Auto-confirm destructive actions (skip prompts)
ruslink package --force --yes

# Backup before modifying
ruslink package --force --backup
```

## Command Reference

```
USAGE:
    ruslink <PACKAGE> [OPTIONS]

ARGUMENTS:
    <PACKAGE>               Package name to manage

OPTIONS:
    -d, --dir <DIR>         Stow directory (default: current working directory)
    -t, --target <DIR>      Target directory (default: parent of --dir)
    
    -D, --delete            Unstow the package only
    -R, --restow            Unstow then stow the package
    -n, --dry-run           Simulate changes without applying them
    
    -v, --verbose           Enable debug output (structured logging)
    -y, --yes               Auto-confirm all prompts
    
    -g, --git               Auto-commit changes in git repository
    --git-push              Push to remote after commit
    -m, --message <MSG>     Custom git commit message
    
    --force                 Overwrite existing destination files
    --backup                Create *.bak backups before modifying
    --adopt                 Replace existing files with symlinks
    
    --merge                 Enable merge mode
    --merge-append          Append content to mergeable files
    --merge-extensions <EXT> Comma-separated file extensions to append
                            (e.g., ".bashrc,.zshrc")
    --show-merge-history    Display merge operation log
    
    -h, --help              Show help message
    --version               Show version
```

## Configuration

### Ignore Patterns

ruslink automatically respects ignore patterns from:

1. **`.gitignore`** - Standard git ignore file
2. **`.ruslink.ignore`** - Custom ruslink-specific ignore patterns

#### Default Ignored Patterns

The following files and directories are always skipped:

```
.git                    # Git metadata
.gitmodules             # Git submodules
.gitignore              # Ignore files themselves
.ruslink.ignore         # ruslink ignore files
README*                 # Documentation
LICENSE*                # License files
COPYING*                # Copy right notices
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

Merge behavior is controlled via command-line flags:

```bash
# Enable merge mode with default extensions
ruslink package --merge --merge-append

# Customize which extensions to merge
ruslink package --merge --merge-append \
  --merge-extensions ".bashrc,.zshrc,.config/fish/config.fish"
```

The merge history is automatically logged to `.ruslink-merge-log` in the stow directory.

## Error Handling & Logging

### Verbose Mode

Enable detailed logging with `-v` or `--verbose`:

```bash
ruslink nvim -v
```

Output includes:
- Configuration details
- Symlink creation operations
- File adoption/backup operations
- Git operations and status
- Pattern matching and ignore decisions

### Human-Friendly Errors

Instead of cryptic panic messages, ruslink provides clear, actionable errors:

```
Error: Conflict: ~/.bashrc already exists (use --force or --adopt)

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
```

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
│   └── merge.rs         # Merge mode and conflict resolution
└── utils/
    ├── mod.rs           # Utils module exports
    ├── confirm.rs       # Interactive user confirmation
    ├── ignore.rs        # Ignore pattern loading and matching
    ├── output.rs        # Colored output helpers
    └── tracing.rs       # Structured logging setup
```

### Key Modules

- **`app.rs`** - Orchestrates the stow/unstow workflow with logging
- **`gitmanager.rs`** - Low-level git operations (add, commit, push)
- **`stowmanager.rs`** - Core symlink creation and file management
- **`merge.rs`** - Conflict detection and merge strategies
- **`ignore.rs`** - Regex-based pattern matching with caching

## Build Options

ruslink supports feature flags for minimal binary sizes:

### Default Build (All Features)
```bash
cargo build --release
# Includes: git, colors, tracing
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
# For systems without git
```

### No Colors
```bash
cargo build --release --no-default-features --features git
# Lighter output, no colored terminal text
```

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

## Notes & Best Practices

- **Dry-run First** - Always use `--dry-run -v` before destructive operations
- **Backup Important Files** - Use `--backup` when overwriting existing files
- **Git Integration** - Enable `--git` to track all changes
- **Merge Mode Clarity** - Use `--show-merge-history` to verify merges
- **Test Permissions** - Ensure proper read/write access to target directories
- **When Dry-Run is Enabled** - Git auto-commit is automatically disabled for safety

## Troubleshooting

### Package Not Found
```
Error: Package 'nvim' not found in ~/.dotfiles
```
Verify the package directory exists: `ls -la ~/.dotfiles/nvim`

### Existing File Conflicts
```
Error: Conflict: ~/.bashrc already exists (use --force or --adopt)
```
Choose a strategy:
- `--adopt` - Replace with symlink
- `--force --backup` - Overwrite and backup
- `--merge` - Merge compatible files

### Git Not Found
```
Error: Git is not installed or not found in PATH. Please install Git first.
```
Install Git: `sudo apt install git` (Linux) or `brew install git` (macOS)

### Permission Denied
```
Error: Failed to create symlink: Permission denied
```
Check directory permissions: `ls -la ~/.dotfiles`

## Contributing

Contributions are welcome! Please ensure:
- Code passes `just lint`
- Tests pass with `just test`
- Documentation is updated

## License

This project is released under the MIT License - see the [LICENSE](LICENSE) file for details.
