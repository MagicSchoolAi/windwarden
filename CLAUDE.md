# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

WindWarden is a high-performance CLI tool for sorting and organizing Tailwind CSS classes. It's built in Rust and uses the Oxc parser for JavaScript/TypeScript AST parsing.

## Development Environment

The project uses Nix for environment management. To set up the development environment:

```bash
nix-shell
```

or use direnv:

```bash
direnv allow
```

This provides the necessary Rust toolchain and dependencies including:
- cargo (build tool)
- rustc (compiler)
- rust-analyzer (LSP)
- rustfmt (formatter)
- clang and pkg-config (for native dependencies)

## Key Architecture

### Core Components

1. **Parser Module** (`src/parser/`)
   - Uses Oxc parser for JavaScript/TypeScript AST parsing
   - Visitor pattern for extracting class strings from various patterns
   - Handles JSX attributes, function calls, template literals, and arrays

2. **Sorter Module** (`src/sorter/`)
   - Core sorting logic following official Tailwind order
   - Category-based sorting system
   - Handles variants, pseudo-classes, and custom prefixes

3. **Processor Module** (`src/processor/`)
   - File processing orchestration
   - Parallel processing using Rayon
   - Multiple output modes (dry-run, write, check-formatted)

### Supported Patterns

- JSX attributes: `className="..."`, `class="..."`
- Utility functions: `cn()`, `twMerge()`, `clsx()`, custom functions
- Template literals: `tw\`...\``, backtick templates
- Arrays: CVA patterns, basic arrays
- All quote styles: single, double, backticks

### Performance Design

- Built on Oxc parser (fastest JS/TS parser in Rust)
- Memory arena allocation for fast AST processing
- Parallel file processing
- Lazy regex compilation
- AST-based targeting preserves original formatting

## Testing

The project includes comprehensive test cases in `TESTCASEREFERENCE.ts` covering:
- Basic JSX patterns
- Function calls with conditionals
- Template literals (static and dynamic)
- CVA patterns and arrays
- Edge cases and malformed input
- Framework-specific patterns (Vue, Svelte)

When implementing features, reference these test cases to ensure compatibility with real-world usage patterns.

## Configuration

WindWarden supports various configuration options:
- Preset regex patterns for different utility functions
- Custom sort orders and categories
- Function name customization
- File extension filters
- JSON configuration files

The tool is designed to work out-of-the-box with common setups while being highly customizable for specific needs.
