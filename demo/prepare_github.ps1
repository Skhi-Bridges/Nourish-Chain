# PowerShell script to prepare the repos for GitHub

# Define the paths to the three variants
$variants = @(
    "C:\Users\skhib\Desktop\NourishChain-Organized\demo\social_leptos",
    "C:\Users\skhib\Desktop\NourishChain-Organized\demo\elxr_leptos",
    "C:\Users\skhib\Desktop\NourishChain-Organized\demo\kombech_leptos"
)

# Create .gitignore file content
$gitignoreContent = @"
/target
/pkg
**/*.rs.bk
Cargo.lock
.DS_Store
node_modules
"@

# Loop through each variant and initialize git
foreach ($variantPath in $variants) {
    $variantName = (Get-Item $variantPath).Name
    Write-Host "Preparing $variantName for GitHub..."
    
    # Navigate to the variant directory
    Set-Location -Path $variantPath
    
    # Create .gitignore
    Write-Host "Creating .gitignore for $variantName..."
    Set-Content -Path ".gitignore" -Value $gitignoreContent
    
    # Initialize git if not already initialized
    if (-Not (Test-Path -Path ".git")) {
        Write-Host "Initializing git for $variantName..."
        git init
    } else {
        Write-Host "Git already initialized for $variantName."
    }
    
    # Add all files
    Write-Host "Adding files to git for $variantName..."
    git add .
    
    # Commit
    Write-Host "Committing files for $variantName..."
    git commit -m "Initial commit for $variantName"
    
    Write-Host "$variantName is ready for pushing to GitHub."
    Write-Host "--------------------------------------------"
}

Write-Host "All variants are prepared for GitHub!"
Write-Host ""
Write-Host "To push to GitHub, for each variant:"
Write-Host "1. Create a new repository on GitHub"
Write-Host "2. Run these commands in each variant directory:"
Write-Host "   git remote add origin https://github.com/your-username/repository-name.git"
Write-Host "   git branch -M main"
Write-Host "   git push -u origin main"
