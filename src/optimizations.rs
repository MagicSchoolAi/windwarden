/// Performance optimizations for WindWarden
///
/// This module contains optimized versions of core functionality
/// to improve performance for large-scale processing.
use crate::sorter::TailwindSorter;

/// Thread-local sorter to avoid repeated allocations and initialization
thread_local! {
    static LOCAL_SORTER: TailwindSorter = TailwindSorter::new();
}

/// Optimized class sorting that reuses thread-local sorter
pub fn sort_classes_optimized(class_string: &str) -> String {
    LOCAL_SORTER.with(|sorter| sorter.sort_classes(class_string))
}

/// String pool for reducing allocations in commonly used strings
pub struct StringPool {
    common_classes: std::collections::HashMap<&'static str, &'static str>,
}

impl StringPool {
    pub fn new() -> Self {
        let mut common_classes = std::collections::HashMap::new();

        // Pre-populate with common Tailwind classes
        let common = [
            "flex",
            "block",
            "inline",
            "hidden",
            "relative",
            "absolute",
            "p-1",
            "p-2",
            "p-3",
            "p-4",
            "p-5",
            "p-6",
            "p-8",
            "m-1",
            "m-2",
            "m-3",
            "m-4",
            "m-5",
            "m-6",
            "m-8",
            "text-sm",
            "text-base",
            "text-lg",
            "text-xl",
            "text-2xl",
            "bg-white",
            "bg-gray-100",
            "bg-gray-200",
            "bg-blue-500",
            "text-black",
            "text-white",
            "text-gray-500",
            "text-gray-900",
            "rounded",
            "rounded-md",
            "rounded-lg",
            "rounded-xl",
            "shadow",
            "shadow-sm",
            "shadow-md",
            "shadow-lg",
            "border",
            "border-2",
            "border-gray-200",
            "border-gray-300",
            "w-full",
            "h-full",
            "max-w-md",
            "max-w-lg",
            "max-w-xl",
        ];

        for &class in &common {
            common_classes.insert(class, class);
        }

        Self { common_classes }
    }

    pub fn get_or_intern<'a>(&self, s: &'a str) -> &'a str {
        self.common_classes.get(s).copied().unwrap_or(s)
    }
}

lazy_static::lazy_static! {
    static ref GLOBAL_STRING_POOL: StringPool = StringPool::new();
}

/// Memory-efficient string processing using string pool
pub fn intern_class(class: &str) -> &str {
    GLOBAL_STRING_POOL.get_or_intern(class)
}

/// Optimized batch processing for large numbers of files
pub struct BatchOptimizer {
    thread_count: usize,
    chunk_size: usize,
}

impl BatchOptimizer {
    pub fn new() -> Self {
        let thread_count = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);

        // Optimal chunk size based on thread count and typical file sizes
        let chunk_size = std::cmp::max(1, 100 / thread_count);

        Self {
            thread_count,
            chunk_size,
        }
    }

    pub fn get_optimal_thread_count(&self) -> usize {
        self.thread_count
    }

    pub fn get_optimal_chunk_size(&self, total_files: usize) -> usize {
        if total_files < self.thread_count {
            1
        } else {
            std::cmp::max(1, total_files / self.thread_count)
        }
    }
}

/// Fast path optimizations for common patterns
pub struct FastPathOptimizer;

impl FastPathOptimizer {
    /// Check if content needs processing (fast pre-check)
    pub fn needs_processing(content: &str) -> bool {
        // Quick heuristics to avoid expensive parsing for files that don't need processing

        // Must contain className or class attribute
        if !content.contains("className") && !content.contains("class=") {
            return false;
        }

        // Must contain common Tailwind patterns
        let has_tailwind_patterns = content.contains("p-")
            || content.contains("m-")
            || content.contains("bg-")
            || content.contains("text-")
            || content.contains("flex")
            || content.contains("grid");

        if !has_tailwind_patterns {
            return false;
        }

        // Quick check for multiple classes that might need sorting
        let class_count = content.matches(' ').count();
        class_count > 2 // Only process if likely to have multiple classes
    }

    /// Extract class strings more efficiently for simple cases
    pub fn extract_classes_fast(content: &str) -> Vec<(usize, usize, String)> {
        let mut classes = Vec::new();

        // Simple regex-free extraction for common patterns
        if let Some(start) = content.find("className=\"") {
            if let Some(end) = content[start + 11..].find('"') {
                let class_str = &content[start + 11..start + 11 + end];
                if !class_str.is_empty() && class_str.contains(' ') {
                    classes.push((start + 11, start + 11 + end, class_str.to_string()));
                }
            }
        }

        classes
    }
}

/// Memory usage optimizations
pub struct MemoryOptimizer {
    max_file_size: usize,
    string_deduplication: bool,
}

impl MemoryOptimizer {
    pub fn new() -> Self {
        Self {
            max_file_size: 10 * 1024 * 1024, // 10MB limit
            string_deduplication: true,
        }
    }

    pub fn should_process_file(&self, file_size: usize) -> bool {
        file_size <= self.max_file_size
    }

    pub fn optimize_string(&self, s: String) -> String {
        if self.string_deduplication && s.len() < 100 {
            // Use string interning for small strings
            intern_class(&s).to_string()
        } else {
            s
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_path_optimizer() {
        assert!(FastPathOptimizer::needs_processing(
            r#"<div className="p-4 m-2 bg-red-500">Test</div>"#
        ));
        assert!(!FastPathOptimizer::needs_processing(
            r#"<div>No classes here</div>"#
        ));
        assert!(!FastPathOptimizer::needs_processing(
            r#"<div className="single-class">Test</div>"#
        ));
    }

    #[test]
    fn test_sort_classes_optimized() {
        let result = sort_classes_optimized("p-4 flex m-2");
        assert_eq!(result, "flex m-2 p-4");
    }

    #[test]
    fn test_batch_optimizer() {
        let optimizer = BatchOptimizer::new();
        assert!(optimizer.get_optimal_thread_count() > 0);
        assert!(optimizer.get_optimal_chunk_size(100) > 0);
    }
}
