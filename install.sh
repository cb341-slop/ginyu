#!/bin/bash

set -e

echo "Building ginyu..."
cargo build --release

echo "Installing ginyu to ~/.cargo/bin..."
cargo install --path .

echo "✓ ginyu installed successfully!"
echo ""
echo "Try it out:"
echo "  ginyu --help"
echo "  ginyu modified"
echo "  ginyu group"
