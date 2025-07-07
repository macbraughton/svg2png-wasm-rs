#!/bin/bash

# Build script for svg2png-wasm-rs

echo "Building svg2png-wasm-rs..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack is not installed. Installing..."
    cargo install wasm-pack
fi

# Build for web (ES modules)
echo "Building for web target..."
wasm-pack build --target web --out-dir pkg --release

# Build for bundler (webpack, rollup, etc.)
echo "Building for bundler target..."
wasm-pack build --target bundler --out-dir pkg-bundler --release

# Build for nodejs
echo "Building for nodejs target..."
wasm-pack build --target nodejs --out-dir pkg-node --release

echo "Build completed!"
echo "Packages created in:"
echo "  - pkg/ (web target)"
echo "  - pkg-bundler/ (bundler target)"
echo "  - pkg-node/ (nodejs target)"

# Optional: Run tests
if [ "$1" = "test" ]; then
    echo "Running tests..."
    wasm-pack test --headless --chrome
fi