# WindWarden

> **A blazing fast CLI tool for sorting Tailwind CSS classes**

WindWarden automatically sorts Tailwind CSS classes according to the official recommended order. Built in Rust for maximum performance, it uses AST parsing to handle complex patterns like JSX attributes, utility functions (`cn()`, `clsx()`), template literals, and arrays.

## ✨ Features

- **🚀 Lightning Fast** - Built in Rust with [Oxc parser](https://github.com/oxc-project/oxc) for maximum performance
- **🎯 Smart Parsing** - AST-based parsing handles complex patterns and preserves formatting
- **🔧 Flexible Configuration** - Support for custom sort orders, function names, and patterns
- **📁 Multi-Format Support** - Works with `.js`, `.jsx`, `.ts`, `.tsx`, `.vue`, `.svelte`, and more
- **⚡ Parallel Processing** - Multi-threaded file processing for large codebases
- **🛡️ Safe Operations** - Preserves quotes, indentation, and original formatting

## 🚀 Quick Start

### Installation

```bash
# Download from GitHub releases
curl -L https://github.com/benduggan/windwarden/releases/latest/download/windwarden-linux-x86_64 -o windwarden
chmod +x windwarden
sudo mv windwarden /usr/local/bin/

# Or build from source
cargo install --git https://github.com/benduggan/windwarden
```

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
| **[Usage Guide](docs/USAGE.md)** | Comprehensive usage examples and CLI options |
| **[Performance Guide](docs/PERFORMANCE.md)** | Performance benchmarks and optimization tips |
| **[Architecture](docs/ARCHITECTUREPLAN.md)** | Technical architecture and design decisions |
| **[Git Integration](docs/GIT_INTEGRATION.md)** | Git hooks, CI/CD setup, and automation |
| **[Shell Completions](docs/COMPLETIONS.md)** | Shell completion setup for bash, zsh, fish |
| **[Custom Sorting](docs/CUSTOM_SORTING_DEMO.md)** | Custom sort orders and configuration |
| **[Project Plan](docs/PROJECTPLAN.md)** | Development roadmap and feature planning |
| **[Resources](docs/RESOURCES.md)** | Additional resources and references |

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

## 🚦 Exit Codes

- `0` - Success (no formatting needed or changes applied successfully)
- `1` - Files need formatting (when using `check` command)
- `2` - Error occurred during processing

## 🤝 Contributing

Contributions are welcome! Please see our [development docs](docs/) for technical details.

## 📄 License

MIT License - see LICENSE file for details.

---

**[View Full Documentation →](docs/)**