set shell := ["bash", "-cu"]

default: check

# First-time setup: install deps, generate code, verify
setup:
    ./scripts/setup.sh

# Full check pipeline
check: fmt lint test

# Format all code
fmt:
    cd rust && cargo fmt --all
    cd flutter && dart format .

# Lint all code
lint:
    cd rust && cargo clippy --workspace -- -D warnings
    cd flutter && dart analyze --fatal-infos

# Run all tests
test:
    cd rust && cargo test --workspace
    cd flutter && bash -c 'for d in packages/*/test apps/*/test; do [ -d "$$d" ] && (cd "$$(dirname $$d)" && flutter test) || true; done'

# Rust check only
rust-check:
    cd rust && cargo check --workspace

# Generate all code (proto + FFI headers)
generate:
    ./scripts/generate_proto.sh
    cd rust && cargo build -p app_core

# Install the pattern-h CLI globally
install-cli:
    cargo install --path tools/cli

# Clean all build artifacts
clean:
    cd rust && cargo clean
    cd flutter && bash -c 'for d in packages/* apps/*; do [ -d "$$d" ] && (cd "$$d" && flutter clean 2>/dev/null) || true; done'
