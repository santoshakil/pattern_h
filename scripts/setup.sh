#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

ok()   { echo -e "${GREEN}[OK]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
fail() { echo -e "${RED}[FAIL]${NC} $1"; exit 1; }

echo "=== Pattern H Setup ==="
echo ""

# Check Rust
if command -v rustc &> /dev/null; then
    ok "Rust $(rustc --version | cut -d' ' -f2)"
else
    fail "Rust not found. Install from https://rustup.rs"
fi

# Check cargo
if command -v cargo &> /dev/null; then
    ok "Cargo found"
else
    fail "Cargo not found"
fi

# Check rustfmt
if rustup component list | grep -q "rustfmt.*installed"; then
    ok "rustfmt installed"
else
    warn "rustfmt not installed, adding..."
    rustup component add rustfmt
fi

# Check clippy
if rustup component list | grep -q "clippy.*installed"; then
    ok "clippy installed"
else
    warn "clippy not installed, adding..."
    rustup component add clippy
fi

# Check Flutter
if command -v flutter &> /dev/null; then
    ok "Flutter $(flutter --version 2>/dev/null | head -1 | cut -d' ' -f2)"
else
    warn "Flutter not found. Install from https://flutter.dev"
    warn "Flutter is needed for the UI layer but Rust tests can run without it"
fi

# Check Dart
if command -v dart &> /dev/null; then
    ok "Dart $(dart --version 2>&1 | cut -d' ' -f4)"
else
    fail "Dart not found. Install Flutter SDK or standalone Dart SDK"
fi

# Check protoc
if command -v protoc &> /dev/null; then
    ok "protoc $(protoc --version | cut -d' ' -f2)"
else
    warn "protoc not found. Install from https://github.com/protocolbuffers/protobuf/releases"
    warn "Needed for protobuf code generation"
fi

# Check protoc Dart plugin
if command -v protoc-gen-dart &> /dev/null; then
    ok "protoc-gen-dart found"
else
    warn "protoc-gen-dart not found"
    if command -v dart &> /dev/null; then
        echo "  Installing via: dart pub global activate protoc_plugin"
        dart pub global activate protoc_plugin
        ok "protoc-gen-dart installed"
    fi
fi

# Check just
if command -v just &> /dev/null; then
    ok "just $(just --version | cut -d' ' -f2)"
else
    warn "just not found. Install from https://github.com/casey/just"
    warn "Optional but recommended for running project commands"
fi

echo ""
echo "=== Installing Dependencies ==="

# Rust deps
echo "Fetching Rust dependencies..."
cd "$ROOT_DIR/rust"
cargo fetch --quiet
ok "Rust dependencies fetched"

# Dart deps
if command -v dart &> /dev/null; then
    echo "Fetching Dart dependencies..."
    cd "$ROOT_DIR/flutter"
    dart pub get --quiet 2>/dev/null || dart pub get
    ok "Dart dependencies fetched"
fi

echo ""
echo "=== Code Generation ==="

# Generate proto (if protoc available)
if command -v protoc &> /dev/null && command -v protoc-gen-dart &> /dev/null; then
    echo "Generating protobuf code..."
    bash "$ROOT_DIR/scripts/generate_proto.sh"
    ok "Protobuf code generated"
else
    warn "Skipping protobuf generation (missing protoc or protoc-gen-dart)"
fi

echo ""
echo "=== Verification ==="

# Rust check
echo "Checking Rust compilation..."
cd "$ROOT_DIR/rust"
if cargo check --workspace --quiet 2>/dev/null; then
    ok "Rust workspace compiles"
else
    warn "Rust compilation has issues (may need proto generation first)"
fi

echo ""
echo -e "${GREEN}=== Setup Complete ===${NC}"
echo ""
echo "Next steps:"
echo "  1. Run 'just check' to verify everything"
echo "  2. Run 'just generate' to regenerate all code"
echo "  3. Start developing!"
