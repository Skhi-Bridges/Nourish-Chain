# PowerShell script to ensure source files are pushed to GitHub

# Navigate to the social_leptos directory
Set-Location -Path "C:\Users\skhib\Desktop\NourishChain-Organized\demo\social_leptos"

# Check what's in the src directory
Write-Host "Source files that should be pushed:"
Get-ChildItem -Path "src" -Recurse | ForEach-Object { $_.FullName }

# Make sure git is tracking the src directory
Write-Host "Explicitly adding src directory..."
git add src --force

# Force add all source files
Write-Host "Force adding all source files..."
Get-ChildItem -Path "src" -Recurse | ForEach-Object {
    git add $_.FullName --force
    Write-Host "Added: $($_.FullName)"
}

# Also add the other files
git add nextjs_integration_example.jsx --force
git add tailwind.config.js --force

# Commit the changes
git commit -m "Add all source files to repository"

# Double check what's being tracked
Write-Host "Files now being tracked by git:"
git ls-files

# Push to GitHub
Write-Host "Pushing all files including source code to GitHub..."
git push -u origin main

Write-Host "All source files have been pushed to the repository!"
