#!/bin/bash

# Ensure cargo is installed
if ! command -v cargo &> /dev/null
then
    echo "cargo (Rust toolchain) could not be found. Please install Rust: https://rustup.rs/"
    exit 1
fi

# Build the Rust project
echo "Building rust_roast project..."
cargo build --release

# Check if build was successful
if [ $? -ne 0 ]; then
    echo "Rust project build failed. Aborting installation."
    exit 1
fi

# Create ~/.local/bin if it doesn't exist
mkdir -p ~/.local/bin

# Copy the compiled executable to ~/.local/bin
echo "Installing rust_roast to ~/.local/bin/"
cp ./target/release/rust_roast ~/.local/bin/rust_roast

# Make it executable (though cargo build --release usually handles this)
chmod +x ~/.local/bin/rust_roast

echo "rust_roast installed successfully!"