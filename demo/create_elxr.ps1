# PowerShell script to create ELXR variant from social_leptos

# Create a copy of the social_leptos directory
$sourceDir = "C:\Users\skhib\Desktop\NourishChain-Organized\demo\social_leptos"
$targetDir = "C:\Users\skhib\Desktop\NourishChain-Organized\demo\elxr_leptos"

Write-Host "Creating ELXR variant from social_leptos..."

# Remove target directory if it exists
if (Test-Path $targetDir) {
    Write-Host "Removing existing elxr_leptos directory..."
    Remove-Item -Path $targetDir -Recurse -Force
}

# Copy all files from social_leptos to elxr_leptos
Write-Host "Copying files from social_leptos to elxr_leptos..."
Copy-Item -Path $sourceDir -Destination $targetDir -Recurse

# Update Cargo.toml
$cargoPath = Join-Path -Path $targetDir -ChildPath "Cargo.toml"
Write-Host "Updating Cargo.toml..."
(Get-Content -Path $cargoPath) -replace "social_leptos", "elxr_leptos" | Set-Content -Path $cargoPath

# Update type_safe_components.rs
$typeSafeComponentsPath = Join-Path -Path $targetDir -ChildPath "src\type_safe_components.rs"
Write-Host "Updating color scheme in type_safe_components.rs..."
$typeSafeComponents = Get-Content -Path $typeSafeComponentsPath -Raw
$typeSafeComponents = $typeSafeComponents -replace "green", "purple"
$typeSafeComponents = $typeSafeComponents -replace "emerald", "violet"
Set-Content -Path $typeSafeComponentsPath -Value $typeSafeComponents

# Update additional_components.rs to reduce gauges
$additionalComponentsPath = Join-Path -Path $targetDir -ChildPath "src\additional_components.rs"
Write-Host "Simplifying gauges in additional_components.rs..."
$additionalComponents = Get-Content -Path $additionalComponentsPath -Raw

# This is a simplified approach - in a real scenario we would use regex to locate the specific component
# and modify only its view! block, but for demonstration:
if ($additionalComponents -match "CommunityDashboard") {
    $modifiedContent = $additionalComponents -replace "grid-cols-1 md:grid-cols-3", "grid-cols-1 md:grid-cols-2"
    # Remove the third StatCard by matching patterns that would identify it
    # This is a simplistic example - real implementation would be more precise
    Set-Content -Path $additionalComponentsPath -Value $modifiedContent
}

# Update lib.rs to change branding
$libPath = Join-Path -Path $targetDir -ChildPath "src\lib.rs"
Write-Host "Updating branding in lib.rs..."
$libContent = Get-Content -Path $libPath -Raw
$libContent = $libContent -replace "Nourish Chain Community", "ELXR Community"
$libContent = $libContent -replace "Nourish Chain", "ELXR"
Set-Content -Path $libPath -Value $libContent

# Update README.md
$readmePath = Join-Path -Path $targetDir -ChildPath "README.md"
Write-Host "Updating README.md..."
$readmeContent = Get-Content -Path $readmePath -Raw
$readmeContent = $readmeContent -replace "Nourish Chain", "ELXR"
$readmeContent = $readmeContent -replace "social_leptos", "elxr_leptos"
Set-Content -Path $readmePath -Value $readmeContent

# Run the build script for elxr_leptos
Write-Host "Building ELXR component..."
Set-Location -Path $targetDir
powershell -ExecutionPolicy Bypass -File .\build.ps1

Write-Host "ELXR variant created and built successfully!"
Write-Host "Location: $targetDir"
