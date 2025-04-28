#!/usr/bin/env bash

set -e  # exit on error

# Build the project
cargo build --release

# Copy the compiled binary to ~/bin/
cp target/release/arc ~/bin/arc

echo "✅ Built and installed to ~/bin/"
