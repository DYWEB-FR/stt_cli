#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "Building release..."
cargo build --release

BIN="$ROOT/target/release/stt_cli"
if [ ! -f "$BIN" ]; then
  echo "Binary not found at $BIN"
  exit 1
fi

PKG_DIR="$ROOT/dist"
rm -rf "$PKG_DIR"
mkdir -p "$PKG_DIR"

TMP="$PKG_DIR/stt_cli_bundle"
rm -rf "$TMP"
mkdir -p "$TMP"

cp "$BIN" "$TMP/" || true
cp README.md "$TMP/" || true
cp docs/USAGE.md "$TMP/" || true
cp Cargo.toml "$TMP/" || true

chmod +x "$TMP/stt_cli" || true

ARCHIVE_NAME="stt_cli-linux.tar.gz"
tar -C "$TMP" -czf "$PKG_DIR/$ARCHIVE_NAME" .

echo "Package created: $PKG_DIR/$ARCHIVE_NAME"
