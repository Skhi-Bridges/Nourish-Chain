#!/bin/bash
# Build script for the social_leptos component

# Ensure wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack not found. Installing..."
    cargo install wasm-pack
fi

# Build for web target (standalone usage)
echo "Building for web target..."
wasm-pack build --target web --out-dir pkg/web

# Build for bundler target (Next.js integration)
echo "Building for bundler target..."
wasm-pack build --target bundler --out-dir pkg/bundler

# Generate TypeScript definitions
echo "Copying TypeScript definitions..."
cp next.d.ts pkg/bundler/

echo "Build completed successfully!"
echo "Web build: ./pkg/web"
echo "Next.js build: ./pkg/bundler"
