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
    @echo " just clean-release-artifacts → Remove release binaries"
    @echo ""
    @echo "=== Release ==="
    @echo " just release-dry-run      → Show what the release will do (safe)"
    @echo " just release              → Create git tag + GitHub Release with changelog"
    @echo " just release-clean        → Delete old release + artifacts then release"
    @echo " just release-local        → Build and install locally"
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

# ─── Release Artifacts Cleanup ─────────────────────────────────
clean-release-artifacts:
    @echo "🧹 Cleaning release artifacts..."
    rm -rf target/release/ruslink target/release/ruslink.exe 2>/dev/null || true
    rm -rf target/release/*.tar.gz target/release/*.zip 2>/dev/null || true
    @echo "→ Release binaries and packages removed"

# ─── Release ───────────────────────────────────────────────────
version := `grep "^version" Cargo.toml | awk -F'"' '{print $2}' | head -n1`

# Generate changelog from commits since last tag
changelog:
    @echo "📜 Generating changelog for v{{version}}..."
    @git log --pretty=format:"- %s" $(git describe --tags --abbrev=0 2>/dev/null || echo "")..HEAD > CHANGELOG.tmp 2>/dev/null || echo "- Initial release" > CHANGELOG.tmp
    @cat CHANGELOG.tmp

# Dry run
release-dry-run:
    @echo "Current version → {{version}}"
    @echo "Tag            → v{{version}}"
    @echo ""
    just changelog

# Clean previous release + artifacts then create new one
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

# Main release with changelog
release:
    @echo "=== Preparing release v{{version}} ==="
    
    just pre-commit

    @echo "Committing Cargo.lock (if changed)..."
    git add Cargo.lock
    git commit -m "chore: update Cargo.lock for v{{version}}" || echo "→ No changes to Cargo.lock"

    @echo "Generating changelog..."
    just changelog

    @echo "Creating annotated tag..."
    git tag -a "v{{version}}" -m "Release v{{version}}"

    @echo "Pushing commit and tag..."
    git push origin main --follow-tags

    @echo "Creating GitHub Release with changelog..."
    gh release create "v{{version}}" \
        --title "v{{version}}" \
        --notes-file CHANGELOG.tmp \
        --latest

    @echo ""
    @echo "🎉 Release v{{version}} created successfully on GitHub!"
    @rm -f CHANGELOG.tmp

# Local install
release-local:
    just build-release-strict
    cargo install --path . --locked