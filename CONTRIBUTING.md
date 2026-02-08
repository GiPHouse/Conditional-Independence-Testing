# Contributing to CI Testing Library

This document provides guidelines and instructions for contributing to the CI Testing library.

## Table of Contents

- [Development Setup](#development-setup)
- [Repository Structure](#repository-structure)
- [Coding Standards](#coding-standards)
- [Adding a New CI Test](#adding-a-new-ci-test)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Questions and Support](#questions-and-support)

## Development Setup

### Prerequisites

- **Rust** (stable, latest version): [Install Rust](https://rustup.rs/)
- **Git**: For version control
- **Python 3.8+** (for Python bindings development)
- **R 4.0+** (for R bindings development)
- **Node.js 16+** (for JavaScript bindings development)

### Initial Setup

1. **Clone the repository**:
```bash
   git clone https://github.com/GiPHouse/Conditional-Independence-Testing
   cd ci-testing
```

2. **Build the workspace**:
```bash
   cargo build --workspace
```

3. **Run tests to verify setup**:
```bash
   cargo test --workspace
```

4. **Install development tools**:
```bash
   # Rustfmt (code formatter)
   rustup component add rustfmt
   
   # Clippy (linter)
   rustup component add clippy
```

### Verify Your Setup

Run these commands to ensure everything works:
```bash
# Check formatting
cargo fmt --all -- --check

# Run linter
cargo clippy --workspace --all-targets -- -D warnings

# Run all tests
cargo test --workspace

# Run benchmarks
cargo bench --workspace
```

If all commands complete successfully, your environment is ready!

## Repository Structure
```
ci-testing/
├── crates/              # Rust workspace
│   ├── ci-core/        # Core CI test implementations
│   ├── ci-python/      # Python bindings (PyO3)
│   ├── ci-r/           # R bindings (extendr)
│   └── ci-js/          # JavaScript bindings (wasm-pack)
├── docs/               # Documentation
│   ├── api/           # Generated API docs
│   ├── guides/        # User and contributor guides
│   └── design/        # Architecture Decision Records (ADRs)
├── examples/          # Usage examples per language
├── tests/             # Cross-language integration tests
├── scripts/           # Build and development utilities
└── .github/           # CI/CD workflows
```

See individual `README.md` files in each directory for more details.

## Coding Standards

### Rust Code Style

We follow the official Rust style guide, enforced by `rustfmt`:

- **Always run `cargo fmt --all` before committing**
- Use 4 spaces for indentation (automatic with rustfmt)
- Maximum line length: 100 characters
- Use trailing commas in multi-line constructs

### Linting

We use Clippy with strict settings:

- **All Clippy warnings must be fixed** before merging
- Run `cargo clippy --workspace --all-targets -- -D warnings`
- If you believe a warning is a false positive, discuss in your PR

### Code Quality

- **Write tests** for all new functionality
- **Document public APIs** with doc comments (`///`)
- **Keep functions focused**: One function should do one thing well
- **Avoid `unwrap()` in library code**: Use proper error handling with `Result` and `?`
- **Use meaningful variable names**: Prefer clarity over brevity

### Git Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) format:
```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Adding or updating tests
- `refactor`: Code restructuring without behavior change
- `perf`: Performance improvements
- `chore`: Maintenance tasks

**Examples**:
```
feat(core): add Student's t-test implementation

fix(python): resolve memory leak in data conversion

docs(readme): update installation instructions

test(core): add property tests for chi-squared test
```

## Adding a New CI Test

Follow these steps to add a new conditional independence test:

### 1. Create Test Implementation

Create a new file in `crates/ci-core/src/tests/`:
```bash
crates/ci-core/src/tests/students_t.rs
```

### 2. Implement the `CITest` Trait

Your test must implement the `CITest` trait defined in `crates/ci-core/src/strategy.rs`.

See existing tests (e.g., `chi_squared.rs`) as examples.

### 3. Register the Test

Add your test to the registry in `crates/ci-core/src/registry.rs`.

### 4. Add Tests

Create test cases in `crates/ci-core/tests/integration/`:

- Test with known inputs and expected outputs
- Test edge cases (empty data, NaN values, etc.)
- Add property-based tests if applicable

### 6. Update Documentation

- Add doc comments to your test struct and methods
- Update `docs/guides/available-tests.md` with test description
- Add usage examples in `examples/`

## Testing

### Running Tests
```bash
# Run all tests
cargo test --workspace

# Run tests for specific crate
cargo test -p ci-core

# Run a specific test
cargo test test_chi_squared

# Run with output (see println! statements)
cargo test -- --nocapture
```

### Test Organization

- **Unit tests**: In the same file as the code, in a `#[cfg(test)] mod tests { }` block
- **Integration tests**: In `crates/*/tests/` directories
- **Cross-language tests**: In `tests/` at repository root

### Writing Tests

- Use descriptive test names: `test_chi_squared_with_independent_variables`
- Test both success and error cases
- Use property-based testing (proptest) for mathematical properties
- Add regression tests for bugs you fix

## Benchmarking

We use [Criterion](https://github.com/criterion-rs/criterion.rs) for benchmarking:
```bash
# Run all benchmarks
cargo bench --workspace

# Run specific benchmark
cargo bench --bench test_performance
```

Benchmark results are saved in `target/criterion/` and include:

- Statistical analysis of performance
- Comparison with previous runs
- HTML reports

**Performance expectations**:
- CI tests should handle 10,000 samples in < 100ms
- Memory usage should scale linearly with data size
- No memory leaks (verify with valgrind or similar)

## Pull Request Process

### Before Opening a PR

1. **Create a feature branch** from `main`:
```bash
   git checkout -b feature/your-feature-name
```

2. **Make your changes** and commit with clear messages

3. **Ensure all checks pass locally**:
```bash
   cargo fmt --all
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   cargo bench --workspace
```

4. **Push your branch**:
```bash
   git push origin feature/your-feature-name
```

### Opening the PR

1. Go to GitHub and create a Pull Request from your branch to `main`

2. **Fill out the PR template** (auto-generated):
   - Describe what the PR does
   - Link related issues
   - List breaking changes (if any)
   - Checklist: tests added, docs updated, benchmarks run

3. **Request review** from at least one team member

### After Opening

- **CI must pass**: GitHub Actions will run all checks automatically
- **Address review comments**: Make changes and push new commits
- **Keep PR updated**: Rebase on `main` if needed to resolve conflicts

### Merging

- PRs require at least one approval from a team member
- All CI checks must pass (green checkmarks)
- We use **squash merging**: Multiple commits become one clean commit on `main`
- Delete your feature branch after merging


## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
- [Criterion Documentation](https://bheisler.github.io/criterion.rs/book/) - Benchmarking
- [PyO3 Guide](https://pyo3.rs/) - Python bindings