# Advanced Usage Guide

Advanced features and optimization techniques for power users of WindWarden.

## Table of Contents

- [Complex Pattern Recognition](#complex-pattern-recognition)
- [Custom Sorting Orders](#custom-sorting-orders)
- [Advanced Function Recognition](#advanced-function-recognition)
- [Performance Optimization](#performance-optimization)
- [Atomic File Operations](#atomic-file-operations)
- [Output Formats](#output-formats)
- [Integration Patterns](#integration-patterns)
- [Power User Tips](#power-user-tips)

## Complex Pattern Recognition

WindWarden recognizes sophisticated class patterns beyond basic JSX attributes.

### Nested Function Calls

```javascript
// Multiple levels of function calls
const styles = cn(
  "flex items-center",
  clsx("p-4 bg-white", {
    "border-red-500": hasError,
    "border-green-500": isValid
  }),
  twMerge("shadow-md hover:shadow-lg")
)
```

### CVA (Class Variance Authority) Patterns

```javascript
// CVA configuration
const buttonVariants = cva(
  ["font-semibold", "border", "rounded", "focus:outline-none", "focus:ring-2"],
  {
    variants: {
      intent: {
        primary: ["bg-blue-500", "text-white", "border-transparent", "hover:bg-blue-600"],
        secondary: ["bg-white", "text-gray-900", "border-gray-300", "hover:bg-gray-50"]
      },
      size: {
        small: ["text-sm", "py-1", "px-2"],
        medium: ["text-base", "py-2", "px-4"],
        large: ["text-lg", "py-3", "px-6"]
      }
    }
  }
)
```

### Template Literal Combinations

```javascript
// Mixed static and dynamic templates
const complexStyles = `
  ${baseClasses}
  ${tw`flex items-center p-4 bg-white`}
  ${isActive ? 'ring-2 ring-blue-500' : ''}
`
```

### Object Property Patterns

```javascript
// Styled-components or emotion patterns
const StyledButton = styled.button`
  ${props => props.variant === 'primary' && tw`bg-blue-500 text-white p-4 rounded`}
`

// Configuration objects
const componentConfig = {
  defaultClasses: "flex items-center p-4",
  variants: {
    primary: "bg-blue-500 text-white",
    secondary: "bg-gray-200 text-gray-900"
  }
}
```

## Custom Sorting Orders

Create your own class ordering system for team preferences or specific design systems.

### Basic Custom Order

```json
{
  "sortOrder": "custom",
  "customOrder": [
    "layout",      // display, position, visibility
    "spacing",     // margin, padding
    "sizing",      // width, height, min-*, max-*
    "typography",  // font-*, text-*, leading-*
    "backgrounds", // bg-*
    "borders",     // border-*, ring-*
    "effects",     // shadow-*, opacity-*
    "interactivity" // cursor-*, select-*
  ]
}
```

### Design System Aligned Order

```json
{
  "sortOrder": "custom",
  "customOrder": [
    "layout",
    "flexbox",
    "grid",
    "spacing",
    "sizing",
    "typography",
    "backgrounds",
    "borders",
    "effects",
    "filters",
    "transforms",
    "transitions",
    "interactivity"
  ]
}
```

### Available Categories

WindWarden supports these built-in categories:

```
layout, flexbox, grid, spacing, sizing, typography, backgrounds, 
borders, effects, filters, tables, transitions, transforms, 
interactivity, svg, accessibility
```

## Advanced Function Recognition

Configure WindWarden to recognize additional utility functions and patterns.

### Custom Utility Functions

Add additional function names to the `functionNames` configuration:

```json
{
  "functionNames": [
    "cn",
    "clsx", 
    "twMerge",
    "myUtil",
    "designSystem.classes",
    "theme"
  ]
}
```

### Supported Patterns

WindWarden automatically recognizes these patterns through AST parsing:

#### Function Calls
```javascript
// Standard utility functions
cn("flex items-center p-4")
clsx("bg-white", { "border-red-500": hasError })
twMerge("text-sm font-bold")

// Custom utility functions (when added to functionNames)
myUtil("rounded-lg shadow-md")
designSystem.classes("primary-button")
```

#### Template Literals
```javascript
// Tagged templates
tw`flex items-center p-4`
css`
  .button {
    @apply bg-blue-500 text-white px-4 py-2 rounded;
  }
`

// Template string interpolation (static parts only)
const classes = `flex items-center ${baseClasses} p-4`
```

#### Array Patterns
```javascript
// CVA (Class Variance Authority) patterns
const variants = cva([
  "font-semibold",
  "border", 
  "rounded",
  "focus:outline-none"
])

// Basic arrays
const classes = ["flex", "items-center", "p-4"]
```

## Performance Optimization

Optimize WindWarden for large codebases and specific requirements.

### Thread Configuration

```bash
# Auto-detect CPU cores (default)
windwarden format --threads 0 src/

# Use specific thread count
windwarden format --threads 4 src/

# Force sequential processing
windwarden format --processing sequential src/
```

### File Filtering

```bash
# Limit file types for better performance
windwarden format --extensions tsx,jsx src/

# Exclude unnecessary directories
windwarden format --exclude "node_modules/**,dist/**,build/**,.next/**" .

# Set maximum file size (in bytes)
windwarden format --config config.json src/
```

```json
{
  "maxFileSize": 2097152,  // 2MB limit
  "fileExtensions": ["tsx", "jsx"],  // Only React files
  "threads": 4
}
```

### Large Codebase Strategies

```bash
# Process incrementally
windwarden format --mode write src/components/
windwarden format --mode write src/pages/
windwarden format --mode write src/lib/

# Use progress tracking
windwarden format --progress --stats src/

# Limit directory depth
windwarden format --max-depth 3 src/
```

### Memory Optimization

```json
{
  "maxFileSize": 1048576,  // 1MB per file
  "threads": 2,            // Reduce concurrent files
  "fileExtensions": ["tsx", "jsx"]  // Limit file types
}
```

## Atomic File Operations

Ensure data safety during file modifications.

### Safety Configuration

```json
{
  "safety": {
    "atomicWrites": true,    // Write to temp file first
    "createBackups": true,   // Create .bak files
    "verifyWrites": true     // Verify content after write
  }
}
```

### How Atomic Writes Work

1. **Create temporary file**: `Button.tsx.tmp.12345`
2. **Write new content**: All changes go to temp file
3. **Sync to disk**: Ensure data is written
4. **Atomic rename**: Move temp file to target
5. **Cleanup**: Remove temporary files

### Backup Strategy

```json
{
  "safety": {
    "createBackups": true,
    "backupExtension": ".bak",  // Custom extension
    "maxBackups": 5             // Keep only recent backups
  }
}
```

## Output Formats

Machine-readable output for tools and automation.

### JSON Diagnostic Format (ESLint-style)

```bash
windwarden format --format json src/ > diagnostics.json
```

```json
{
  "version": "1.0.0",
  "tool": "windwarden",
  "results": [{
    "filePath": "/path/to/Button.tsx",
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
    }]
  }]
}
```

### Check Format (Prettier-style)

```bash
windwarden format --format check --mode verify src/
```

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

### Summary Format

```bash
windwarden format --format summary src/
```

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

## Integration Patterns

### Language Server Integration

```javascript
// LSP server implementation
const { execSync } = require('child_process');

function getDiagnostics(filePath) {
  try {
    const result = execSync(
      `windwarden format --format json "${filePath}"`,
      { encoding: 'utf8' }
    );
    
    const diagnostics = JSON.parse(result);
    return convertToDiagnostics(diagnostics.results);
  } catch (error) {
    return [];
  }
}
```

### Custom Webpack Plugin

```javascript
class WindWardenPlugin {
  constructor(options = {}) {
    this.options = {
      mode: 'verify',
      failOnError: true,
      ...options
    };
  }

  apply(compiler) {
    compiler.hooks.beforeCompile.tapAsync('WindWardenPlugin', (params, callback) => {
      const { execSync } = require('child_process');
      
      try {
        execSync(`windwarden format --mode ${this.options.mode} src/`, {
          stdio: 'inherit'
        });
        callback();
      } catch (error) {
        if (this.options.failOnError) {
          callback(new Error('WindWarden: Class formatting issues found'));
        } else {
          console.warn('WindWarden: Class formatting issues found');
          callback();
        }
      }
    });
  }
}

module.exports = WindWardenPlugin;
```

### Custom ESLint Rule

```javascript
module.exports = {
  rules: {
    'tailwind-class-order': {
      meta: {
        type: 'layout',
        docs: {
          description: 'Enforce Tailwind CSS class order'
        },
        fixable: 'code'
      },
      
      create(context) {
        return {
          JSXAttribute(node) {
            if (node.name.name === 'className' && node.value.type === 'Literal') {
              const { execSync } = require('child_process');
              const tempFile = `/tmp/eslint-windwarden-${Date.now()}.tsx`;
              
              // Write content to temp file and check with WindWarden
              // Report issues as ESLint errors with fix suggestions
            }
          }
        };
      }
    }
  }
};
```

## Power User Tips

### 1. Batch Processing Strategies

```bash
# Process by file type
find src -name "*.tsx" -exec windwarden format --mode write {} +
find src -name "*.jsx" -exec windwarden format --mode write {} +

# Process with xargs for large file lists
find src -name "*.tsx" | xargs -P 4 -I {} windwarden format --mode write {}
```

### 2. Configuration Management

```bash
# Multiple configurations for different projects
windwarden --config .windwarden.react.json format src/
windwarden --config .windwarden.typescript.json format src/

# Environment-specific configurations
if [ "$NODE_ENV" = "development" ]; then
  CONFIG=".windwarden.dev.json"
else
  CONFIG=".windwarden.prod.json"
fi
windwarden --config $CONFIG format src/
```

### 3. Selective Processing

```bash
# Process only changed files (with Git)
git diff --name-only --diff-filter=ACMR | grep -E '\.(tsx?|jsx?)$' | xargs windwarden format --mode write

# Process files modified in last commit
git diff --name-only HEAD~1 | grep -E '\.(tsx?|jsx?)$' | xargs windwarden format --mode verify
```

### 4. Custom Workflows

```bash
#!/bin/bash
# pre-commit-windwarden.sh

# Get staged files
STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACMR | grep -E '\.(tsx?|jsx?)$')

if [ -n "$STAGED_FILES" ]; then
  echo "Running WindWarden on staged files..."
  
  # Format staged files
  echo "$STAGED_FILES" | xargs windwarden format --mode write
  
  # Re-stage formatted files
  echo "$STAGED_FILES" | xargs git add
  
  echo "WindWarden formatting complete!"
fi
```

### 5. Monitoring and Metrics

```bash
# Generate formatting report
windwarden format --format summary --stats src/ > formatting-report.json

# Track formatting coverage over time
echo "$(date): $(jq '.summary.filesChanged' formatting-report.json)" >> formatting-history.log
```

### 6. Debug and Analysis

```bash
# Detailed analysis of a single file
windwarden format --mode check --diff --stats Button.tsx

# Test configuration changes
windwarden --config test-config.json format --mode check src/

# Performance profiling
time windwarden format --mode check --stats src/
```

These advanced techniques help you integrate WindWarden deeply into your development workflow and optimize it for your specific needs. For basic usage, see the [Getting Started Guide](./getting-started.md).