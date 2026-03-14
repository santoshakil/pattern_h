#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

echo "=== Rust Format ==="
cd "$ROOT_DIR/rust"
cargo fmt --all -- --check

echo "=== Rust Clippy ==="
cargo clippy --workspace -- -D warnings

echo "=== Rust Tests ==="
cargo test --workspace

echo "=== Dart Format ==="
cd "$ROOT_DIR/flutter"
dart format --set-exit-if-changed .

echo "=== Flutter Analyze ==="
dart analyze --fatal-infos

echo "=== Flutter Tests ==="
if command -v flutter &> /dev/null; then
  for pkg in "$ROOT_DIR"/flutter/packages/*/test "$ROOT_DIR"/flutter/apps/*/test; do
    if [ -d "$pkg" ]; then
      echo "Testing $(basename "$(dirname "$pkg")")..."
      (cd "$(dirname "$pkg")" && flutter test)
    fi
  done
else
  echo "Flutter SDK not found, skipping Flutter tests"
fi

echo "=== All checks passed ==="
