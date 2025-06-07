## Overview

WindWarden is a high-performance CLI tool for sorting and organizing Tailwind CSS classes. Built in Rust for speed and reliability, it automatically sorts Tailwind classes according to the official order, removes duplicates, and supports various file formats, utility function patterns, and customization

---

## Core Features

### 1. Tailwind CSS Class Sorting

* Automatically sorts Tailwind CSS classes according to official or custom order
* Removes duplicate classes (configurable)
* Preserves class functionality and improves readability
* Supports all Tailwind CSS versions and custom configs
* Recognizes `class`, `className`, and custom patterns (arrays, CVA, etc.)
* Supports all quote styles: single, double, and backticks

### 2. Multi-Pattern Support

#### Standard Patterns

* `className="flex items-center p-4"`
* `class="flex items-center p-4"`

#### Utility Function Support

* `cn("flex items-center", true && "flex p-4")`
* `twMerge("flex items-center", "p-4")`
* `clsx("flex items-center", "p-4")`
* `classNames("flex items-center", "p-4")`
* `classList("flex items-center", "p-4")`
* User-defined utility functions
* Tagged template literals (e.g., `tw\`bg-white p-4\`\`)

#### Advanced Array Patterns (CVA Support)

* Handles arrays of class strings
* Supports CVA-style syntax:

  ```js
  cva(['p-4', 'flex', 'items-center'], { variants: { ... } })
  // => cva(['flex', 'items-center', 'p-4'], { variants: { ... } })
  ```
* Supports nested arrays

### 3. File Format Support

* `.js`, `.jsx`, `.ts`, `.tsx`
* `.html`, `.vue`, `.svelte`, `.astro`, `.mdx`

### 4. Configuration Options

#### CLI Configuration

* Preset regex patterns:

  * `--preset-regex cn`
  * `--preset-regex tw-merge`
  * `--preset-regex clsx`
  * `--preset-regex utility-functions`
  * `--preset-regex combined`
  * `--preset-regex all`

#### JSON Configuration File

```json
{
  "sortOrder": ["custom", "class", "order"],
  "presetRegex": "all",
  "functionNames": ["myMerge", "myClsx"],
  "customRegex": "custom-pattern"
}
```

#### Advanced Configuration

* **Category Order**: Defines how categories are sorted
* **Categories**: Maps Tailwind class prefixes to named categories
* **Pseudo Classes Order**: Sort order for responsive and pseudo variants
* **Custom Prefixes**: Accepts non-standard prefixes for class detection and function calls

### 5. Advanced Sorting Options

#### Custom Sort Orders

* Define custom order in config files

### 6. Operation Modes

#### Processing Modes

* `--dry-run`: Preview changes
* `--write`: Apply changes
* Console output
* `--stdin`: Process standard input
* `--check-formatted`: Verify sort status with an exit code (useful for CI)

#### Performance Features

* Multi-threaded processing
* Filters relevant file types

### 7. Safety Features

#### Content Preservation

* AST-based targeting for edits
* Preserves quote style and indentation

#### File Safety

* Ignore patterns and directory filters
* Extension-based processing
* Dry-run and error-safe previewing
* Detailed error handling

### 8. Integration Support

#### Development Tools

* Exit codes for CI/CD
* Compatible with Git hooks
* Works with VS Code, Vim, etc.

---

## Technical Architecture

### Performance Characteristics

* Language: Rust
* Low memory usage
* Parallel processing
* Optimized, compiled regexes
* **Parser and AST powered by ****************************************************************[Oxc](https://github.com/oxc-project/oxc)****************************************************************:**

  * Oxc is the fastest and most conformant JS/TS parser in Rust
  * Memory arena (bumpalo) for fast allocation/deallocation
  * CompactString for inlined short strings
  * AST avoids ambiguous estree nodes by using specific types like `BindingIdentifier` vs `IdentifierReference`
  * Parser defers symbol resolution and scope binding to semantic analysis

---

## Use Cases

### 1. Code Consistency

* Enforce Tailwind class ordering
* Reduce VCS diff noise
* Improve readability and maintainability

### 2. Large Codebase Management

* Scale to thousands of files
* Automate formatting in CI/CD
* Cross-developer consistency

### 3. Component Library Development

* Sort reusable component classes
* Full CVA support
* Consistent component variant styling

### 4. Migration and Refactoring

* Standardize during Tailwind upgrades
* Format legacy codebases
* Automate bulk refactors
