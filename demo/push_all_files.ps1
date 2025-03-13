# PowerShell script to ensure ALL files are pushed to GitHub

# Navigate to the social_leptos directory
Set-Location -Path "C:\Users\skhib\Desktop\NourishChain-Organized\demo\social_leptos"

# Make sure we have a clean state
Write-Host "Checking the repository status..."
git status

# Ensure all files are tracked and committed
Write-Host "Adding ALL files to git..."
git add --all

# List all files being tracked
Write-Host "Files being tracked by git:"
git ls-files

# Commit any new changes
git commit -m "Add all project files including source code and scripts"

# Check to make sure we have everything
Write-Host "Verifying git status after commit..."
git status

# Remove any existing origin
git remote remove origin 2>$null

# Add the specified repository as origin
git remote add origin "https://github.com/Skhi-Bridges/nourish-chain-supper-club.git"
git branch -M main

# Force push to ensure everything goes up
Write-Host "Force pushing ALL files to https://github.com/Skhi-Bridges/nourish-chain-supper-club.git..."
git push -u origin main --force

Write-Host "All files have been pushed to the Nourish Chain Supper Club repository!"
Write-Host "Submission is complete!"
