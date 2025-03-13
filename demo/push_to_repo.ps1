# PowerShell script to push to the specified GitHub repository

# Navigate to the social_leptos directory
Set-Location -Path "C:\Users\skhib\Desktop\NourishChain-Organized\demo\social_leptos"

# Update the README with Supper Club concept
$readmeAddition = @"

## Nourish Chain Supper Club - March 5, 2025

This is the official submission for the Nourish Chain Supper Club component. The Supper Club is a community hub bringing people together around food security and sustainable agriculture.

### Key Features:
- **Impact Metrics Dashboard**: Visualize food security improvements
- **Harvest Tracking**: Monitor community garden and farm outputs
- **Meal Planning**: Community-driven meal planning tools
- **Resource Sharing**: Platform for sharing agricultural knowledge
- **Community Engagement**: Tools for organizing local food events

Built with Rust, Leptos, and WebAssembly for seamless Next.js integration.
"@

# Update README if it doesn't already contain the supper club info
$readmeContent = Get-Content -Path "README.md" -Raw
if (-not ($readmeContent -match "Supper Club")) {
    Add-Content -Path "README.md" -Value $readmeAddition
    git add README.md
    git commit -m "Update README for Nourish Chain Supper Club concept"
}

# Configure git if needed
git config --local user.name "Skhi-Bridges"
git config --local user.email "user@example.com"  # Replace with actual email if available

# Remove any existing origin
git remote remove origin 2>$null

# Add the specified repository as origin
git remote add origin "https://github.com/Skhi-Bridges/nourish-chain-supper-club.git"
git branch -M main

# Push to the repository
Write-Host "Pushing to https://github.com/Skhi-Bridges/nourish-chain-supper-club.git..."
git push -u origin main

Write-Host "Code has been pushed to the Nourish Chain Supper Club repository!"
Write-Host "Submission is complete!"
