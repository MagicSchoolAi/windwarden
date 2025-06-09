Here's my plan for building WindWarden:

## Architecture Overview

```
windwarden/
├── src/
│   ├── main.rs                 # CLI entry point
│   ├── lib.rs                  # Library interface
│   ├── cli/
│   │   ├── mod.rs              # CLI arg parsing (clap)
│   │   └── config.rs           # Config file handling
│   ├── parser/
│   │   ├── mod.rs              # Oxc integration
│   │   ├── visitor.rs          # AST visitor for class extraction
│   │   └── patterns.rs         # Pattern matching logic
│   ├── sorter/
│   │   ├── mod.rs              # Core sorting logic
│   │   ├── categories.rs       # Tailwind category definitions
│   │   └── order.rs            # Custom order handling
│   ├── processor/
│   │   ├── mod.rs              # File processing orchestration
│   │   ├── parallel.rs         # Rayon-based parallel processing
│   │   └── formatter.rs        # Output formatting
│   └── utils/
│       ├── mod.rs
│       ├── regex.rs            # Compiled regex patterns
│       └── error.rs            # Error types
├── tests/
│   ├── fixtures/               # Test files
│   └── integration/            # End-to-end tests
└── Cargo.toml
```

## Implementation Plan

### Phase 1: Core Infrastructure
1. **CLI Setup** - Use `clap` for argument parsing with derive API
2. **Config Loading** - `serde_json` for config files, merge CLI args with file config
3. **Error Handling** - Custom error types with `thiserror`

### Phase 2: Parser Integration
1. **Oxc Setup** - Integrate `oxc_parser` and `oxc_ast`
2. **AST Visitor** - Implement visitor pattern to find class strings in:
   - JSX attributes (`className`, `class`)
   - Function calls (`cn()`, `twMerge()`, etc.)
   - Template literals
   - Arrays (CVA patterns)
3. **Pattern Matching** - Regex-based detection with preset patterns

### Phase 3: Sorting Engine
1. **Category System** - Map Tailwind prefixes to categories (layout, spacing, typography, etc.)
2. **Sort Algorithm** - Multi-level sort:
   - By category order
   - Within category by official Tailwind order
   - Pseudo-class variants last
3. **Duplicate Removal** - HashSet-based deduplication

### Phase 4: File Processing
1. **Parallel Processing** - Use `rayon` for multi-threaded file handling
2. **File I/O** - Efficient reading/writing with proper error handling
3. **Dry Run Mode** - Show diffs without writing

### Phase 5: Advanced Features
1. **Custom Patterns** - User-defined regex support
2. **Multiple Output Modes** - stdout, file write, check-only
3. **Performance Optimization** - Lazy regex compilation, string interning

## Key Dependencies

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
oxc_parser = "0.x"
oxc_ast = "0.x"
oxc_span = "0.x"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
regex = "1"
rayon = "1"
thiserror = "1"
once_cell = "1"  # For lazy static regexes
walkdir = "2"
colored = "2"    # For colored output
```

## Technical Decisions

1. **Oxc over tree-sitter** - Faster, more accurate JS/TS parsing
2. **Rayon for parallelism** - Simple data parallelism for file processing
3. **Regex compilation** - Use `once_cell` for lazy static compilation
4. **Memory efficiency** - Process files streaming-style, don't load all into memory
5. **AST modification** - Create new AST nodes rather than mutating for safety

## Testing Strategy

1. **Unit tests** - Each module tested independently
2. **Snapshot tests** - Compare sorted output against expected
3. **Property tests** - Use `proptest` for edge cases
4. **Benchmarks** - Track performance regressions

This architecture prioritizes performance, correctness, and extensibility while keeping the codebase maintainable.
