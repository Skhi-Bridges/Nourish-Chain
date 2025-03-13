# PowerShell build script for the social_leptos component on Windows

# Ensure wasm-pack is installed
if (-not (Get-Command wasm-pack -ErrorAction SilentlyContinue)) {
    Write-Host "wasm-pack not found. Installing..."
    cargo install wasm-pack
}

# Create output directories if they don't exist
if (-not (Test-Path -Path "pkg/web")) {
    New-Item -ItemType Directory -Path "pkg/web" -Force
}

if (-not (Test-Path -Path "pkg/bundler")) {
    New-Item -ItemType Directory -Path "pkg/bundler" -Force
}

# Build for web target (standalone usage)
Write-Host "Building for web target..."
wasm-pack build --target web --out-dir pkg/web

# Build for bundler target (Next.js integration)
Write-Host "Building for bundler target..."
wasm-pack build --target bundler --out-dir pkg/bundler

# Generate TypeScript definitions
Write-Host "Copying TypeScript definitions..."
Copy-Item -Path "next.d.ts" -Destination "pkg/bundler/" -Force

Write-Host "Build completed successfully!"
Write-Host "Web build: ./pkg/web"
Write-Host "Next.js build: ./pkg/bundler"
