# Contributing to WindWarden

Thank you for your interest in contributing to WindWarden! This document provides guidelines and information for contributors.

## Development Setup

### Prerequisites

WindWarden uses [Nix](https://nixos.org/) for reproducible development environments. Install Nix and then:

```bash
# Clone the repository
git clone https://github.com/benduggan/windwarden
cd windwarden

# Enter the development environment
nix-shell

# Or with direnv (recommended for automatic environment loading)
direnv allow
```

The Nix environment provides:
- Rust toolchain (rustc, cargo, rustfmt, clippy)
- rust-analyzer for IDE support
- Required system dependencies (clang, pkg-config)

### Building and Testing

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run all tests
cargo test

# Run integration tests
cargo test --test integration

# Run benchmarks
cargo bench

# Format code
cargo fmt

# Lint code
cargo clippy

# Check for common issues
cargo clippy -- -D warnings
```

## Project Structure

```
src/
├── lib.rs              # Library entry point
├── main.rs             # CLI entry point
├── cli/                # Command-line interface
├── config.rs           # Configuration handling
├── parser/             # AST parsing with Oxc
├── sorter/             # Core sorting logic
├── processor/          # File processing
├── output/             # Output formatting and reporting
└── utils/              # Shared utilities

tests/
├── fixtures/           # Test files
├── integration/        # Integration tests
└── ...

docs/                   # Documentation
benches/                # Performance benchmarks
```

## Making Changes

### Workflow

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature/my-feature`
3. **Make** your changes
4. **Test** your changes thoroughly
5. **Format** and **lint** your code
6. **Commit** with a clear message
7. **Push** to your fork
8. **Create** a pull request

### Code Style

We follow standard Rust conventions:

- Run `cargo fmt` before committing
- Run `cargo clippy` and address all warnings
- Use meaningful variable and function names
- Add documentation for public APIs
- Write tests for new functionality

### Commit Messages

Use clear, descriptive commit messages:

```
feat: add support for Vue.js single-file components
fix: handle nested template literals correctly
docs: update installation instructions
test: add tests for edge cases in array parsing
```

## Testing

### Unit Tests

```bash
# Run all unit tests
cargo test

# Run tests for specific module
cargo test parser

# Run tests with output
cargo test -- --nocapture
```

### Integration Tests

```bash
# Run integration tests
cargo test --test integration

# Test specific functionality
cargo test --test integration cli_tests
```

### Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench performance
```

### Test Fixtures

When adding new features, include test cases in the appropriate fixture files:

- `tests/fixtures/` - Basic test files

## Documentation

### Code Documentation

- Document all public functions and modules
- Use `///` for documentation comments
- Include examples in documentation when helpful

### User Documentation

- Update relevant files in `docs/` when adding features
- Keep the README.md up to date
- Add examples for new functionality

## Performance Considerations

WindWarden is designed for high performance. When contributing:

- Avoid unnecessary allocations in hot paths
- Use `rayon` for parallelizable operations
- Profile changes with `cargo bench`
- Consider memory usage patterns
- Leverage Oxc's arena allocation where possible


## Release Process

For maintainers:

1. Update version in `Cargo.toml`
2. GitHub Actions will build and publish releases

## License

By contributing to WindWarden, you agree that your contributions will be licensed under the MIT License.
