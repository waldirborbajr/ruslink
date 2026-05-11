# ruslink

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

