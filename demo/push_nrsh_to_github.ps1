# PowerShell script to push NRSH to GitHub

# Navigate to the NRSH directory
Set-Location -Path "C:\Users\skhib\Desktop\NourishChain-Organized\demo\social_leptos"

# Create a README section specifically for the submission
$readmeAddition = @"

## NRSH Submission - March 5, 2025

This is the official submission for the Nourish Chain Social Hub component. Key features include:

- **Fully Type-Safe Implementation**: Built entirely in Rust with Leptos
- **WebAssembly Integration**: Seamless integration with Next.js
- **Community Dashboard**: Real-time metrics and statistics
- **Social Media Integration**: Twitter, Discord, Telegram feeds
- **Interactive Elements**: Community engagement features
- **Responsive Design**: Works on all devices
- **Dark Mode Support**: Toggleable light/dark themes

All code is production-ready and follows best practices for performance and maintainability.
"@

# Add to the README
Add-Content -Path "README.md" -Value $readmeAddition

# Commit the README update
git add README.md
git commit -m "Update README for official NRSH submission"

# Ask for GitHub username and repository name
Write-Host "To push to GitHub, please provide the following information:"
$githubUsername = Read-Host -Prompt "Your GitHub username"
$repoName = Read-Host -Prompt "Repository name (default: nourish-chain-social-hub)"

if ([string]::IsNullOrWhiteSpace($repoName)) {
    $repoName = "nourish-chain-social-hub"
}

# Set up the remote and push
git remote add origin "https://github.com/$githubUsername/$repoName.git"
git branch -M main
git push -u origin main

Write-Host "NRSH has been pushed to GitHub at: https://github.com/$githubUsername/$repoName"
Write-Host "Submission is complete!"
