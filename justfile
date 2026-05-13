# ┌───────────────────────────────────────────────────────────────┐
# │ Justfile for ruslink                                          │
# │                                                               │
# │ Commands:                                                     │
# │ just → show this help message                                 │
# └───────────────────────────────────────────────────────────────┘

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set dotenv-load := true

# Default recipe
default: help

# Show this help message
help:
    @echo "Available commands for ruslink:"
    @echo ""
    @echo " just → show this help"
    @echo " just build / b → watch + build (default features)"
    @echo " just run / r → watch + run"
    @echo ""
    @echo "=== Feature Builds ==="
    @echo " just build-minimal          → Build without git and colors"
    @echo " just build-no-git           → Build with colors but no git"
    @echo " just build-no-colors        → Build with git but no colors"
    @echo " just build-release          → Release build (default features)"
    @echo " just build-release-minimal  → Smallest possible binary"
    @echo ""
    @echo " just test → run tests"
    @echo " just clean → cargo clean"
    @echo " just release → install locally"
    @echo ""

# ─── Build & Development ─────────────────────────────────────────

# Watch + build (default features: git + colors)
build:
    cargo watch -c -w src/ -x "build --color=always"

# Watch + run
run:
    cargo watch -c -w src/ -x "run --color=always"

# Shortcuts
b: build
r: run

# ─── Feature Builds ─────────────────────────────────────────────

# Build minimal (smallest binary - no git, no colors)
build-minimal:
    cargo build --no-default-features --features minimal

# Build with colors but without git support
build-no-git:
    cargo build --no-default-features --features colors

# Build with git but without colors
build-no-colors:
    cargo build --no-default-features --features git

# Release builds
build-release:
    cargo build --release

build-release-minimal:
    cargo build --release --no-default-features --features minimal

# ─── Other Commands ─────────────────────────────────────────────

# Run tests
test:
    cargo test -- --nocapture

# Clean build artifacts
clean:
    cargo clean

# Remove cargo cache
cache:
    cargo-cache --remove-dir all || echo "cargo-cache not installed"

# Build and install release version locally
release:
    cargo build --release
    cargo install --path . --locked

# Quick linting
lint:
    cargo fmt --all           # formata o código
    cargo fmt --all -- --check  # valida que ficou tudo formatado
    cargo clippy --all-targets -- -D warnings

# Update dependencies
update:
    cargo update
    just cache

# Size check (useful after minimal builds)
size:
    @echo "Binary sizes:"
    @ls -lh target/release/ruslink 2>/dev/null || echo "Release binary not found"
    @ls -lh target/debug/ruslink 2>/dev/null || echo "Debug binary not found"
