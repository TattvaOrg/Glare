#!/bin/bash
set -e

REPO="https://github.com/AbsolOrg/Glare.git"
INSTALL_DIR="/usr/local/bin"
TMP_DIR=$(mktemp -d)

trap 'rm -rf "$TMP_DIR"' EXIT

echo ":: Cloning Glare..."
git clone --depth 1 "$REPO" "$TMP_DIR/glare" 2>/dev/null

echo ":: Building (release)..."
cargo build --release --manifest-path "$TMP_DIR/glare/Cargo.toml" 2>/dev/null

echo ":: Installing to $INSTALL_DIR..."
sudo install -Dm755 "$TMP_DIR/glare/target/release/glare" "$INSTALL_DIR/glare"

echo ":: Done. Run 'glare' to start."
