#!/bin/bash
# Setup script for pre-commit hooks

set -e

echo "🔧 Setting up pre-commit hooks for honeycomb-rs..."

# Check if pre-commit is installed
if ! command -v pre-commit &> /dev/null; then
    echo "📦 Installing pre-commit..."
    pip install pre-commit
fi

# Install the git hooks
echo "⚙️  Installing git hooks..."
pre-commit install

# Run hooks against all files to verify setup
echo "✅ Running hooks against all files..."
pre-commit run --all-files || true

echo "✨ Pre-commit hooks setup complete!"
echo ""
echo "Your commits will now automatically run:"
echo "  - cargo fmt"
echo "  - cargo clippy (with pedantic/nursery lints)"
echo "  - cargo test"
echo ""
echo "To skip hooks temporarily: git commit --no-verify"
