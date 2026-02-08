#!/bin/bash
# Run mutation testing locally with cargo-mutants

set -e

echo "🧬 Running mutation testing with cargo-mutants..."
echo ""

# Check if cargo-mutants is installed
if ! command -v cargo-mutants &> /dev/null; then
    echo "📦 Installing cargo-mutants..."
    cargo install cargo-mutants --locked
fi

# Run mutation testing
echo "🔬 Testing mutants (this may take a while)..."
cargo mutants --all-features --output mutants.out

# Parse results
CAUGHT=$(grep -oP 'caught \K\d+' mutants.out/mutants.out.txt || echo "0")
TOTAL=$(grep -oP 'total \K\d+' mutants.out/mutants.out.txt || echo "1")
SCORE=$(awk "BEGIN {printf \"%.1f\", ($CAUGHT/$TOTAL)*100}")

echo ""
echo "📊 Results:"
echo "  - Mutation Score: ${SCORE}%"
echo "  - Caught: ${CAUGHT}/${TOTAL} mutants"
echo ""

if (( $(echo "$SCORE < 80.0" | bc -l) )); then
    echo "❌ Mutation score ${SCORE}% is below 80% threshold"
    echo "📝 Review mutants.out/missed.txt for uncaught mutants"
    echo "💡 Tip: Add tests that exercise edge cases and error paths"
    exit 1
fi

echo "✅ Mutation score ${SCORE}% meets threshold!"
echo "🎉 Your tests are catching mutations effectively"
