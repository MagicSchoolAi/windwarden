Here's what I'd provide to an agentic AI for successful implementation:

## 1. Tailwind Class Reference Data

```rust
// example mapping of Tailwind categories and their order
const TAILWIND_CATEGORIES: &[(&str, &[&str])] = &[
    ("layout", &["container", "box-", "block", "inline", "flex", "grid", "table-", "float-", "clear-", "isolate", "object-", "overflow-", "overscroll-", "position", "static", "fixed", "absolute", "relative", "sticky", "inset-", "top-", "right-", "bottom-", "left-", "z-"]),
    ("flexbox-grid", &["flex-", "order-", "grid-", "col-", "row-", "gap-", "justify-", "items-", "content-", "self-", "place-"]),
    ("spacing", &["p-", "px-", "py-", "pt-", "pr-", "pb-", "pl-", "m-", "mx-", "my-", "mt-", "mr-", "mb-", "ml-", "space-"]),
    ("sizing", &["w-", "min-w-", "max-w-", "h-", "min-h-", "max-h-", "size-"]),


## 2. Parser Patterns & AST Navigation

```rust
// Specific AST patterns to recognize
enum ClassPattern {
    // <div className="...">
    JSXAttribute { name: String, quote_style: QuoteStyle },

    // cn("...", "...")
    FunctionCall {
        name: String,
        args: Vec<ArgumentType>
    },

    // tw`...`
    TaggedTemplate { tag: String },

    // ["class1", "class2"]
    ArrayLiteral { depth: usize },

    // { className: "..." }
    ObjectProperty { key: String },
}

// AST visitor hints
struct VisitorContext {
    // Track if we're inside a dynamic expression
    in_template_expression: bool,
    // Track function call depth for nested cn() calls
    call_depth: usize,
    // Track if current string is computed
    is_dynamic: bool,
}
```

## 3. Performance Benchmarks & Constraints

```rust
// Performance targets
const PERF_TARGETS: &str = r#"
- Process 1000 files in < 1 second
- Memory usage < 100MB for large codebases
- Support files up to 10MB
- Regex compilation < 5ms startup time
"#;

// Optimization strategies
const OPTIMIZATIONS: &[&str] = &[
    "Use string interning for repeated class names",
    "Cache sorted results for identical class strings",
    "Early exit on files with no classes",
    "Batch file I/O operations",
    "Use SIMD for string searching where possible",
];
```

## 4. Integration Examples

```bash
# Git pre-commit hook
#!/bin/sh
windwarden --check-formatted src/
if [ $? -ne 0 ]; then
    echo "Tailwind classes are not sorted. Run 'windwarden --write src/' to fix."
    exit 1
fi

# VS Code task
{
    "label": "Sort Tailwind Classes",
    "type": "shell",
    "command": "windwarden",
    "args": ["--write", "${file}"],
    "problemMatcher": []
}

# CI/CD pipeline
- name: Check Tailwind Sorting
  run: windwarden --check-formatted .
```

## 5. Error Messages & Recovery

```rust
// User-friendly error messages
const ERROR_TEMPLATES: &[(&str, &str)] = &[
    ("PARSE_ERROR", "Failed to parse {file}: {details}. This might be due to syntax errors in the file."),
    ("REGEX_ERROR", "Invalid regex pattern '{pattern}': {error}"),
    ("CONFIG_ERROR", "Configuration error in {file}: {error}. Check the JSON syntax."),
    ("PERMISSION_ERROR", "Cannot write to {file}: Permission denied"),
];

// Recovery strategies
enum RecoveryStrategy {
    SkipFile,           // Continue with next file
    UseDefaults,        // Fall back to default config
    PartialProcess,     // Process what we can
    AbortOperation,     // Stop everything
}
```

## 6. Real-World Config Examples

```json
// Next.js project config
{
    "functionNames": ["cn", "cx"],
    "sortOrder": "official",
    "removeNullClasses": true,
    "ignorePaths": [".next", "node_modules"],
    "fileExtensions": ["tsx", "jsx"]
}

// Component library config
{
    "functionNames": ["cn", "clsx", "twMerge", "tv"],
    "categories": {
        "custom-animations": ["animate-", "motion-"]
    },
    "preserveDuplicates": false
}
```

