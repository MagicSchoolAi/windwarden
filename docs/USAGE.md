# WindWarden Usage Guide

This comprehensive guide covers all WindWarden features, configuration options, and how they work together.

## Table of Contents

- [Quick Start](#quick-start)
- [Command Reference](#command-reference)
- [Configuration System](#configuration-system)
- [Supported Patterns](#supported-patterns)
- [Sorting Rules](#sorting-rules)
- [Output Formats](#output-formats)
- [Advanced Features](#advanced-features)
- [Integration](#integration)
- [Troubleshooting](#troubleshooting)

## Quick Start

### Installation

```bash
# Install from releases (replace with actual method)
curl -L https://github.com/your-org/windwarden/releases/latest/download/windwarden-linux -o windwarden
chmod +x windwarden
sudo mv windwarden /usr/local/bin/
```

### Basic Usage

```bash
# Check files (preview mode)
windwarden format src/

# Format files in place
windwarden format --mode write src/

# Check if files are formatted (CI/CD)
windwarden format --mode verify .
```

## Command Reference

### Global Options

```bash
windwarden [OPTIONS] [COMMAND]
```

| Option | Description | Example |
|--------|-------------|---------|
| `--stdin` | Process from stdin | `echo '<div className="p-4 flex">' \| windwarden --stdin` |
| `--config FILE` | Configuration file path | `windwarden --config custom.json format src/` |
| `--dry-run` | Preview changes (legacy) | `windwarden --dry-run format src/` |
| `--check-formatted` | Verify formatting (legacy) | `windwarden --check-formatted format src/` |

### Format Command

The main command for processing files:

```bash
windwarden format [OPTIONS] <PATHS>...
```

#### Mode Options

| Mode | Description | Exit Code | Use Case |
|------|-------------|-----------|----------|
| `check` | Preview changes | 0 | Development, seeing what would change |
| `write` | Modify files | 0 on success | Formatting files |
| `verify` | Check formatting | 1 if unformatted | CI/CD, pre-commit hooks |

```bash
# Preview what would change
windwarden format --mode check src/

# Format files in place
windwarden format --mode write src/

# Verify files are formatted (exit 1 if not)
windwarden format --mode verify src/
```

#### Processing Options

| Option | Description | Default | Example |
|--------|-------------|---------|---------|
| `--processing` | Sequential or parallel | `parallel` | `--processing sequential` |
| `--threads N` | Number of threads | Auto-detect | `--threads 4` |
| `--extensions` | File extensions | From config | `--extensions tsx,jsx,ts,js` |
| `--exclude` | Exclude patterns | None | `--exclude node_modules,dist` |
| `--max-depth N` | Directory depth | Unlimited | `--max-depth 3` |
| `--follow-links` | Follow symlinks | false | `--follow-links` |

```bash
# Process only TypeScript files with 2 threads
windwarden format --mode write --extensions tsx,ts --threads 2 src/

# Exclude node_modules and build directories
windwarden format --exclude "node_modules/**,dist/**,build/**" .
```

#### Output Options

| Option | Description | Example |
|--------|-------------|---------|
| `--stats` | Show detailed statistics | `--stats` |
| `--progress` | Show progress bar | `--progress` |
| `--diff` | Show differences | `--diff` |

### Config Command

Manage configuration files:

```bash
# Create default configuration
windwarden config init

# Create config in specific location
windwarden config init --path ./my-config.json

# Show current effective configuration
windwarden config show

# Validate configuration file
windwarden config validate
windwarden config validate ./my-config.json
```

### Check Command

Alias for `format --mode verify`:

```bash
windwarden check [OPTIONS] <PATHS>...
```

Supports the same options as `format` but always operates in verify mode.

## Configuration System

WindWarden uses a hierarchical configuration system that merges settings from multiple sources.

### Configuration Precedence

1. **Command line arguments** (highest priority)
2. **Configuration file** (`--config` or discovered file)
3. **Default values** (lowest priority)

### Configuration Discovery

WindWarden searches for configuration files in this order:

1. `--config` argument path
2. `.windwarden.json` in current directory
3. `.windwarden.json` in parent directories (walking up to root)
4. `~/.windwarden.json` (user home directory)

### Complete Configuration Reference

Create a configuration file with `windwarden config init`:

```json
{
  "sortOrder": "official",
  "customOrder": [],
  "functionNames": ["cn", "clsx", "twMerge", "classnames"],
  "presetRegex": "all",
  "customRegex": [],
  "fileExtensions": ["tsx", "jsx", "ts", "js", "vue", "svelte"],
  "maxFileSize": 1048576,
  "threads": 0,
  "removeNullClasses": true,
  "preserveDuplicates": false,
  "defaultMode": null,
  "safety": {
    "atomicWrites": true,
    "createBackups": false,
    "verifyWrites": false
  }
}
```

#### Core Sorting Options

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `sortOrder` | `"official"` \| `"custom"` | `"official"` | Which sorting order to use |
| `customOrder` | `string[]` | `[]` | Custom category order (when sortOrder is "custom") |

**Custom Sort Order Example:**

```json
{
  "sortOrder": "custom",
  "customOrder": [
    "layout",
    "flexbox",
    "spacing",
    "sizing",
    "typography",
    "backgrounds",
    "borders",
    "effects",
    "transforms",
    "interactivity"
  ]
}
```

#### Function Recognition

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `functionNames` | `string[]` | `["cn", "clsx", "twMerge", "classnames"]` | Functions to process |
| `presetRegex` | `"all"` \| `"react"` \| `"vue"` \| `"svelte"` \| `"angular"` | `"all"` | Framework preset |
| `customRegex` | `string[]` | `[]` | Additional regex patterns |

**Custom Functions Example:**

```json
{
  "functionNames": ["cn", "clsx", "twMerge", "myCustomUtil", "classnames"],
  "presetRegex": "react",
  "customRegex": [
    "myFramework\\([\"'`]([^\"'`]+)[\"'`]\\)"
  ]
}
```

#### File Processing

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `fileExtensions` | `string[]` | `["tsx", "jsx", "ts", "js", "vue", "svelte"]` | File types to process |
| `maxFileSize` | `number` | `1048576` | Max file size in bytes (1MB) |
| `threads` | `number` | `0` | Thread count (0 = auto-detect) |

#### Content Processing

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `removeNullClasses` | `boolean` | `true` | Remove empty/null classes |
| `preserveDuplicates` | `boolean` | `false` | Keep duplicate classes |
| `defaultMode` | `"format"` \| `"check"` \| `"diff"` \| `null` | `null` | Default operation mode |

#### Safety Features

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `safety.atomicWrites` | `boolean` | `true` | Use atomic file operations |
| `safety.createBackups` | `boolean` | `false` | Create .bak files before writing |
| `safety.verifyWrites` | `boolean` | `false` | Verify content after writing |

### Configuration Rules and Interactions

#### 1. Sort Order Rules

- When `sortOrder` is `"official"`, `customOrder` is ignored
- When `sortOrder` is `"custom"`, `customOrder` must be provided and valid
- Custom order must contain valid Tailwind categories

**Available Categories:**
```
layout, flexbox, grid, spacing, sizing, typography, backgrounds, 
borders, effects, filters, tables, transitions, transforms, 
interactivity, svg, accessibility
```

#### 2. Function Processing Rules

- Functions in `functionNames` are processed for class sorting
- `presetRegex` adds framework-specific patterns automatically
- `customRegex` patterns are added to the function recognition
- Order matters: functions are checked in the order listed

#### 3. File Discovery Rules

- `fileExtensions` filters which files are processed
- `maxFileSize` prevents processing very large files
- `ignorePaths` excludes directories like `node_modules`, `.next`, etc.

#### 4. Safety Rules

- `atomicWrites` ensures files aren't corrupted during writing
- `createBackups` creates `.bak` files before modification
- `verifyWrites` reads back written content to verify correctness
- All safety features work together - can enable multiple simultaneously

#### 5. Threading Rules

- `threads: 0` auto-detects CPU cores
- `threads: 1` forces sequential processing
- `threads > 1` uses specified thread count
- Command line `--threads` overrides config setting
- Command line `--processing sequential` sets threads to 1

## Supported Patterns

WindWarden recognizes and sorts classes in these patterns:

### 1. JSX Attributes

```jsx
// className attribute
<div className="p-4 flex m-2 items-center">

// class attribute (Vue, HTML)
<div class="p-4 flex m-2 items-center">

// Single quotes
<div className='p-4 flex m-2 items-center'>

// Expression syntax
<div className={"p-4 flex m-2 items-center"}>
```

### 2. Function Calls

```javascript
// Utility functions
cn("p-4 flex m-2 items-center")
clsx("p-4 flex m-2 items-center")
twMerge("p-4 flex m-2 items-center")
classnames("p-4 flex m-2 items-center")

// Multiple arguments
cn("p-4 flex", "m-2 items-center", "bg-white")

// With conditionals (non-string args preserved)
cn("p-4 flex", isActive && "bg-blue-500", "m-2")

// With objects (objects preserved, strings sorted)
cn("p-4 flex", { "bg-blue-500": isActive }, "m-2")
```

### 3. Template Literals

```javascript
// Static template literals
className={`p-4 flex m-2 items-center`}

// Tagged templates
const styles = tw`p-4 flex m-2 items-center`

// Dynamic templates (skipped - not sorted)
className={`p-4 ${baseClass} m-2`}
```

### 4. Arrays

```javascript
// Basic arrays
className={["p-4", "flex", "m-2", "items-center"].join(" ")}

// CVA patterns
cva(['p-4', 'flex'], {
  variants: {
    size: {
      sm: ['text-sm', 'p-2'],
      lg: ['text-lg', 'p-6']
    }
  }
})
```

### 5. Object Properties

```javascript
// Object with className property
const props = { className: "p-4 flex m-2 items-center" }

// Object with class property
const config = { class: "p-4 flex m-2 items-center" }
```

### 6. String Concatenation

```javascript
// Simple concatenation
"p-4 flex m-2" + "items-center bg-white"

// Multi-line concatenation
className={
  "p-4 flex m-2 " +
  "items-center bg-white " +
  "hover:bg-gray-100"
}
```

### Pattern Recognition Rules

1. **String Content Analysis**: WindWarden analyzes string content to determine if it contains Tailwind classes
2. **Heuristic Detection**: Uses common Tailwind prefixes to identify class strings
3. **Dynamic Content Skipping**: Template literals with variables are skipped
4. **Nested Function Support**: Handles nested function calls
5. **Quote Preservation**: Original quote style is maintained

## Sorting Rules

### Official Tailwind Order

WindWarden follows the official Tailwind CSS class order:

1. **Layout**: `display`, `position`, `visibility`, `z-index`, etc.
2. **Flexbox & Grid**: `flex`, `grid`, `justify-*`, `items-*`, etc.
3. **Spacing**: `m-*`, `p-*`, `space-*`
4. **Sizing**: `w-*`, `h-*`, `min-*`, `max-*`
5. **Typography**: `font-*`, `text-*`, `leading-*`, etc.
6. **Backgrounds**: `bg-*`
7. **Borders**: `border-*`, `divide-*`, `ring-*`
8. **Effects**: `shadow-*`, `opacity-*`
9. **Filters**: `filter`, `backdrop-*`
10. **Tables**: `table-*`
11. **Transitions**: `transition-*`, `duration-*`, `ease-*`
12. **Transforms**: `transform`, `scale-*`, `rotate-*`, etc.
13. **Interactivity**: `cursor-*`, `select-*`, `resize-*`
14. **SVG**: `fill-*`, `stroke-*`
15. **Accessibility**: `sr-only`

### Variant Handling

Variants are sorted by specificity:

```css
/* Base classes first */
flex p-4 bg-blue-500

/* Responsive variants */
sm:p-2 md:p-6 lg:p-8

/* State variants */
hover:bg-blue-600 focus:bg-blue-700

/* Combined variants */
sm:hover:bg-blue-600 md:focus:bg-blue-700
```

### Custom Classes

Non-Tailwind classes are handled intelligently:

```css
/* Custom classes are preserved in their relative position */
my-custom-class flex p-4 another-custom bg-blue-500
```

### Duplicate Handling

```json
{
  "preserveDuplicates": false  // Remove duplicates (default)
  "preserveDuplicates": true   // Keep all instances
}
```

## Output Formats

WindWarden supports multiple output formats for different use cases.

### Text Format (Default)

Human-readable output with colors and formatting:

```
✅ All files are properly formatted!

Statistics:
  Total files: 25
  Processed: 25
  Changed: 0
  Failed: 0
  Success rate: 100.0%
```

### JSON Format

Machine-readable output for tools and IDEs:

```bash
windwarden format --format json src/
```

#### Diagnostic Format (ESLint-style)

```json
{
  "version": "1.0.0",
  "tool": "windwarden",
  "results": [{
    "filePath": "/path/to/file.tsx",
    "messages": [{
      "ruleId": "class-order",
      "severity": "warning",
      "message": "Classes not in Tailwind order",
      "line": 15,
      "column": 12,
      "endLine": 15,
      "endColumn": 45,
      "source": "p-4 bg-blue-500 flex",
      "suggestions": [{
        "desc": "Sort classes according to Tailwind order",
        "fix": {
          "range": [245, 278],
          "text": "flex p-4 bg-blue-500"
        }
      }]
    }],
    "errorCount": 0,
    "warningCount": 1,
    "fixableErrorCount": 0,
    "fixableWarningCount": 1
  }],
  "errorCount": 0,
  "warningCount": 1
}
```

#### Check Format (Prettier-style)

```json
{
  "version": "1.0.0",
  "tool": "windwarden",
  "summary": {
    "totalFiles": 50,
    "formattedFiles": 47,
    "unformattedFiles": 3
  },
  "unformattedFiles": [{
    "filePath": "/src/Button.tsx",
    "issues": ["3 class ordering issues"]
  }]
}
```

#### Diff Format

```json
{
  "version": "1.0.0",
  "tool": "windwarden",
  "summary": {
    "filesProcessed": 25,
    "filesChanged": 3,
    "classesProcessed": 145,
    "classesSorted": 12
  },
  "changes": [{
    "filePath": "/src/Button.tsx",
    "modifications": [{
      "line": 15,
      "column": 12,
      "originalText": "p-4 bg-blue-500 flex",
      "newText": "flex p-4 bg-blue-500",
      "reason": "Tailwind class order correction"
    }]
  }]
}
```

#### Summary Format

```json
{
  "version": "1.0.0",
  "tool": "windwarden",
  "summary": {
    "filesProcessed": 100,
    "filesChanged": 15,
    "classesProcessed": 500,
    "classesSorted": 75
  }
}
```

### Format Selection

```bash
# Use specific format
windwarden format --format json src/
windwarden format --format check src/
windwarden format --format diff src/
windwarden format --format summary src/

# Default is text format
windwarden format src/
```

## Advanced Features

### Atomic File Operations

Prevents file corruption during writes:

```json
{
  "safety": {
    "atomicWrites": true,    // Write to temp file, then move
    "createBackups": true,   // Create .bak files
    "verifyWrites": true     // Verify content after write
  }
}
```

**How it works:**
1. Write content to temporary file (`file.tsx.tmp`)
2. Sync to disk
3. Atomically move to target (`file.tsx`)
4. Optionally verify content matches
5. Cleanup temporary files

### Custom Sort Orders

Define your own class ordering:

```json
{
  "sortOrder": "custom",
  "customOrder": [
    "layout",      // display, position, etc.
    "spacing",     // margin, padding
    "sizing",      // width, height
    "typography",  // font, text
    "backgrounds", // bg-*
    "borders",     // border-*
    "effects"      // shadow, opacity
  ]
}
```

### Advanced Regex Patterns

Add custom pattern recognition:

```json
{
  "customRegex": [
    "myUtil\\([\"'`]([^\"'`]+)[\"'`]\\)",
    "styled\\.[a-z]+`([^`]+)`",
    "css`[^`]*className[^`]*?([\"'`])([^\"'`]+)\\1[^`]*`"
  ]
}
```

### Performance Tuning

```json
{
  "threads": 4,           // Specific thread count
  "maxFileSize": 2097152, // 2MB limit
  "fileExtensions": ["tsx", "jsx"] // Limit file types
}
```

```bash
# Command line overrides
windwarden format --threads 8 --max-file-size 5242880 src/
```

## Integration

### IDE Integration

#### VS Code Tasks

```json
{
  "version": "2.0.0",
  "tasks": [{
    "label": "WindWarden: Format",
    "type": "shell",
    "command": "windwarden",
    "args": ["format", "--mode", "write", "${file}"],
    "group": "build",
    "presentation": {
      "echo": true,
      "reveal": "always"
    }
  }]
}
```

#### Language Server Integration

Use JSON output format for LSP integration:

```javascript
// Language server can parse WindWarden JSON output
const diagnostics = execSync('windwarden format --format json file.tsx');
const results = JSON.parse(diagnostics);
```

### CI/CD Integration

#### GitHub Actions

```yaml
- name: Check CSS class ordering
  run: |
    windwarden format --mode verify --format json . > results.json
    if [ $? -ne 0 ]; then
      echo "::error::CSS classes need sorting"
      cat results.json
      exit 1
    fi
```

#### Pre-commit Hooks

```yaml
repos:
  - repo: local
    hooks:
      - id: windwarden
        name: WindWarden CSS Class Sorter
        entry: windwarden
        language: system
        files: \.(tsx?|jsx?|vue|svelte)$
        args: ["format", "--mode", "verify"]
```

### Build Tool Integration

#### Webpack Plugin

```javascript
// Custom webpack plugin example
class WindWardenPlugin {
  apply(compiler) {
    compiler.hooks.beforeCompile.tapAsync('WindWardenPlugin', (params, callback) => {
      exec('windwarden format --mode verify src/', (error) => {
        if (error) {
          console.error('WindWarden: Classes need sorting');
          process.exit(1);
        }
        callback();
      });
    });
  }
}
```

#### ESLint Integration

```javascript
// Custom ESLint rule that calls WindWarden
module.exports = {
  rules: {
    'tailwind-class-order': {
      create(context) {
        return {
          JSXAttribute(node) {
            if (node.name.name === 'className') {
              // Call WindWarden to check this attribute
              // Report issues as ESLint errors
            }
          }
        };
      }
    }
  }
};
```

## Troubleshooting

### Common Issues

#### 1. Files Not Being Processed

**Problem**: WindWarden skips files unexpectedly

**Solutions**:
- Check file extensions: `windwarden format --extensions tsx,jsx,ts,js src/`
- Verify file paths: Use absolute paths if relative paths don't work
- Check ignore paths: Files might be excluded by `ignorePaths` configuration
- File size limits: Large files might exceed `maxFileSize`

```bash
# Debug file discovery
windwarden format --stats --progress src/
```

#### 2. Configuration Not Loading

**Problem**: Configuration seems to be ignored

**Solutions**:
- Check config file location: `windwarden config show`
- Validate config syntax: `windwarden config validate`
- Use explicit path: `windwarden --config .windwarden.json format src/`

```bash
# Debug configuration
windwarden config show
windwarden config validate
```

#### 3. Performance Issues

**Problem**: WindWarden is slow on large codebases

**Solutions**:
- Exclude unnecessary directories: `--exclude node_modules,dist`
- Limit file types: `--extensions tsx,jsx`
- Reduce thread count: `--threads 2`
- Increase file size limit if needed

```bash
# Performance optimized command
windwarden format --mode write \
  --exclude "node_modules/**,dist/**,build/**" \
  --extensions tsx,jsx \
  --threads 4 \
  src/
```

#### 4. Classes Not Being Sorted

**Problem**: Some class strings aren't recognized

**Solutions**:
- Add custom functions: Update `functionNames` in config
- Add custom regex: Use `customRegex` for unusual patterns
- Check string content: WindWarden uses heuristics to detect Tailwind classes

```json
{
  "functionNames": ["cn", "clsx", "myCustomUtil"],
  "customRegex": [
    "myFunction\\([\"'`]([^\"'`]+)[\"'`]\\)"
  ]
}
```

### Error Messages

#### Parse Errors

```
❌ Parse error in Button.tsx at line 15: Unexpected token '}'

💡 Suggestions:
   • Check the syntax around line 15
   • Ensure proper quote matching
   • Verify JSX/TSX syntax is valid
```

**Solution**: Fix syntax errors in the source file before running WindWarden.

#### Configuration Errors

```
❌ Configuration error: Invalid sort_order 'invalid'. Must be 'official' or 'custom'

💡 Suggestions:
   • Run 'windwarden config validate' to check your config
   • Use 'windwarden config init' to create a default config
   • Check command line arguments syntax
```

**Solution**: Fix configuration file or use valid command line arguments.

#### Permission Errors

```
❌ Permission denied: /protected/file.tsx

💡 Suggestions:
   • Check file permissions with 'ls -la /protected/file.tsx'
   • Run with appropriate privileges (sudo)
   • Ensure you have write access to the directory
```

**Solution**: Fix file permissions or run with appropriate privileges.

### Debug Mode

Enable verbose output for troubleshooting:

```bash
# Show detailed processing information
windwarden format --stats --progress --diff src/

# Validate configuration
windwarden config validate --verbose

# Test on a single file
windwarden format --mode check --diff single-file.tsx
```

### Getting Help

1. **Check the help**: `windwarden --help`
2. **Validate config**: `windwarden config validate`
3. **Test single files**: Start with one file to isolate issues
4. **Check file extensions**: Ensure you're processing the right file types
5. **Review configuration**: Use `windwarden config show` to see effective config

## Best Practices

### 1. Team Setup

- Commit `.windwarden.json` to version control
- Use pre-commit hooks to enforce formatting
- Add configuration validation to CI/CD
- Document team-specific settings

### 2. Performance

- Exclude build directories and node_modules
- Use appropriate thread counts for your system
- Limit file extensions to what you actually use
- Consider file size limits for very large files

### 3. Safety

- Use atomic writes in production environments
- Enable backups for critical operations
- Test configuration changes on small sets of files first
- Use verify mode in CI/CD to catch issues

### 4. Configuration

- Start with defaults and customize as needed
- Use custom sort orders for team preferences
- Add custom functions for your utility libraries
- Validate configuration regularly

This comprehensive guide covers all aspects of WindWarden usage. For specific use cases or advanced scenarios, refer to the relevant sections and examples provided.