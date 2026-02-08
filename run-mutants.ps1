# Run mutation testing locally with cargo-mutants

Write-Host "🧬 Running mutation testing with cargo-mutants..." -ForegroundColor Cyan
Write-Host ""

# Check if cargo-mutants is installed
if (-not (Get-Command cargo-mutants -ErrorAction SilentlyContinue)) {
    Write-Host "📦 Installing cargo-mutants..." -ForegroundColor Yellow
    cargo install cargo-mutants --locked
}

# Run mutation testing
Write-Host "🔬 Testing mutants (this may take a while)..." -ForegroundColor Yellow
cargo mutants --all-features --output mutants.out

# Parse results
$mutantsOut = Get-Content mutants.out/mutants.out.txt -Raw
$caught = if ($mutantsOut -match 'caught (\d+)') { [int]$Matches[1] } else { 0 }
$total = if ($mutantsOut -match 'total (\d+)') { [int]$Matches[1] } else { 1 }
$score = [math]::Round(($caught / $total) * 100, 1)

Write-Host ""
Write-Host "📊 Results:" -ForegroundColor White
Write-Host "  - Mutation Score: $score%" -ForegroundColor White
Write-Host "  - Caught: $caught/$total mutants" -ForegroundColor White
Write-Host ""

if ($score -lt 80.0) {
    Write-Host "❌ Mutation score $score% is below 80% threshold" -ForegroundColor Red
    Write-Host "📝 Review mutants.out/missed.txt for uncaught mutants" -ForegroundColor Yellow
    Write-Host "💡 Tip: Add tests that exercise edge cases and error paths" -ForegroundColor Cyan
    exit 1
}

Write-Host "✅ Mutation score $score% meets threshold!" -ForegroundColor Green
Write-Host "🎉 Your tests are catching mutations effectively" -ForegroundColor Green
