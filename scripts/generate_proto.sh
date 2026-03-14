#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
PROTO_DIR="$ROOT_DIR/protos"
DART_OUT="$ROOT_DIR/flutter/packages/proto_models/lib/src/generated"

mkdir -p "$DART_OUT"

PROTO_FILES=()
while IFS= read -r -d '' f; do
    PROTO_FILES+=("$f")
done < <(find "$PROTO_DIR" -name "*.proto" -type f -print0)

if [ ${#PROTO_FILES[@]} -eq 0 ]; then
    echo "No .proto files found in $PROTO_DIR"
    exit 0
fi

echo "Generating Dart protobuf models..."
protoc \
    --dart_out="$DART_OUT" \
    -I "$PROTO_DIR" \
    "${PROTO_FILES[@]}"

echo "Generating Rust protobuf types..."
cd "$ROOT_DIR/rust"
cargo check -p proto

echo "Done."
