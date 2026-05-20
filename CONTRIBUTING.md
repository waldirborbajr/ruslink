# Contributing to ruslink

Thank you for your interest in contributing to ruslink! This document provides guidelines and instructions for contributing code, documentation, bug reports, and feature requests.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Code Style & Quality](#code-style--quality)
- [Commit Messages](#commit-messages)
- [Pull Request Process](#pull-request-process)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Features](#suggesting-features)
- [Documentation](#documentation)
- [Release Process](#release-process)

---

## Code of Conduct

This project adheres to the Rust Community Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

Key principles:
- Be respectful and inclusive
- Welcome diverse perspectives and experiences
- Focus on constructive feedback
- Report harassment or discriminatory behavior
- Help create a safe and welcoming environment

---

## Getting Started

### Prerequisites

- Rust 1.70+ (install from https://rustup.rs/)
- Cargo (comes with Rust)
- Git
- Just (recommended, for convenience)

Install Just:

Linux / macOS:
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to ~/.cargo/bin

Windows (via Cargo):
cargo install just

### Fork & Clone

1. Fork the repository on GitHub: https://github.com/waldirborbajr/ruslink/fork
2. Clone your fork:
   git clone https://github.com/YOUR_USERNAME/ruslink.git
   cd ruslink
3. Add upstream remote:
   git remote add upstream https://github.com/waldirborbajr/ruslink.git
4. Create a tracking branch:
   git branch --set-upstream-to=upstream/main main

---

## Development Setup

### Using Just (Recommended)

The project includes a `justfile` for convenient development:

Show all available commands:
just help

Build with watch mode:
just build

Run with watch mode:
just run

Run tests:
just test

Run full linting:
just lint

### Using Cargo Directly

Build the project:
cargo build

Build release:
cargo build --release

Run:
cargo run -- --help

Run tests:
cargo test

Format code:
cargo fmt

Lint code:
cargo clippy --all-targets --all-features

---

## Making Changes

### Creating a Feature Branch

1. Sync with upstream:
   git fetch upstream
   git rebase upstream/main

2. Create a feature branch:
   git checkout -b feature/your-feature-name

3. Make your changes
4. Keep your branch up to date:
   git fetch upstream
   git rebase upstream/main

### Branch Naming Convention

Use descriptive branch names following this pattern:

feature/description - New features
bugfix/description - Bug fixes
docs/description - Documentation changes
refactor/description - Code refactoring
perf/description - Performance improvements
test/description - Adding or improving tests

Examples:
feature/merge-mode-enhancements
bugfix/symlink-validation
docs/security-section
refactor/config-module
perf/parallel-operations
test/conflict-scenarios

### What to Include in Your PR

- Clear description of changes
- Motivation and context
- Tests for new functionality
- Documentation updates
- Updated CHANGELOG.md (if applicable)

---

## Testing

### Running Tests

Run all tests:
just test

Run specific test:
cargo test test_name -- --nocapture

Run with backtrace on failure:
RUST_BACKTRACE=1 cargo test

### Writing Tests

Tests should be placed in the same module as the code being tested:

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let result = some_function();
        assert_eq!(result, expected_value);
    }

    #[test]
    fn test_error_case() {
        let result = fallible_function();
        assert!(result.is_err());
    }
}

### Test Requirements

- All new features must have tests
- All bug fixes should include regression tests
- Tests should cover happy path and error cases
- Use descriptive test names
- Aim for >80% code coverage (use `cargo tarpaulin`)

### Integration Tests

For complex scenarios, use the `tests/` directory:

tests/integration_test.rs

use tempfile::TempDir;
use ruslink::stow::stow_package;

#[test]
fn test_stow_integration() {
    let temp = TempDir::new().unwrap();
    let result = stow_package(&temp.path(), &temp.path(), &config, &[]);
    assert!(result.is_ok());
}

---

## Code Style & Quality

### Formatting

Format all code before committing:
just fmt

This runs `cargo fmt --all`.

### Clippy Linting

Fix clippy warnings:
just clippy-fix

Run clippy manually:
just clippy

All warnings must be resolved (no clippy warnings allowed in PRs).

### Full Linting Check

Run complete linting pipeline:
just lint

This runs: fmt --check + clippy

### Code Quality Standards

- Use idiomatic Rust (follow Rust API guidelines)
- Prefer iterators over explicit loops
- Use Result<T, E> for fallible operations, never unwrap() in library code
- Add `#[derive(Debug)]` for types that impl Clone
- Document public APIs with doc comments (///)
- Use consistent naming (snake_case for functions, CamelCase for types)
- Keep functions focused and small (aim for <50 lines)
- Extract complex logic into helper functions

### Documentation Standards

All public items must have doc comments:

/// Stows a package from source to target directory.
///
/// This function creates symlinks from source files to target location,
/// respecting ignore patterns and conflict resolution settings.
///
/// # Arguments
///
/// * `source` - Path to the package directory
/// * `target` - Path to the target directory
/// * `config` - Configuration settings
/// * `ignores` - Regex patterns to ignore
///
/// # Returns
///
/// Returns `Ok(StowStats)` with statistics on success,
/// or `Err` with error message on failure.
///
/// # Examples
///
/// let stats = stow_package(&source, &target, &config, &ignores)?;
/// println!("Linked {} files", stats.files_linked);
///
/// # Panics
///
/// Never panics in library code.
pub fn stow_package(
    source: &Path,
    target: &Path,
    config: &Config,
    ignores: &[regex::Regex],
) -> Result<StowStats> {
    // implementation
}

---

## Commit Messages

### Conventional Commits Format

Follow the Conventional Commits specification: https://www.conventionalcommits.org/

Format:
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]

Types:
- feat: A new feature
- fix: A bug fix
- docs: Documentation only changes
- style: Changes that don't affect code meaning (formatting, etc.)
- refactor: Code change that neither fixes a bug nor adds a feature
- perf: Code change that improves performance
- test: Adding or updating tests
- chore: Changes to build system, dependencies, CI, etc.
- ci: Changes to CI configuration files and scripts

Examples:

feat(merge): add intelligent file appending for shell configs

This implementation allows merging content from multiple packages
into shell configuration files (.bashrc, .zshrc, etc.) with clear
markers to identify which package added which content.

Fixes #123

fix(symlink): validate symlink targets before creation

Prevents creation of symlinks pointing outside the target directory,
improving security and user safety.

docs(readme): add security section

Adds comprehensive documentation on backup strategies, symlink
validation, and safety guarantees.

Closes #456

refactor(config): simplify configuration loading

Extract configuration loading logic into separate module for better
testability and maintainability.

perf(ignore): cache compiled regex patterns

Reduces compilation overhead for ignore patterns by caching them
across multiple package operations.

### Commit Message Guidelines

- Use imperative mood ("add feature" not "added feature")
- Don't capitalize first letter of description
- Don't end description with period
- Limit first line to 50 characters (soft limit 72)
- Explain WHAT and WHY, not HOW
- Reference issues: "Fixes #123" or "Closes #456"
- Keep commits atomic (one logical change per commit)

---

## Pull Request Process

### Before Submitting

1. Ensure all tests pass:
   just test

2. Run full linting:
   just lint

3. Format code:
   just fmt

4. Update documentation if needed

5. Add yourself to CONTRIBUTORS.md (optional)

6. Rebase on main:
   git fetch upstream
   git rebase upstream/main

7. Force push to your branch:
   git push -f origin your-branch-name

### PR Description Template

Use this template when creating a PR:

Title: [SHORT DESCRIPTION]

Closes #ISSUE_NUMBER

## Description

Brief description of changes.

## Motivation and Context

Why are these changes needed? What problem do they solve?

## Testing

How have you tested these changes?

- [ ] All tests pass
- [ ] New tests added for new functionality
- [ ] Tested on Linux
- [ ] Tested on macOS
- [ ] Tested on Windows

## Documentation

- [ ] Updated README if needed
- [ ] Updated doc comments
- [ ] Updated CHANGELOG.md

## Breaking Changes

- [ ] No breaking changes
- [ ] Breaking changes (describe below)

Breaking changes description:

## Checklist

- [ ] Code follows style guidelines (just lint passes)
- [ ] Self-review of code completed
- [ ] Comments added for complex logic
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] All tests passing
- [ ] No new warnings introduced

### PR Review Process

1. At least one maintainer review required
2. All CI checks must pass
3. Code coverage should not decrease
4. Changes should follow project guidelines
5. Feedback will be addressed in new commits
6. Once approved, PR will be merged

### After Merge

- Your branch will be deleted
- Your contribution will be credited
- Changes will be included in next release

---

## Reporting Bugs

### Before Reporting

1. Check existing issues: https://github.com/waldirborbajr/ruslink/issues
2. Check closed issues for solutions
3. Verify you have the latest version: `ruslink --version`

### Bug Report Template

Title: [BUG] Short description of the issue

Description:

Describe the bug clearly and concisely. What did you expect to happen?
What actually happened?

To Reproduce

Steps to reproduce the behavior:
1. Run command: 'ruslink ...'
2. Use these options: '...'
3. See error

Expected behavior

What should have happened?

Actual behavior

What actually happened?

Environment

- OS: (Linux / macOS / Windows)
- Rust version: (rustc --version)
- ruslink version: (ruslink --version)
- Package structure: (brief description)

Logs

Include relevant output with verbose flag:
ruslink package --verbose --dry-run 2>&1

Additional context

Any other context that might be helpful.

Minimal Example

If possible, provide a minimal reproducible example:
- Small package structure that triggers the bug
- Exact commands to reproduce
- Expected vs actual output

---

## Suggesting Features

### Before Suggesting

1. Check TODO.md: See if feature is already planned
2. Search existing issues: https://github.com/waldirborbajr/ruslink/issues
3. Review closed issues/discussions

### Feature Request Template

Title: [FEATURE] Short description of requested feature

Description:

Clear and concise description of what you want to happen.

Use Case

Why do you need this feature? What problem does it solve?

Current Behavior

How do you currently work around this limitation?

Proposed Solution

Detailed description of how you think this should work.

Alternatives Considered

Other solutions or features you've considered.

Example Usage

If applicable, example of how the feature would be used:

ruslink package --new-feature-option value

Additional Context

Links, related issues, screenshots, etc.

---

## Documentation

### Documentation Standards

- Keep documentation clear and concise
- Use examples for complex features
- Update README.md for user-facing changes
- Update doc comments for API changes
- Add architecture documentation for large changes

### Updating README

- Add examples for new features
- Update feature tables if applicable
- Update command reference if adding/changing options
- Keep examples working and tested

### Adding Architecture Documentation

For complex features, consider adding docs/:

docs/architecture.md - Overall architecture
docs/merge-mode.md - Merge mode implementation details
docs/git-integration.md - Git integration architecture

### Changelog

Update CHANGELOG.md following Keep a Changelog format: https://keepachangelog.com/

Example:

### Added
- Symlink validation with `--validate` flag
- New `--merge-extensions` parameter

### Changed
- Improved error messages for conflict scenarios
- Performance optimization in ignore pattern matching

### Fixed
- Bug where broken symlinks were not detected
- Race condition in concurrent stow operations

### Deprecated
- Old `--legacy-mode` flag (use `--merge` instead)

### Removed
- Support for Rust < 1.70

### Security
- Fix potential directory traversal vulnerability

---

## Release Process

### For Maintainers Only

This section describes how maintainers create releases.

### Pre-Release Checklist

1. Update version in Cargo.toml
2. Update CHANGELOG.md with changes since last release
3. Verify all tests pass: `just test`
4. Run full linting: `just lint`
5. Create release branch: `git checkout -b release/v0.X.Y`
6. Commit version changes:
   git add Cargo.toml CHANGELOG.md
   git commit -m "chore(release): bump version to 0.X.Y"
7. Create annotated tag:
   git tag -a v0.X.Y -m "Release v0.X.Y"
8. Push to main: `git push upstream main`
9. Push tag: `git push upstream v0.X.Y`

### GitHub Actions

Once the tag is pushed:
1. GitHub Actions builds release artifacts
2. Binaries are created for Linux/macOS/Windows
3. Release is published to GitHub Releases
4. Crate is published to crates.io (automatic via GitHub Actions)

### After Release

1. Announce release in project discussions/social media
2. Update website if applicable
3. Close any resolved issues
4. Plan next release/milestone

---

## Getting Help

### Resources

- Rust Book: https://doc.rust-lang.org/book/
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/
- Clippy Lints: https://rust-lang.github.io/rust-clippy/
- ruslink Documentation: See README.md

### Community

- GitHub Issues: Report bugs and suggest features
- GitHub Discussions: Ask questions and discuss ideas
- Rust Community: https://www.rust-lang.org/community

### Common Questions

**Q: How do I run tests?**
A: Use `just test` or `cargo test --all-features`

**Q: Why is my PR not being reviewed?**
A: Maintainers are volunteers. Please be patient. If urgent, mention in issue.

**Q: Can I add a new dependency?**
A: Discuss in an issue first. Minimize dependencies to keep binary small.

**Q: How do I handle Windows-specific code?**
A: Use `#[cfg(windows)]` or `#[cfg(unix)]` attributes.

**Q: Where should I put new tests?**
A: Same file as code with `#[cfg(test)] mod tests { ... }`

**Q: Can I commit directly to main?**
A: No. All changes must go through PR process.

---

## Tips for Successful Contributions

1. Start with small contributions (documentation, tests)
2. Comment on issues before starting large features
3. Keep PRs focused on a single feature/fix
4. Ask for help if stuck
5. Be patient with review process
6. Learn from feedback and improve
7. Test your changes thoroughly
8. Keep code style consistent
9. Write clear commit messages
10. Update documentation

---

## Recognition

Contributors will be:
- Added to CONTRIBUTORS.md
- Thanked in release notes
- Recognized in GitHub contributors page

---

## Questions?

If you have questions about contributing:

1. Check existing issues and discussions
2. Ask in GitHub Discussions
3. Open a new issue with [QUESTION] prefix

---

## Summary

Thank you for contributing to ruslink! Your efforts make this project better for everyone.

Quick checklist before submitting PR:
✓ Code formatted: `just fmt`
✓ Tests added: `just test`
✓ Linting passes: `just lint`
✓ Commit messages follow convention
✓ Documentation updated
✓ No new warnings
✓ Branch rebased on main

---

**Last Updated:** May 2024
**Status:** Active
**Maintained by:** ruslink maintainers

For the latest updates, visit: https://github.com/waldirborbajr/ruslink/blob/main/CONTRIBUTING.md
