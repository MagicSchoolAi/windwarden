# WindWarden Usage Guide

WindWarden is a high-performance CLI tool for sorting and organizing Tailwind CSS classes according to the official Tailwind order. It works with React and any framework that uses Tailwind CSS with JavaScript/TypeScript.

## Quick Reference

### Basic Commands

| Command | Description | Example |
|---------|-------------|---------|
| `windwarden format src/` | Preview changes (safe) | See what would be sorted |
| `windwarden format --mode write src/` | Format files in place | Apply changes to files |
| `windwarden format --mode verify .` | Check formatting | Verify files are formatted (CI/CD) |
| `windwarden config init` | Create configuration | Generate `.windwarden.json` |
| `windwarden config show` | Show current config | Debug configuration issues |

### Essential Options

| Option | Description | Example |
|--------|-------------|---------|
| `--mode check` | Preview mode (default) | `windwarden format --mode check src/` |
| `--mode write` | Apply changes | `windwarden format --mode write src/` |
| `--mode verify` | Check formatting | `windwarden format --mode verify .` |
| `--stats` | Show statistics | `windwarden format --stats src/` |
| `--diff` | Show differences | `windwarden format --diff src/` |
| `--exclude` | Skip patterns | `--exclude "node_modules/**,dist/**"` |
| `--extensions` | File types | `--extensions tsx,jsx,ts,js` |
| `--config` | Config file | `--config ./my-config.json` |

## Comprehensive Guides

### 📚 [Getting Started Guide](./guides/getting-started.md)
**Perfect for beginners** - Installation, basic usage, common patterns, and your first formatting session. Start here if you're new to WindWarden.

### ⚡ [Advanced Usage Guide](./guides/advanced-usage.md) 
**For power users** - Complex patterns, custom sorting orders, performance optimization, atomic operations, and integration with build tools.

### ⚙️ [Configuration Guide](./guides/configuration.md)
**Complete configuration reference** - All settings, file format, function recognition, safety features, and configuration validation.

### 🔧 [Troubleshooting Guide](./guides/troubleshooting.md)
**When things go wrong** - Common issues, error messages, debug techniques, performance problems, and getting help.

## Quick Examples

### Before and After

**Before:**
```jsx
<div className="p-4 bg-blue-500 flex items-center m-2 text-white">
  <span className="font-bold text-lg p-2 bg-white text-black">
    Button
  </span>
</div>
```

**After:**
```jsx
<div className="flex items-center m-2 p-4 bg-blue-500 text-white">
  <span className="p-2 bg-white text-lg font-bold text-black">
    Button
  </span>
</div>
```

### Utility Functions

**Before:**
```javascript
const styles = cn("p-4 bg-blue-500 flex", isActive && "ring-2", "text-white")
```

**After:**
```javascript
const styles = cn("flex p-4 bg-blue-500 text-white", isActive && "ring-2")
```

## Supported Patterns

WindWarden automatically detects and sorts classes in:

- **JSX attributes**: `className="..."`, `class="..."`
- **Utility functions**: `cn()`, `clsx()`, `twMerge()`, `classnames()`
- **Template literals**: `tw\`...\``, `css\`...\``
- **Arrays**: CVA patterns, basic arrays
- **Object properties**: `{ className: "..." }`

## File Support

**Supported file types:**
- React: `.tsx`, `.jsx`, `.ts`, `.js`

**Recognition patterns:**
- All quote styles (single, double, backticks)
- Conditional expressions (preserved)
- Dynamic content (skipped safely)
- Nested function calls

## Common Workflows

### Development
```bash
# Check what would change
windwarden format src/

# Format specific component
windwarden format --mode write src/components/Button.tsx

# Format with progress
windwarden format --mode write --progress src/
```

### CI/CD
```bash
# Check if files are formatted (exit code 1 if not)
windwarden format --mode verify .

# Generate JSON report for tooling
windwarden format --mode verify --format json .
```

### Batch Operations
```bash
# Format entire project, excluding build artifacts
windwarden format --mode write --exclude "node_modules/**,dist/**,build/**" .

# Format only TypeScript React files
windwarden format --mode write --extensions tsx,ts src/
```

## Configuration

Create a `.windwarden.json` file to customize behavior:

```json
{
  "sortOrder": "official",
  "functionNames": ["cn", "clsx", "twMerge", "classnames"],
  "fileExtensions": ["tsx", "jsx", "ts", "js"],
  "removeNullClasses": true,
  "preserveDuplicates": false,
  "safety": {
    "atomicWrites": true,
    "createBackups": false,
    "verifyWrites": false
  }
}
```

**Generate default configuration:**
```bash
windwarden config init
```

**Validate configuration:**
```bash
windwarden config validate
```

## Getting Help

1. **Start with guides** - Use the comprehensive guides above for detailed information
2. **Check configuration** - `windwarden config show`
3. **Test single files** - `windwarden format test-file.tsx`
4. **Use debug output** - `windwarden format --stats --progress --diff`
5. **Validate config** - `windwarden config validate`

## Next Steps

- **New users**: Start with [Getting Started Guide](./guides/getting-started.md)
- **Need customization**: Check [Configuration Guide](./guides/configuration.md)  
- **Power user features**: See [Advanced Usage Guide](./guides/advanced-usage.md)
- **Having issues**: Visit [Troubleshooting Guide](./guides/troubleshooting.md)

For installation instructions, see the [main README](../README.md#installation).