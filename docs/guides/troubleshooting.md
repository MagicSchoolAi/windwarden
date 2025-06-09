# Troubleshooting Guide

Solutions for common issues and debugging techniques for WindWarden.

## Table of Contents

- [Common Issues](#common-issues)
- [Error Messages](#error-messages)
- [Debug Techniques](#debug-techniques)
- [Performance Issues](#performance-issues)
- [Configuration Problems](#configuration-problems)
- [File Processing Issues](#file-processing-issues)
- [Getting Help](#getting-help)

## Common Issues

### 1. Files Not Being Processed

**Problem**: WindWarden skips files or reports "0 files processed"

**Symptoms**:
```
✅ All files are properly formatted!
Statistics:
  Total files: 0
  Processed: 0
```

**Solutions**:

#### Check File Extensions
```bash
# Verify which extensions WindWarden is looking for
windwarden config show

# Override extensions for testing
windwarden format --extensions tsx,jsx,ts,js,vue,svelte src/
```

#### Verify File Paths
```bash
# Use absolute paths if relative paths don't work
windwarden format /full/path/to/src/

# Check current directory
pwd
ls -la src/
```

#### Check Exclusion Patterns
```bash
# Files might be excluded by ignore patterns
windwarden format --exclude "" src/

# Test with minimal exclusions
windwarden format --exclude "node_modules" src/
```

#### File Size Limits
```bash
# Check if files exceed size limits
find src/ -name "*.tsx" -size +1M

# Increase size limit
windwarden format --config custom-config.json src/
```

```json
{
  "maxFileSize": 2097152  // 2MB instead of 1MB
}
```

#### Debug File Discovery
```bash
# Use stats and progress to see what's happening
windwarden format --stats --progress src/

# Test on a single file
windwarden format src/components/Button.tsx
```

### 2. Configuration Not Loading

**Problem**: Configuration seems to be ignored or not found

**Symptoms**:
- Settings don't take effect
- Default behavior instead of configured behavior
- "Configuration not found" warnings

**Solutions**:

#### Check Configuration Location
```bash
# Show current effective configuration
windwarden config show

# Show where WindWarden is looking for config
windwarden format --stats src/
```

#### Validate Configuration Syntax
```bash
# Check if configuration file is valid JSON
windwarden config validate

# Validate specific file
windwarden config validate .windwarden.json

# Check JSON syntax manually
cat .windwarden.json | jq .
```

#### Use Explicit Configuration Path
```bash
# Specify configuration file explicitly
windwarden --config .windwarden.json format src/

# Use absolute path
windwarden --config /full/path/to/config.json format src/
```

#### Configuration Discovery Order
WindWarden searches in this order:
1. `--config` argument path
2. `.windwarden.json` in current directory  
3. `.windwarden.json` in parent directories
4. `~/.windwarden.json` (home directory)

### 3. Classes Not Being Sorted

**Problem**: Some class strings aren't recognized or sorted

**Symptoms**:
- Certain utility function calls are ignored
- Template literals not processed
- Custom patterns not detected

**Solutions**:

#### Add Custom Functions
```json
{
  "functionNames": ["cn", "clsx", "twMerge", "myCustomUtil"]
}
```

#### Add Custom Regex Patterns
```json
{
  "customRegex": [
    "myFunction\\([\"'`]([^\"'`]+)[\"'`]\\)",
    "styled\\.[a-z]+`([^`]+)`"
  ]
}
```

#### Check String Content Detection
WindWarden uses heuristics to detect Tailwind classes:
- Must contain common Tailwind prefixes
- Avoids processing non-class strings
- Skips dynamic template literals

#### Debug Pattern Matching
```bash
# Test on a single file with specific pattern
windwarden format --diff src/component-with-issue.tsx

# Use different preset regex
windwarden format --config test-config.json src/
```

```json
{
  "presetRegex": "all",  // Try "all" instead of framework-specific
  "functionNames": ["*"] // Add all common function names
}
```

### 4. Performance Issues

**Problem**: WindWarden is slow on large codebases

**Symptoms**:
- Takes a long time to process files
- High CPU or memory usage
- Timeouts or crashes

**Solutions**:

#### Optimize File Processing
```bash
# Exclude unnecessary directories
windwarden format --exclude "node_modules/**,dist/**,build/**,.next/**" src/

# Limit file types
windwarden format --extensions tsx,jsx src/

# Process smaller batches
windwarden format src/components/
windwarden format src/pages/
```

#### Adjust Thread Settings
```bash
# Reduce thread count
windwarden format --threads 2 src/

# Force sequential processing
windwarden format --processing sequential src/
```

#### Configuration Optimization
```json
{
  "threads": 2,
  "maxFileSize": 1048576,
  "fileExtensions": ["tsx", "jsx"],
  "functionNames": ["cn"]  // Limit to essential functions
}
```

#### Monitor Performance
```bash
# Time the operation
time windwarden format --stats src/

# Use progress tracking
windwarden format --progress src/

# Profile with system tools
top -p $(pgrep windwarden)
```

### 5. File Write Issues

**Problem**: Files aren't being written or are corrupted

**Symptoms**:
- "Permission denied" errors
- Files appear unchanged
- Corrupted or empty files

**Solutions**:

#### Check Permissions
```bash
# Verify write permissions
ls -la src/components/Button.tsx

# Check directory permissions
ls -la src/components/

# Fix permissions if needed
chmod 644 src/components/Button.tsx
chmod 755 src/components/
```

#### Use Safety Features
```json
{
  "safety": {
    "atomicWrites": true,    // Prevent corruption
    "createBackups": true,   // Create .bak files
    "verifyWrites": true     // Verify after writing
  }
}
```

#### Test Write Operations
```bash
# Test on a single file first
windwarden format --mode write src/components/Button.tsx

# Use check mode to preview changes
windwarden format --mode check --diff src/components/Button.tsx
```

## Error Messages

### Parse Errors

```
❌ Parse error in Button.tsx at line 15: Unexpected token '}'
```

**Cause**: Syntax errors in JavaScript/TypeScript code

**Solution**: Fix syntax errors before running WindWarden
```bash
# Check syntax with TypeScript compiler
npx tsc --noEmit Button.tsx

# Or with ESLint
npx eslint Button.tsx
```

### Configuration Errors

```
❌ Configuration error: Invalid sortOrder 'invalid'. Must be 'official' or 'custom'
```

**Cause**: Invalid configuration values

**Solution**: Fix configuration file
```bash
# Validate configuration
windwarden config validate

# Regenerate default config
windwarden config init --force
```

### Permission Errors

```
❌ Permission denied: /protected/file.tsx
```

**Cause**: Insufficient file system permissions

**Solutions**:
```bash
# Check permissions
ls -la /protected/file.tsx

# Fix permissions
chmod 644 /protected/file.tsx

# Or run with sudo (if appropriate)
sudo windwarden format /protected/file.tsx
```

### Memory Errors

```
❌ Out of memory processing large file
```

**Cause**: File exceeds available memory

**Solutions**:
```bash
# Reduce max file size
windwarden format --config reduced-config.json src/

# Process files individually
find src -name "*.tsx" -exec windwarden format --mode write {} \;
```

## Debug Techniques

### 1. Verbose Output

```bash
# Show detailed statistics
windwarden format --stats src/

# Show progress information
windwarden format --progress src/

# Show differences
windwarden format --diff src/

# Combine for maximum information
windwarden format --stats --progress --diff src/
```

### 2. Single File Testing

```bash
# Test on one file to isolate issues
windwarden format --mode check --diff Button.tsx

# Test with specific configuration
windwarden --config test-config.json format Button.tsx

# Test with minimal settings
echo '{"sortOrder": "official"}' > minimal-config.json
windwarden --config minimal-config.json format Button.tsx
```

### 3. Configuration Debugging

```bash
# Show effective configuration
windwarden config show

# Validate configuration
windwarden config validate --verbose

# Test different configurations
windwarden --config config1.json format --mode check src/
windwarden --config config2.json format --mode check src/
```

### 4. Pattern Testing

Create test files to verify pattern recognition:

```tsx
// test-patterns.tsx
import { cn, clsx, twMerge } from 'utils';

export function TestComponent() {
  return (
    <div className="p-4 bg-blue-500 flex">
      <span className={cn("text-white font-bold m-2")}>Test</span>
      <button className={clsx("px-4 py-2 rounded bg-red-500")}>Button</button>
    </div>
  );
}
```

```bash
# Test pattern recognition
windwarden format --diff test-patterns.tsx
```

### 5. Performance Profiling

```bash
# Time operations
time windwarden format --stats src/

# Monitor system resources
# Terminal 1:
windwarden format src/
# Terminal 2:
htop  # or top

# Profile with sampling
perf record windwarden format src/
perf report
```

## Performance Issues

### Large Codebases

**Problem**: Slow processing on large projects (1000+ files)

**Solutions**:

#### Incremental Processing
```bash
# Process by directory
windwarden format src/components/
windwarden format src/pages/
windwarden format src/lib/

# Process by file type
find src -name "*.tsx" | head -100 | xargs windwarden format --mode write
```

#### Optimization Configuration
```json
{
  "threads": 4,
  "maxFileSize": 1048576,
  "fileExtensions": ["tsx", "jsx"],
  "functionNames": ["cn"],
  "presetRegex": "react",
  "customRegex": []
}
```

#### Exclude Large Directories
```bash
windwarden format \
  --exclude "node_modules/**,dist/**,build/**,.next/**,coverage/**" \
  src/
```

### Memory Usage

**Problem**: High memory consumption

**Solutions**:

#### Reduce Concurrency
```bash
# Use fewer threads
windwarden format --threads 2 src/

# Process sequentially
windwarden format --processing sequential src/
```

#### Limit File Size
```json
{
  "maxFileSize": 524288  // 512KB limit
}
```

#### Process in Batches
```bash
# Process files in smaller batches
find src -name "*.tsx" | split -l 50 - batch_
for batch in batch_*; do
  cat $batch | xargs windwarden format --mode write
done
```

## Configuration Problems

### Invalid JSON

```bash
# Check JSON syntax
cat .windwarden.json | jq .

# Find syntax errors
windwarden config validate
```

### Schema Validation

```bash
# Validate against schema
windwarden config validate

# Fix common issues
windwarden config init --force  # Recreate with defaults
```

### Conflicting Settings

**Problem**: Command line arguments conflict with configuration

**Solution**: Understand precedence order:
1. Command line arguments (highest)
2. Configuration file  
3. Defaults (lowest)

```bash
# Override config with command line
windwarden --config myconfig.json format --threads 8 src/
```

## File Processing Issues

### Files Skipped

**Debug checklist**:
```bash
# 1. Check file extensions
windwarden config show | grep fileExtensions

# 2. Check file size
ls -lh problematic-file.tsx

# 3. Check exclusion patterns
windwarden format --stats --progress src/

# 4. Test single file
windwarden format problematic-file.tsx
```

### Encoding Issues

**Problem**: Files with non-UTF8 encoding

**Solution**:
```bash
# Check file encoding
file -i problematic-file.tsx

# Convert to UTF-8
iconv -f ISO-8859-1 -t UTF-8 problematic-file.tsx > fixed-file.tsx
```

### Symlink Issues

**Problem**: Symlinked files not processed

**Solution**:
```bash
# Follow symlinks
windwarden format --follow-links src/

# Or process target directly
windwarden format $(readlink src/symlinked-file.tsx)
```

## Getting Help

### Self-Diagnosis Steps

1. **Check version**: `windwarden --version`
2. **Validate config**: `windwarden config validate`
3. **Test single file**: `windwarden format test.tsx`
4. **Check permissions**: `ls -la target-file.tsx`
5. **Review logs**: Use `--stats --progress` for detailed output

### Information to Gather

When reporting issues, include:

```bash
# System information
windwarden --version
uname -a

# Configuration
windwarden config show

# Error reproduction
windwarden format --stats --progress problematic-file.tsx 2>&1
```

### Common Command Combinations

```bash
# Full diagnostic run
windwarden format --stats --progress --diff src/ 2>&1 | tee windwarden-debug.log

# Minimal test case
echo '{}' > minimal-config.json
windwarden --config minimal-config.json format test-file.tsx

# Performance test
time windwarden format --stats src/ > performance-report.txt 2>&1
```

### Create Minimal Reproduction

```bash
# Create test directory
mkdir windwarden-test
cd windwarden-test

# Create test file
cat > test.tsx << 'EOF'
export function Test() {
  return <div className="p-4 bg-blue-500 flex">Test</div>;
}
EOF

# Create minimal config
echo '{"sortOrder": "official"}' > .windwarden.json

# Test
windwarden format --stats test.tsx
```

This minimal setup helps isolate issues from project-specific configuration or environment problems.

For additional help, see the [Getting Started Guide](./getting-started.md) or [Configuration Guide](./configuration.md).