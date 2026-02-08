# Contributing to honeycomb-rs

Thanks for your interest in contributing! This document provides guidelines and information for contributors.

## Development Setup

1. **Install Rust** (latest stable):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the repository**:
   ```bash
   git clone https://github.com/madmax983/honeycomb-rs.git
   cd honeycomb-rs
   ```

3. **Install pre-commit hooks** (recommended):
   ```bash
   # Linux/macOS
   ./setup-hooks.sh

   # Windows PowerShell
   .\setup-hooks.ps1
   ```

## Code Standards

We follow strict quality gates to ensure code quality:

### Test-Driven Development (TDD)
- Write failing tests first (RED)
- Minimal implementation to pass (GREEN)
- Clean up while tests stay green (REFACTOR)
- Target: **85-90% test coverage minimum**

### Rust Standards
- Run `cargo fmt` before every commit
- Pass `cargo clippy` with pedantic/nursery lints
- No `unwrap()` in production code (document exceptions in comments)
- Use `thiserror` for library errors, `anyhow` for binary errors
- Strong typing: prefer newtypes for IDs over raw primitives

### Quality Gates (enforced by CI)
1. **Format check**: `cargo fmt --all -- --check`
2. **Clippy**: `cargo clippy --all-targets --all-features`
3. **Tests**: `cargo test --all-features`
4. **Coverage**: Must meet 85% threshold

## Pull Request Process

1. **Create a feature branch**: `git checkout -b feature/my-feature`
2. **Write tests first**: Follow TDD approach
3. **Implement your changes**: Keep it focused and minimal
4. **Run quality checks**:
   ```bash
   cargo fmt
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all-features
   cargo tarpaulin --all-features --workspace
   ```
5. **Update documentation**: If adding/changing public APIs
6. **Update CHANGELOG.md**: Under `[Unreleased]` section
7. **Submit PR**: Fill out the PR template completely

## Testing

```bash
# Run all tests
cargo test --all-features

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Check coverage
cargo tarpaulin --all-features --workspace --out Html

# Mutation testing (validate test quality)
./run-mutants.sh       # Linux/macOS
.\run-mutants.ps1      # Windows
```

### Mutation Testing

We use [cargo-mutants](https://mutants.rs/) to validate test effectiveness. Mutation testing introduces deliberate bugs and verifies that tests catch them:

- **Required**: 80% mutation score minimum
- **Run locally**: `./run-mutants.sh` or `.\run-mutants.ps1`
- **CI**: Runs on PRs and weekly
- **Results**: Check `mutants.out/missed.txt` for uncaught mutants

If tests miss mutants:
1. Review `mutants.out/missed.txt` for the specific mutations
2. Add tests that exercise those code paths
3. Focus on edge cases, error conditions, and boundary values

## Benchmarks

For performance-critical code, add benchmarks using [criterion](https://github.com/bheisler/criterion.rs):

```bash
cargo bench
```

## Documentation

```bash
# Generate and open docs
cargo doc --all-features --open

# Check docs build without warnings
RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps
```

## Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): subject

body

footer
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`, `ci`

Examples:
- `feat(client): add retry logic with exponential backoff`
- `fix(config): handle missing environment variables gracefully`
- `docs: update README with installation instructions`

## Getting Help

- Open an [issue](https://github.com/madmax983/honeycomb-rs/issues) for bugs
- Start a [discussion](https://github.com/madmax983/honeycomb-rs/discussions) for questions
- Review existing issues before creating new ones

## License

By contributing, you agree that your contributions will be dual-licensed under the MIT and Apache-2.0 licenses.
