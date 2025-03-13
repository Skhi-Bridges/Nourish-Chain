# PowerShell script to create KOMBECH variant from social_leptos with golden yellow theme

# Create a copy of the social_leptos directory
$sourceDir = "C:\Users\skhib\Desktop\NourishChain-Organized\demo\social_leptos"
$targetDir = "C:\Users\skhib\Desktop\NourishChain-Organized\demo\kombech_leptos"

Write-Host "Creating KOMBECH variant with golden yellow theme..."

# Remove target directory if it exists
if (Test-Path $targetDir) {
    Write-Host "Removing existing kombech_leptos directory..."
    Remove-Item -Path $targetDir -Recurse -Force
}

# Copy all files from social_leptos to kombech_leptos
Write-Host "Copying files from social_leptos to kombech_leptos..."
Copy-Item -Path $sourceDir -Destination $targetDir -Recurse

# Update Cargo.toml
$cargoPath = Join-Path -Path $targetDir -ChildPath "Cargo.toml"
Write-Host "Updating Cargo.toml..."
(Get-Content -Path $cargoPath) -replace "social_leptos", "kombech_leptos" | Set-Content -Path $cargoPath

# Update type_safe_components.rs
$typeSafeComponentsPath = Join-Path -Path $targetDir -ChildPath "src\type_safe_components.rs"
Write-Host "Updating color scheme in type_safe_components.rs..."
$typeSafeComponents = Get-Content -Path $typeSafeComponentsPath -Raw
$typeSafeComponents = $typeSafeComponents -replace "green", "yellow"
$typeSafeComponents = $typeSafeComponents -replace "emerald", "amber"
Set-Content -Path $typeSafeComponentsPath -Value $typeSafeComponents

# Modify the additional_components.rs to update gauge colors
$additionalComponentsPath = Join-Path -Path $targetDir -ChildPath "src\additional_components.rs"
Write-Host "Updating gauges in additional_components.rs..."
$additionalComponents = Get-Content -Path $additionalComponentsPath -Raw
$additionalComponents = $additionalComponents -replace "bg-green", "bg-yellow"
$additionalComponents = $additionalComponents -replace "text-green", "text-yellow"
Set-Content -Path $additionalComponentsPath -Value $additionalComponents

# Update lib.rs to change branding
$libPath = Join-Path -Path $targetDir -ChildPath "src\lib.rs"
Write-Host "Updating branding in lib.rs..."
$libContent = Get-Content -Path $libPath -Raw
$libContent = $libContent -replace "Nourish Chain Community", "KOMBECH Community"
$libContent = $libContent -replace "Nourish Chain", "KOMBECH"
Set-Content -Path $libPath -Value $libContent

# Update README.md
$readmePath = Join-Path -Path $targetDir -ChildPath "README.md"
Write-Host "Updating README.md..."
$readmeContent = Get-Content -Path $readmePath -Raw
$readmeContent = $readmeContent -replace "Nourish Chain", "KOMBECH"
$readmeContent = $readmeContent -replace "green", "golden yellow"
$readmeContent = $readmeContent -replace "social_leptos", "kombech_leptos"
Set-Content -Path $readmePath -Value $readmeContent

# Create a special golden styling for KOMBECH
$customCssPath = Join-Path -Path $targetDir -ChildPath "src\golden_theme.rs"
Write-Host "Creating special golden theme for KOMBECH..."
$goldenThemeContent = @"
use crate::type_safe_components::{TailwindClass, ResponsiveClass};

pub fn golden_gradient() -> TailwindClass {
    TailwindClass::new()
        .add("bg-gradient-to-r")
        .add("from-yellow-500")
        .add("to-amber-400")
}

pub fn golden_button() -> TailwindClass {
    TailwindClass::new()
        .add("bg-amber-500")
        .add("hover:bg-amber-600")
        .add("text-white")
        .add("font-bold")
        .add("rounded-lg")
        .add("transition-colors")
        .add("duration-300")
}

pub fn golden_text() -> TailwindClass {
    TailwindClass::new()
        .add("font-bold")
        .add("text-amber-600")
        .add("dark:text-yellow-400")
}

pub fn kombech_theme(dark_mode: bool) -> TailwindClass {
    let mut class = TailwindClass::new();
    if dark_mode {
        class.add("bg-gray-900").add("text-yellow-100");
    } else {
        class.add("bg-amber-50").add("text-gray-900");
    }
    class
}
"@
Set-Content -Path $customCssPath -Value $goldenThemeContent

# Run the build script for kombech_leptos
Write-Host "Building KOMBECH component..."
Set-Location -Path $targetDir
powershell -ExecutionPolicy Bypass -File .\build.ps1

Write-Host "KOMBECH variant with golden yellow theme created and built successfully!"
Write-Host "Location: $targetDir"
