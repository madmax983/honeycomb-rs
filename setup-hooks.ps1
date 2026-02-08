# Setup script for pre-commit hooks (Windows PowerShell)

Write-Host "🔧 Setting up pre-commit hooks for honeycomb-rs..." -ForegroundColor Cyan

# Check if pre-commit is installed
if (-not (Get-Command pre-commit -ErrorAction SilentlyContinue)) {
    Write-Host "📦 Installing pre-commit..." -ForegroundColor Yellow
    pip install pre-commit
}

# Install the git hooks
Write-Host "⚙️  Installing git hooks..." -ForegroundColor Yellow
pre-commit install

# Run hooks against all files to verify setup
Write-Host "✅ Running hooks against all files..." -ForegroundColor Yellow
pre-commit run --all-files

Write-Host "✨ Pre-commit hooks setup complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Your commits will now automatically run:" -ForegroundColor White
Write-Host "  - cargo fmt" -ForegroundColor Gray
Write-Host "  - cargo clippy (with pedantic/nursery lints)" -ForegroundColor Gray
Write-Host "  - cargo test" -ForegroundColor Gray
Write-Host ""
Write-Host "To skip hooks temporarily: git commit --no-verify" -ForegroundColor DarkGray
