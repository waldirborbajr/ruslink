# ┌───────────────────────────────────────────────────────────────┐
# │ Justfile for ruslink                                          │
# │                                                               │
# │ Commands: just → show this help message                       │
# └───────────────────────────────────────────────────────────────┘

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set dotenv-load := true

# Default recipe
default: help

# ─── Help ──────────────────────────────────────────────────────
help:
    @echo "Available commands for ruslink:"
    @echo ""
    @echo "=== Development ==="
    @echo " just / just help          → Show this help message"
    @echo " just build / b            → Watch + build (default features)"
    @echo " just run / r              → Watch + run"
    @echo ""
    @echo "=== Feature Builds ==="
    @echo " just build-minimal        → Build without git and colors"
    @echo " just build-no-git         → Build with colors but no git"
    @echo " just build-no-colors      → Build with git but no colors"
    @echo " just build-release        → Release build (default features)"
    @echo " just build-release-minimal→ Smallest possible binary"
    @echo ""
    @echo "=== Quality ==="
    @echo " just fmt                  → Format code"
    @echo " just fmt-check            → Check formatting"
    @echo " just lint                 → fmt-check + clippy"
    @echo " just clippy               → Run clippy (warnings denied)"
    @echo " just clippy-fix           → Auto-fix clippy suggestions"
    @echo " just test                 → Run tests"
    @echo " just check                → Cargo check"
    @echo ""
    @echo "=== Maintenance ==="
    @echo " just update               → Update dependencies + Cargo.lock"
    @echo " just pre-commit           → Full preparation before commit (recommended)"
    @echo " just check-lock           → Verify Cargo.lock consistency"
    @echo " just build-release-strict → Build with --locked (CI-like)"
    @echo " just release              → Build + install locally"
    @echo " just clean                → Clean build artifacts"
    @echo " just cache                → Clear cargo cache"
    @echo " just size                 → Show binary sizes"
    @echo ""

# ─── Build & Development ───────────────────────────────────────
# Watch + build (default features: git + colors)
build:
    cargo watch -c -w src/ -x "build --color=always"

# Watch + run
run:
    cargo watch -c -w src/ -x "run --color=always"

# Shortcuts
b: build
r: run

# ─── Feature Builds ────────────────────────────────────────────
build-minimal:
    cargo build --no-default-features --features minimal

build-no-git:
    cargo build --no-default-features --features colors

build-no-colors:
    cargo build --no-default-features --features git

build-release:
    cargo build --release

build-release-minimal:
    cargo build --release --no-default-features --features minimal

# ─── Quality ───────────────────────────────────────────────────
fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

check:
    cargo check --all-targets --all-features

test:
    cargo test --all-features -- --nocapture

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

clippy-fix:
    cargo clippy --fix --allow-dirty --allow-staged --all-targets --all-features

lint: fmt-check clippy

# ─── Maintenance ───────────────────────────────────────────────

# Update dependencies and Cargo.lock
update:
    cargo update
    just cache
    @echo "✅ Dependencies updated and Cargo.lock regenerated!"
    @echo "💡 Don't forget to commit the lockfile:"
    @echo "   git add Cargo.lock && git commit -m 'chore: update Cargo.lock'"

# Full preparation before committing (recommended)
pre-commit:
    just fmt
    just lint
    just update
    just check-lock
    @echo "🎉 Pre-commit checks completed! Ready to commit."

# Verify Cargo.lock is consistent with Cargo.toml
check-lock:
    cargo check --locked
    @echo "✅ Cargo.lock is consistent with Cargo.toml"

# Build with strict locked mode (same as CI)
build-release-strict:
    cargo build --release --locked
    @echo "✅ Release build with --locked completed successfully!"

# Clean build artifacts
clean:
    cargo clean

# Clear cargo cache
cache:
    cargo-cache --remove-dir all || echo "cargo-cache not installed (optional tool)"

# Build and install locally
release:
    just build-release-strict
    cargo install --path . --locked

# Show binary sizes
size:
    @echo "Binary sizes:"
    @ls -lh target/release/ruslink 2>/dev/null || echo "Release binary not found"
    @ls -lh target/debug/ruslink 2>/dev/null || echo "Debug binary not found"
