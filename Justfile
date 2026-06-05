# chumsky-101 task runner. `just` is managed by mise.
# List all recipes:  just            (or just --list)

# Run recipes under bash with strict mode (abort on error / unset var / failed pipe).
set shell := ["bash", "-euo", "pipefail", "-c"]

# Default: show all available recipes
default:
    @just --list

# ── Environment / dependencies ───────────────────────────────────────────
# Bootstrap a fresh checkout: install mise tools + git hooks
setup: install-tools install-hooks

# Install the tools declared in mise.toml (nextest / insta / cargo-expand / prek / just …)
install-tools:
    mise install

# Upgrade tools to the latest version matching their "latest" pin
update:
    mise upgrade

# ── prek (git hooks) ─────────────────────────────────────────────────────
# Install git hooks into .git/hooks (pre-commit + pre-push)
install-hooks:
    prek install

# Run every hook against all files (same gate as on commit)
hooks:
    prek run --all-files

# ── Pre-push self-check (mirrors ci.yml exactly) ─────────────────────────
# If this passes locally, CI should stay green
verify:
    cargo fmt --all --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo nextest run --all-targets
