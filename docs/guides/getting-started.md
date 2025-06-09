# Getting Started with WindWarden

A quick introduction to WindWarden, the high-performance CLI tool for sorting and organizing Tailwind CSS classes.

## What is WindWarden?

WindWarden automatically sorts your Tailwind CSS classes according to the official Tailwind order, making your code more consistent and readable. It works with React and any framework that uses Tailwind CSS with JavaScript/TypeScript.

## Installation

See the [main README](../../README.md#installation) for installation instructions.

## Quick Start

### Basic Commands

```bash
# Preview what would change (safe - doesn't modify files)
windwarden format src/

# Format files in place
windwarden format --mode write src/

# Check if files are formatted (for CI/CD)
windwarden format --mode verify .
```

### Your First Format

1. **Preview mode** - See what changes would be made:
   ```bash
   windwarden format src/components/Button.tsx
   ```

2. **Format the file** - Apply the changes:
   ```bash
   windwarden format --mode write src/components/Button.tsx
   ```

3. **Format entire directory**:
   ```bash
   windwarden format --mode write src/
   ```

## Common Use Cases

### 1. Format Your Components

Before:
```jsx
<div className="p-4 bg-blue-500 flex items-center m-2 text-white">
  Button
</div>
```

After:
```jsx
<div className="flex items-center m-2 p-4 bg-blue-500 text-white">
  Button
</div>
```

### 2. Sort Utility Function Calls

Before:
```javascript
const buttonClass = cn("p-4 bg-blue-500 flex", isActive && "ring-2")
```

After:
```javascript
const buttonClass = cn("flex p-4 bg-blue-500", isActive && "ring-2")
```

### 3. Clean Up Template Literals

Before:
```javascript
const styles = tw`p-4 bg-blue-500 flex items-center text-white`
```

After:
```javascript
const styles = tw`flex items-center p-4 bg-blue-500 text-white`
```

## Basic Configuration

Create a configuration file to customize WindWarden:

```bash
# Create default configuration
windwarden config init
```

This creates `.windwarden.json` with sensible defaults:

```json
{
  "sortOrder": "official",
  "functionNames": ["cn", "clsx", "twMerge", "classnames"],
  "fileExtensions": ["tsx", "jsx", "ts", "js"]
}
```

## Essential Command Options

| Option | Description | Example |
|--------|-------------|---------|
| `--mode check` | Preview changes | `windwarden format --mode check src/` |
| `--mode write` | Apply changes | `windwarden format --mode write src/` |
| `--mode verify` | Check formatting | `windwarden format --mode verify .` |
| `--stats` | Show statistics | `windwarden format --stats src/` |
| `--diff` | Show differences | `windwarden format --diff src/` |
| `--exclude` | Skip patterns | `--exclude node_modules,dist` |

## File Support

WindWarden works with these file types by default:
- **React**: `.tsx`, `.jsx`, `.ts`, `.js`

And recognizes these patterns:
- JSX `className` and `class` attributes
- Utility functions: `cn()`, `clsx()`, `twMerge()`, `classnames()`
- Template literals: `tw\`...\``
- Array patterns and object properties

## Quick Tips

1. **Start with preview mode** - Always use `--mode check` first to see what changes
2. **Use exclusions** - Skip unnecessary directories: `--exclude "node_modules/**,dist/**"`
3. **Check your config** - Use `windwarden config show` to see current settings
4. **Test single files** - Start with one file to understand the behavior
5. **CI/CD integration** - Use `--mode verify` in automated checks

## Next Steps

- **Advanced usage**: See [Advanced Usage Guide](./advanced-usage.md) for complex patterns and performance tuning
- **Configuration**: Check [Configuration Guide](./configuration.md) for customization options
- **Troubleshooting**: Visit [Troubleshooting Guide](./troubleshooting.md) if you encounter issues

## Examples for Common Frameworks

### React/Next.js
```bash
# Format all React components
windwarden format --mode write src/components/

# Include pages and app directories
windwarden format --mode write src/ pages/ app/
```


### Monorepo
```bash
# Format multiple packages
windwarden format --mode write packages/*/src/

# Exclude build artifacts
windwarden format --mode write . --exclude "**/node_modules/**,**/dist/**,**/build/**"
```

Ready to start formatting? Try the basic commands above on your project!