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
    @echo " just pre-commit           → Full preparation before commit"
    @echo " just check-lock           → Verify Cargo.lock consistency"
    @echo " just build-release-strict → Build with --locked (CI-like)"
    @echo " just clean                → Cargo clean"
    @echo " just clean-release-artifacts → Remove local release artifacts"
    @echo ""
    @echo "=== Release ==="
    @echo " just release-dry-run      → Preview release"
    @echo " just release              → Create tag + push (triggers GitHub CI)"
    @echo " just release-clean        → Delete old release + create new one"
    @echo " just release-local        → Build and install locally (for testing)"
    @echo ""

# ─── Build & Development ───────────────────────────────────────
build:
    cargo watch -c -w src/ -x "build --color=always"

run:
    cargo watch -c -w src/ -x "run --color=always"

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
update:
    cargo update
    just cache
    @echo "✅ Dependencies updated and Cargo.lock regenerated!"

pre-commit:
    just fmt
    just lint
    just update
    just check-lock
    @echo "🎉 Pre-commit checks completed! Ready to commit."

check-lock:
    cargo check --locked
    @echo "✅ Cargo.lock is consistent with Cargo.toml"

build-release-strict:
    cargo build --release --locked
    @echo "✅ Release build with --locked completed successfully!"

clean:
    cargo clean

cache:
    cargo-cache --remove-dir all || echo "cargo-cache not installed (optional)"

# ─── Release Artifacts Cleanup (local only) ────────────────────
clean-release-artifacts:
    @echo "🧹 Cleaning local release artifacts..."
    rm -rf target/release/ruslink target/release/ruslink.exe 2>/dev/null || true
    rm -rf target/release/*.tar.gz target/release/*.zip 2>/dev/null || true
    @echo "→ Local artifacts removed"

# ─── Release ───────────────────────────────────────────────────
version := `grep "^version" Cargo.toml | awk -F'"' '{print $2}' | head -n1`

release-dry-run:
    @echo "Current version in Cargo.toml → {{version}}"
    @echo "Tag that will be created     → v{{version}}"
    @echo ""
    @echo "This will trigger the GitHub Actions workflow to build official binaries."

release-clean:
    @echo "🧹 Preparing fresh release for v{{version}}..."

    just clean-release-artifacts

    @echo "→ Deleting old GitHub Release and tag..."
    gh release delete "v{{version}}" --yes --cleanup-tag 2>/dev/null && echo "→ Old release deleted" || echo "→ No previous release found"

    @echo "→ Deleting local tag..."
    git tag -d "v{{version}}" 2>/dev/null && echo "→ Local tag deleted" || echo "→ No local tag"

    @echo ""
    @echo "🚀 Starting clean release..."
    just release

release:
    @echo "=== Preparing release v{{version}} ==="
    
    just pre-commit

    @echo "Committing Cargo.lock (if changed)..."
    git add Cargo.lock
    git commit -m "chore: update Cargo.lock for v{{version}}" || echo "→ No changes to Cargo.lock"

    @echo "Creating annotated tag v{{version}}..."
    git tag -a "v{{version}}" -m "Release v{{version}}"

    @echo "Pushing commit and tag to GitHub..."
    git push origin main --follow-tags

    @echo ""
    @echo "🎉 Tag v{{version}} pushed successfully!"
    @echo "→ GitHub Actions is now building the official binaries and creating the release."

# Local install (for testing only)
release-local:
    just build-release-strict
    cargo install --path . --locked
    @echo "✅ ruslink installed locally from source (for testing)"

# Hidden helper (kept for internal use)
build-release-strict:
    cargo build --release --locked