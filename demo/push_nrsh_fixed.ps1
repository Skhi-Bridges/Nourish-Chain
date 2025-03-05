# PowerShell script to push NRSH to GitHub (fixed version)

# Navigate to the NRSH directory
Set-Location -Path "C:\Users\skhib\Desktop\NourishChain-Organized\demo\social_leptos"

# Commit the README update (skip if already committed)
if (git status --porcelain | Select-String "README.md") {
    git add README.md
    git commit -m "Update README for official NRSH submission"
}

# Ask for GitHub username and repository name
Write-Host "To push to GitHub, please provide the following information:"
$githubUsername = Read-Host -Prompt "Your GitHub username"
$repoName = Read-Host -Prompt "Repository name (use hyphens instead of spaces, e.g. nourish-chain-social-hub)"

# Validate repository name (replace spaces with hyphens)
$repoName = $repoName -replace "\s+", "-"
if ([string]::IsNullOrWhiteSpace($repoName)) {
    $repoName = "nourish-chain-social-hub"
}

# Remove any existing remote named 'origin'
git remote remove origin 2>$null

# Set up the remote and push
Write-Host "Setting up remote repository: https://github.com/$githubUsername/$repoName.git"
git remote add origin "https://github.com/$githubUsername/$repoName.git"
git branch -M main

# Attempt to push
Write-Host "Pushing to GitHub..."
$pushResult = git push -u origin main 2>&1

# Check if push was successful
if ($LASTEXITCODE -eq 0) {
    Write-Host "NRSH has been successfully pushed to GitHub at: https://github.com/$githubUsername/$repoName"
    Write-Host "Submission is complete!"
} else {
    Write-Host "Error pushing to GitHub. Please check the error message above."
    Write-Host "You might need to:"
    Write-Host "1. Create the repository first at https://github.com/new"
    Write-Host "2. Make sure the repository name matches exactly: $repoName"
    Write-Host "3. Check your GitHub credentials"
}
