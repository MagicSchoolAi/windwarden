# Claude Code Context for WindWarden

## Project Overview
You're building WindWarden, a Rust-based CLI tool for sorting Tailwind CSS classes using AST parsing (not regex).

## Current Phase
**Phase 1**: Basic functionality - single file processing with JSX className sorting

## Key Technical Decisions
- **Parser**: Use Oxc (not tree-sitter or regex) for AST parsing
- **No Regex**: All pattern matching should be AST-based
- **Testing**: Use test cases from `tests/fixtures/TESTCASEREFERENCES.ts`
- **Error Handling**: Use `thiserror` for custom error types
- **Sorting**: Follow Tailwind's official category order (see RESOURCES.md)

## Development Workflow

### 1. Always Run Tests After Changes
```bash
cargo test
cargo test -- --nocapture  # To see println! output
```

### 2. Format and Lint Before Committing
```bash
cargo fmt
cargo clippy -- -D warnings
```

### 3. Test Against Real Files
```bash
# Test basic functionality
echo '<div className="p-4 flex m-2">' | cargo run -- --stdin

# Test on a real file
cargo run -- --dry-run tests/fixtures/sample.tsx
```

## File Structure Conventions
- Keep modules small and focused
- Use `mod.rs` for public API of each module
- Implementation details go in separate files
- Always add unit tests in the same file as the implementation

## Oxc AST Visitor Pattern
When implementing AST visitors, follow this pattern:
```rust
impl<'a> Visit<'a> for ClassExtractor {
    fn visit_jsx_attribute(&mut self, node: &JSXAttribute<'a>) {
        // Extract className/class attributes
        walk_jsx_attribute(self, node); // Don't forget to walk!
    }
}
```

## Common Pitfalls to Avoid

1. **Don't use regex for parsing** - We're using AST for accuracy
2. **Don't mutate AST nodes** - Create new ones for safety
3. **Don't load entire files into memory** - Process streaming when possible
4. **Don't forget to handle quote styles** - Preserve original quotes
5. **Don't sort dynamic expressions** - Skip template literals with variables

## Testing Strategy

### Unit Test Template
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_jsx_sorting() {
        let input = r#"<div className="p-4 flex m-2">"#;
        let expected = r#"<div className="flex m-2 p-4">"#;

        let result = process_file_content(input, "test.tsx").unwrap();
        assert_eq!(result, expected);
    }
}
```

### Integration Test Structure
```rust
// tests/integration/basic_sorting.rs
use windwarden::process_file;

#[test]
fn sorts_basic_jsx_file() {
    let input_path = "tests/fixtures/basic.tsx";
    let output = process_file(input_path, ProcessOptions::default()).unwrap();
    // Compare with expected output
}
```

## Performance Considerations
- Start simple, optimize later
- Use `&str` where possible instead of `String`
- Consider using `Cow<str>` for potentially modified strings
- Benchmark before optimizing

## Debugging Tips

1. **AST Visualization**: Print AST nodes during development
```rust
dbg!(&node); // Remove before committing
```

2. **Span Information**: Use spans for precise source location
```rust
println!("Found class at {:?}", string_literal.span);
```

3. **Test Single Files**: Focus on one test case at a time
```bash
cargo test test_basic_jsx_sorting -- --nocapture
```

## Progress Tracking

### Phase 1 Checklist
- [ ] Project setup with dependencies
- [ ] CLI argument parsing (clap)
- [ ] Basic file I/O
- [ ] Oxc AST parser integration
- [ ] JSX className attribute visitor
- [ ] Tailwind class sorter
- [ ] Basic test passing (basic_jsx_classname)

### Next Phases (Don't implement yet)
- Phase 2: Function calls (cn, twMerge)
- Phase 3: Arrays and template literals
- Phase 4: Config files and parallel processing
- Phase 5: Advanced patterns and optimizations

## Questions to Ask Yourself
1. Is this the simplest solution that works?
2. Have I added a test for this functionality?
3. Am I preserving the original formatting where needed?
4. Will this handle malformed input gracefully?

## Resources Priority
1. First read: `ARCHITECTUREPLAN.md` for structure
2. Reference: `RESOURCES.md` for Tailwind rules and AST patterns
3. Test with: `TESTCASEREFERENCES.ts` for validation
4. Context: `README.md` for feature requirements

## Remember
- We're building iteratively - Phase 1 first!
- Correctness over performance initially
- Use the test cases to guide development
- Ask for clarification if something is ambiguous
