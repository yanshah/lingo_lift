#!/bin/bash

# Compile the Rust project
cargo build --release

# Copy the binary executable to /usr/local/bin
cp target/release/lingo_lift /usr/local/bin

echo "Installation complete!"
