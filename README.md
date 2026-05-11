# ruslink

`ruslink` is a lightweight Rust-based stow utility for managing dotfiles and package-style deployments with support for ignore patterns, dry-run mode, auto git commit, force overwrite, and backup.

## Features

- Stows a package directory into a target location using symlinks
- Supports uninstalling packages and restowing
- Respects `.gitignore` and `.ruslink.ignore` patterns
- Auto commit changes in a git repository
- Force overwrite existing destination files
- Backup existing files before overwriting/removing them
- Dry-run mode for safe previews

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
- `-v, --verbose`: Enable verbose output
- `-g, --git`: Auto commit changes in the package git repository
- `--force`: Overwrite existing destination files
- `--backup`: Backup existing files before modifying them
- `-m, --message <MSG>`: Custom git commit message

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

