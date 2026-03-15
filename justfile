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
    cd flutter && melos run test

# Rust check only
rust-check:
    cd rust && cargo check --workspace

# Generate all code (proto + FFI headers)
generate:
    ./scripts/generate_proto.sh
    cd rust && cargo build -p app_core

# Create a new project from this skeleton
scaffold *args:
    cargo run --manifest-path tools/cli/Cargo.toml -- {{args}}

# Install the scaffold CLI globally
install-cli:
    cargo install --path tools/cli

# Clean all build artifacts
clean:
    cd rust && cargo clean
    cd flutter && melos run clean
