# Configuration Guide

Complete reference for configuring WindWarden to match your project's needs.

## Table of Contents

- [Configuration System Overview](#configuration-system-overview)
- [Configuration File Format](#configuration-file-format)
- [Core Settings](#core-settings)
- [Function Recognition](#function-recognition)
- [File Processing](#file-processing)
- [Safety Features](#safety-features)
- [Configuration Rules](#configuration-rules)
- [Examples](#examples)
- [Validation](#validation)

## Configuration System Overview

WindWarden uses a hierarchical configuration system that merges settings from multiple sources.

### Configuration Precedence

Settings are applied in this order (highest to lowest priority):

1. **Command line arguments** - `--threads 4`, `--mode write`
2. **Configuration file** - JSON file specified by `--config` or auto-discovered
3. **Default values** - Built-in sensible defaults

### Configuration Discovery

WindWarden searches for configuration files in this order:

1. **Explicit path**: `--config ./my-config.json`
2. **Current directory**: `.windwarden.json`
3. **Parent directories**: Walking up to filesystem root
4. **User home**: `~/.windwarden.json`

### Creating Configuration

```bash
# Create default configuration in current directory
windwarden config init

# Create in specific location
windwarden config init --path ./custom-config.json

# Show current effective configuration
windwarden config show

# Validate configuration file
windwarden config validate
windwarden config validate ./my-config.json
```

## Configuration File Format

Configuration files use JSON format with this structure:

```json
{
  "sortOrder": "official",
  "customOrder": [],
  "functionNames": ["cn", "clsx", "twMerge", "classnames"],
  "fileExtensions": ["tsx", "jsx", "ts", "js"],
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

## Core Settings

### Sort Order Configuration

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `sortOrder` | `"official"` \| `"custom"` | `"official"` | Which sorting order to use |
| `customOrder` | `string[]` | `[]` | Custom category order (required when sortOrder is "custom") |

#### Official Order
Uses the standard Tailwind CSS class order:

```json
{
  "sortOrder": "official"
}
```

#### Custom Order
Define your own category ordering:

```json
{
  "sortOrder": "custom",
  "customOrder": [
    "layout",
    "spacing", 
    "sizing",
    "typography",
    "backgrounds",
    "borders",
    "effects"
  ]
}
```

#### Available Categories

```
layout, flexbox, grid, spacing, sizing, typography, backgrounds, 
borders, effects, filters, tables, transitions, transforms, 
interactivity, svg, accessibility
```

### Content Processing Options

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `removeNullClasses` | `boolean` | `true` | Remove empty/null classes from output |
| `preserveDuplicates` | `boolean` | `false` | Keep duplicate classes instead of deduplicating |
| `defaultMode` | `"format"` \| `"check"` \| `"diff"` \| `null` | `null` | Default operation mode |

#### Remove Null Classes

```json
{
  "removeNullClasses": true  // "flex  p-4" becomes "flex p-4"
}
```

#### Preserve Duplicates

```json
{
  "preserveDuplicates": false  // "flex flex p-4" becomes "flex p-4"
  "preserveDuplicates": true   // "flex flex p-4" stays "flex flex p-4"
}
```

## Function Recognition

Configure which utility functions WindWarden should process.

### Function Names

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `functionNames` | `string[]` | `["cn", "clsx", "twMerge", "classnames"]` | Function names to process |

```json
{
  "functionNames": [
    "cn",           // clsx/cn utility
    "clsx",         // clsx library
    "twMerge",      // tailwind-merge
    "classnames",   // classnames library
    "myCustomUtil", // custom utility function
    "cx",           // emotion cx
    "classNames"    // alternative naming
  ]
}
```

### Pattern Recognition

WindWarden automatically recognizes Tailwind CSS classes in these patterns:

- **JSX attributes**: `className="..."`, `class="..."`
- **Utility functions**: Functions listed in `functionNames` configuration
- **Template literals**: Tagged template literals like `tw\`...\``
- **Arrays**: CVA patterns and basic array syntax

The parser uses AST-based detection to find class strings while preserving your original code formatting.

## File Processing

Configure which files WindWarden processes and how.

### File Extension Filtering

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `fileExtensions` | `string[]` | `["tsx", "jsx", "ts", "js"]` | File types to process |

```json
{
  "fileExtensions": ["tsx", "jsx"]  // React only
  "fileExtensions": ["ts", "js"]    // TypeScript + JavaScript
}
```

### File Size and Performance

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `maxFileSize` | `number` | `1048576` | Maximum file size in bytes (1MB) |
| `threads` | `number` | `0` | Thread count (0 = auto-detect CPU cores) |

```json
{
  "maxFileSize": 2097152,  // 2MB limit
  "threads": 4             // Use 4 threads
}
```

#### Thread Configuration

```json
{
  "threads": 0   // Auto-detect CPU cores (default)
  "threads": 1   // Sequential processing
  "threads": 4   // Use 4 threads
  "threads": 8   // Use 8 threads
}
```

## Safety Features

Protect your files during processing with these safety options.

### Safety Configuration

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `safety.atomicWrites` | `boolean` | `true` | Use atomic file operations |
| `safety.createBackups` | `boolean` | `false` | Create .bak files before writing |
| `safety.verifyWrites` | `boolean` | `false` | Verify content after writing |

```json
{
  "safety": {
    "atomicWrites": true,     // Write to temp file first, then rename
    "createBackups": true,    // Create .bak files
    "verifyWrites": true      // Read back and verify written content
  }
}
```

### Atomic Writes Process

When `atomicWrites` is enabled:

1. Create temporary file: `Button.tsx.tmp.12345`
2. Write new content to temp file
3. Sync temp file to disk
4. Atomically rename temp file to target: `Button.tsx`
5. Clean up any remaining temp files

### Backup Strategy

```json
{
  "safety": {
    "createBackups": true,
    "backupExtension": ".bak",  // Custom backup extension
    "maxBackups": 5             // Keep only 5 most recent backups
  }
}
```

## Configuration Rules

Understanding how settings interact with each other.

### 1. Sort Order Rules

- When `sortOrder` is `"official"`, `customOrder` is ignored
- When `sortOrder` is `"custom"`, `customOrder` must be provided and contain valid categories
- Invalid categories in `customOrder` will cause validation errors

### 2. Function Processing Rules

- Functions are processed based on the `functionNames` configuration
- Default function names include: `cn`, `clsx`, `twMerge`, `classNames`, `classList`, `cva`
- Additional function names can be specified in the `functionNames` array
- AST-based parsing ensures accurate detection of class strings

### 3. File Discovery Rules

- `fileExtensions` filters which files are processed during directory traversal
- `maxFileSize` prevents processing files that exceed the byte limit
- Hidden files and directories are skipped by default
- Standard ignore patterns apply: `node_modules`, `.git`, `dist`, etc.

### 4. Safety Feature Interactions

- All safety features can be enabled simultaneously
- `atomicWrites` is recommended for production environments
- `createBackups` provides an extra safety net
- `verifyWrites` adds a verification step after writing

### 5. Threading Rules

- `threads: 0` auto-detects available CPU cores
- `threads: 1` forces sequential processing
- Command line `--threads` overrides config setting
- Command line `--processing sequential` sets threads to 1

## Examples

### React Project Configuration

```json
{
  "sortOrder": "official",
  "functionNames": ["cn", "clsx", "twMerge"],
  "fileExtensions": ["tsx", "jsx", "ts", "js"],
  "maxFileSize": 1048576,
  "threads": 0,
  "removeNullClasses": true,
  "preserveDuplicates": false,
  "safety": {
    "atomicWrites": true,
    "createBackups": false,
    "verifyWrites": false
  }
}
```

### TypeScript Project Configuration

```json
{
  "sortOrder": "official",
  "functionNames": ["cn", "clsx"],
  "fileExtensions": ["ts", "tsx", "js", "jsx"],
  "maxFileSize": 2097152,
  "threads": 4,
  "safety": {
    "atomicWrites": true,
    "createBackups": true,
    "verifyWrites": false
  }
}
```

### Monorepo Configuration

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
    "effects"
  ],
  "functionNames": ["cn", "clsx", "twMerge", "classNames"],
  "fileExtensions": ["tsx", "jsx", "ts", "js"],
  "maxFileSize": 1048576,
  "threads": 0,
  "safety": {
    "atomicWrites": true,
    "createBackups": false,
    "verifyWrites": true
  }
}
```

### Performance-Optimized Configuration

```json
{
  "sortOrder": "official",
  "functionNames": ["cn"],
  "fileExtensions": ["tsx", "jsx"],
  "maxFileSize": 524288,  // 512KB limit
  "threads": 2,           // Limited threads
  "removeNullClasses": true,
  "preserveDuplicates": false,
  "safety": {
    "atomicWrites": true,
    "createBackups": false,
    "verifyWrites": false
  }
}
```

### Development vs Production

#### Development Configuration
```json
{
  "sortOrder": "official",
  "functionNames": ["cn", "clsx", "twMerge", "classnames"],
  "safety": {
    "atomicWrites": true,
    "createBackups": true,
    "verifyWrites": false
  }
}
```

#### Production Configuration
```json
{
  "sortOrder": "official",
  "functionNames": ["cn"],
  "fileExtensions": ["tsx", "jsx"],
  "safety": {
    "atomicWrites": true,
    "createBackups": false,
    "verifyWrites": true
  }
}
```

## Validation

Ensure your configuration is correct and complete.

### Manual Validation

```bash
# Validate current configuration
windwarden config validate

# Validate specific file
windwarden config validate ./my-config.json

# Show current effective configuration
windwarden config show
```

### Common Validation Errors

#### Invalid Sort Order
```json
{
  "sortOrder": "invalid"  // Error: must be "official" or "custom"
}
```

#### Missing Custom Order
```json
{
  "sortOrder": "custom"   // Error: customOrder required when sortOrder is "custom"
  // Missing customOrder array
}
```

#### Invalid Category Names
```json
{
  "sortOrder": "custom",
  "customOrder": ["invalid-category"]  // Error: unknown category
}
```

#### Invalid Thread Count
```json
{
  "threads": -1  // Error: must be non-negative integer
}
```

### Configuration Schema

WindWarden validates configuration against this schema:

```json
{
  "type": "object",
  "properties": {
    "sortOrder": {"enum": ["official", "custom"]},
    "customOrder": {"type": "array", "items": {"type": "string"}},
    "functionNames": {"type": "array", "items": {"type": "string"}},
    "fileExtensions": {"type": "array", "items": {"type": "string"}},
    "maxFileSize": {"type": "integer", "minimum": 0},
    "threads": {"type": "integer", "minimum": 0},
    "removeNullClasses": {"type": "boolean"},
    "preserveDuplicates": {"type": "boolean"},
    "defaultMode": {"enum": ["format", "check", "diff", null]},
    "safety": {
      "type": "object",
      "properties": {
        "atomicWrites": {"type": "boolean"},
        "createBackups": {"type": "boolean"},
        "verifyWrites": {"type": "boolean"}
      }
    }
  }
}
```

For more examples and advanced configuration patterns, see the [Advanced Usage Guide](./advanced-usage.md).