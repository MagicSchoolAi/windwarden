# WindWarden

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **A blazing fast CLI tool for sorting Tailwind CSS classes**

WindWarden automatically sorts Tailwind CSS classes according to the official recommended order. Built in Rust for maximum performance, it uses AST parsing to handle complex patterns like JSX attributes, utility functions (`cva()`, `clsx()`), template literals, and arrays.

## ✨ Features

- **🚀 Lightning Fast** - Built in Rust with [Oxc parser](https://github.com/oxc-project/oxc) for maximum performance
- **🎯 Smart Parsing** - AST-based parsing handles complex patterns and preserves formatting
- **🔧 Flexible Configuration** - Support for custom sort orders, function names, and patterns
- **📁 Multi-Format Support** - Works with `.js`, `.jsx`, `.ts`, `.tsx` files
- **⚡ Parallel Processing** - Multi-threaded file processing for large codebases
- **🛡️ Safe Operations** - Preserves quotes, indentation, and original formatting

## 🚀 Quick Start

### Installation

#### From GitHub Releases (Recommended)

Download pre-built binaries for your platform

### Basic Usage

```bash
# Format files in place
windwarden format --mode write src/

# Check if files need formatting (CI/CD)
windwarden check src/

# Preview changes without writing
windwarden format --mode check src/

# Process from stdin
echo '<div className="p-4 flex m-2">' | windwarden --stdin
```

## 🎯 What It Does

**Before:**
```jsx
<div className="p-4 bg-red-500 flex justify-center items-center m-2 text-white">
  <span className="font-bold text-lg p-2">Hello</span>
</div>
```

**After:**
```jsx
<div className="flex items-center justify-center m-2 p-4 bg-red-500 text-white">
  <span className="p-2 text-lg font-bold">Hello</span>
</div>
```

## 📖 Documentation

| Document | Description |
|----------|-------------|
| **[Usage Guide](docs/USAGE.md)** | Complete command reference and quick start |
| **[Getting Started](docs/guides/getting-started.md)** | Beginner-friendly setup and basic usage |
| **[Advanced Usage](docs/guides/advanced-usage.md)** | Complex patterns and power user features |
| **[Configuration](docs/guides/configuration.md)** | Complete configuration options and setup |
| **[Troubleshooting](docs/guides/troubleshooting.md)** | Common issues and debug tips |
| **[Git Integration](docs/GIT_INTEGRATION.md)** | Git hooks, CI/CD setup, and automation |
| **[Shell Completions](docs/COMPLETIONS.md)** | Shell completion setup for bash, zsh, fish |
| **[Performance Guide](docs/PERFORMANCE.md)** | Performance benchmarks and optimization tips |
| **[Custom Sorting](docs/CUSTOM_SORTING_DEMO.md)** | Custom sort orders and configuration |

## 🔧 Supported Patterns

WindWarden handles all common Tailwind CSS patterns:

```jsx
// JSX className
<div className="flex items-center p-4" />

// Utility functions
cn("flex items-center", condition && "p-4")
clsx("flex", "items-center", "p-4")
twMerge("flex items-center", "p-4")

// Template literals
const classes = `flex items-center p-4`
const styled = tw`flex items-center p-4`

// Arrays and CVA patterns
cva(["flex", "items-center"], { variants: { ... } })
```

## ⚙️ Configuration

Create a `.windwarden.json` config file:

```json
{
  "sortOrder": "official",
  "functionNames": ["cn", "clsx", "twMerge"],
  "fileExtensions": ["tsx", "jsx", "ts", "js"],
  "ignorePaths": ["node_modules", "dist"]
}
```

## ✅ Check Mode

The `check` command is a convenient alias for `format --mode verify`:

```bash
# These are equivalent:
windwarden check src/
windwarden format --mode verify src/

# Both exit with code 1 if files need formatting
```

## 🚦 Exit Codes

- `0` - Success (no formatting needed or changes applied successfully)
- `1` - Files need formatting (when using `check` command)
- `2` - Error occurred during processing

## 🛠️ Development

### Prerequisites

WindWarden uses [Nix](https://nixos.org/) for development environment management:

```bash
# Enter the development environment
nix-shell

# Or with direnv (recommended)
direnv allow
```

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Performance Testing

```bash
# Run performance benchmarks
cargo bench --bench performance

# Compare optimization strategies
cargo bench --bench optimization_comparison
```

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Before submitting a PR:
1. Run `cargo fmt` to format code
2. Run `cargo clippy` to check for issues
3. Run `cargo test` to ensure tests pass
4. Add tests for new features

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**[View Full Documentation →](docs/)**
