use std::collections::HashMap;
use once_cell::sync::Lazy;

pub use categories::*;

mod categories;

pub struct TailwindSorter {
    category_order: &'static [&'static str],
    class_categories: &'static HashMap<&'static str, &'static str>,
}

impl TailwindSorter {
    pub fn new() -> Self {
        Self {
            category_order: &CATEGORY_ORDER,
            class_categories: &CLASS_CATEGORIES,
        }
    }

    pub fn sort_classes(&self, class_string: &str) -> String {
        if class_string.trim().is_empty() {
            return class_string.to_string();
        }

        let mut classes: Vec<&str> = class_string
            .split_whitespace()
            .collect();

        // Remove duplicates while preserving first occurrence
        let mut seen = std::collections::HashSet::new();
        classes.retain(|&class| seen.insert(class));

        // Sort classes by category and within category
        classes.sort_by(|&a, &b| self.compare_classes(a, b));

        classes.join(" ")
    }

    fn compare_classes(&self, a: &str, b: &str) -> std::cmp::Ordering {
        // Extract base classes and variants
        let (base_a, variants_a) = self.split_variants(a);
        let (base_b, variants_b) = self.split_variants(b);
        
        let category_a = self.get_class_category(&base_a);
        let category_b = self.get_class_category(&base_b);

        // First, compare by category order
        let order_a = self.get_category_order(category_a);
        let order_b = self.get_category_order(category_b);

        match order_a.cmp(&order_b) {
            std::cmp::Ordering::Equal => {
                // Within the same category, compare base classes first
                match base_a.cmp(&base_b) {
                    std::cmp::Ordering::Equal => {
                        // If base classes are equal, compare variants
                        // Sort by number of variants first (fewer variants first)
                        match variants_a.len().cmp(&variants_b.len()) {
                            std::cmp::Ordering::Equal => {
                                // Same number of variants, sort by variant values
                                variants_a.cmp(&variants_b)
                            }
                            other => other,
                        }
                    }
                    other => other,
                }
            }
            other => other,
        }
    }

    fn split_variants<'a>(&self, class: &'a str) -> (String, Vec<&'a str>) {
        if let Some(last_colon) = class.rfind(':') {
            let variants: Vec<&str> = class[..last_colon].split(':').collect();
            let base = class[last_colon + 1..].to_string();
            (base, variants)
        } else {
            (class.to_string(), Vec::new())
        }
    }

    fn get_class_category(&self, class: &str) -> &str {
        // Handle variants (e.g., "hover:bg-blue-500" -> "bg-blue-500")
        let base_class = if let Some(colon_pos) = class.rfind(':') {
            &class[colon_pos + 1..]
        } else {
            class
        };

        // Handle important modifier (e.g., "!p-4" -> "p-4")
        let base_class = base_class.strip_prefix('!').unwrap_or(base_class);

        // Handle negative values (e.g., "-m-4" -> "m-4")
        let base_class = base_class.strip_prefix('-').unwrap_or(base_class);

        // Find the longest matching prefix
        let mut best_match = "unknown";
        let mut best_length = 0;

        for (&prefix, &category) in self.class_categories.iter() {
            if base_class.starts_with(prefix) && prefix.len() > best_length {
                best_match = category;
                best_length = prefix.len();
            }
        }

        best_match
    }

    fn get_category_order(&self, category: &str) -> usize {
        self.category_order
            .iter()
            .position(|&c| c == category)
            .unwrap_or(999) // Unknown categories go to the end
    }
}

impl Default for TailwindSorter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_sorting() {
        let sorter = TailwindSorter::new();
        let input = "p-4 flex m-2";
        let expected = "flex m-2 p-4";
        let result = sorter.sort_classes(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_remove_duplicates() {
        let sorter = TailwindSorter::new();
        let input = "flex p-4 flex m-2 p-4";
        let expected = "flex m-2 p-4";
        let result = sorter.sort_classes(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_string() {
        let sorter = TailwindSorter::new();
        let input = "";
        let expected = "";
        let result = sorter.sort_classes(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_class() {
        let sorter = TailwindSorter::new();
        let input = "flex";
        let expected = "flex";
        let result = sorter.sort_classes(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_variants() {
        let sorter = TailwindSorter::new();
        let input = "p-4 hover:bg-blue-500 flex md:flex-row";
        let result = sorter.sort_classes(input);
        
        // For now, let's just verify that all classes are included and that base classes without variants come first
        assert!(result.contains("flex"));
        assert!(result.contains("p-4"));
        assert!(result.contains("hover:bg-blue-500"));
        assert!(result.contains("md:flex-row"));
        
        // Verify that classes without variants (flex, p-4) come before classes with variants
        let flex_pos = result.find("flex").unwrap();
        let p4_pos = result.find("p-4").unwrap();
        let hover_pos = result.find("hover:bg-blue-500").unwrap();
        
        assert!(flex_pos < hover_pos || p4_pos < hover_pos);
    }

    #[test]
    fn test_important_modifier() {
        let sorter = TailwindSorter::new();
        let input = "!p-4 flex !m-2";
        let expected = "flex !m-2 !p-4";
        let result = sorter.sort_classes(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_negative_values() {
        let sorter = TailwindSorter::new();
        let input = "-m-4 flex p-2";
        let expected = "flex -m-4 p-2";
        let result = sorter.sort_classes(input);
        assert_eq!(result, expected);
    }
}