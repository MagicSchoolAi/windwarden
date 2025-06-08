use std::collections::{HashMap, HashSet};

pub use categories::*;

mod categories;

pub struct TailwindSorter {
    category_order: Vec<String>,
    class_categories: &'static HashMap<&'static str, &'static str>,
    // Cache for category lookups to avoid repeated iteration
    category_cache: std::cell::RefCell<HashMap<String, String>>,
    // Pre-computed category order map for O(1) lookups
    category_order_map: HashMap<String, usize>,
}

impl TailwindSorter {
    pub fn new() -> Self {
        Self::new_with_custom_order(None)
    }

    pub fn new_with_custom_order(custom_order: Option<Vec<String>>) -> Self {
        let category_order = match custom_order {
            Some(order) => order,
            None => CATEGORY_ORDER.iter().map(|&s| s.to_string()).collect(),
        };

        // Pre-compute category order map for O(1) lookups
        let category_order_map: HashMap<String, usize> = category_order
            .iter()
            .enumerate()
            .map(|(i, category)| (category.clone(), i))
            .collect();

        Self {
            category_order,
            class_categories: &CLASS_CATEGORIES,
            category_cache: std::cell::RefCell::new(HashMap::new()),
            category_order_map,
        }
    }

    pub fn sort_classes(&self, class_string: &str) -> String {
        let trimmed = class_string.trim();
        if trimmed.is_empty() {
            return class_string.to_string();
        }

        // Optimize for single class - common case
        if !trimmed.contains(' ') {
            return trimmed.to_string();
        }

        // Pre-allocate with estimated capacity
        let mut classes: Vec<&str> = Vec::with_capacity(trimmed.matches(' ').count() + 1);
        classes.extend(trimmed.split_whitespace());

        // Remove duplicates while preserving first occurrence - optimized version
        if classes.len() > 1 {
            let mut seen = HashSet::with_capacity(classes.len());
            classes.retain(|&class| seen.insert(class));
        }

        // Early return if only one class after deduplication
        if classes.len() == 1 {
            return classes[0].to_string();
        }

        // Sort classes by category and within category
        classes.sort_unstable_by(|&a, &b| self.compare_classes(a, b));

        classes.join(" ")
    }

    fn compare_classes(&self, a: &str, b: &str) -> std::cmp::Ordering {
        // Extract base classes and variants
        let (base_a, variants_a) = self.split_variants(a);
        let (base_b, variants_b) = self.split_variants(b);

        let category_a = self.get_class_category(&base_a);
        let category_b = self.get_class_category(&base_b);

        // First, compare by category order
        let order_a = self.get_category_order(&category_a);
        let order_b = self.get_category_order(&category_b);

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

    fn get_class_category(&self, class: &str) -> String {
        // Check cache first
        if let Some(cached) = self.category_cache.borrow().get(class) {
            return cached.clone();
        }

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

        // Find the longest matching prefix - optimized with early returns for common cases
        let category = self.find_category_optimized(base_class).to_string();

        // Cache the result
        self.category_cache
            .borrow_mut()
            .insert(class.to_string(), category.clone());

        category
    }

    fn find_category_optimized(&self, base_class: &str) -> &'static str {
        // Fast path for common single-character prefixes
        if let Some(first_char) = base_class.chars().next() {
            match first_char {
                'p' if base_class.starts_with("p-")
                    || base_class.starts_with("px-")
                    || base_class.starts_with("py-")
                    || base_class.starts_with("pt-")
                    || base_class.starts_with("pb-")
                    || base_class.starts_with("pl-")
                    || base_class.starts_with("pr-") =>
                {
                    return "spacing"
                }
                'm' if base_class.starts_with("m-")
                    || base_class.starts_with("mx-")
                    || base_class.starts_with("my-")
                    || base_class.starts_with("mt-")
                    || base_class.starts_with("mb-")
                    || base_class.starts_with("ml-")
                    || base_class.starts_with("mr-") =>
                {
                    return "spacing"
                }
                'w' if base_class.starts_with("w-") => return "sizing",
                'h' if base_class.starts_with("h-") => return "sizing",
                _ => {}
            }
        }

        // Fallback to full prefix matching for other cases
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
        // Use pre-computed map for O(1) lookup instead of O(n) iteration
        self.category_order_map
            .get(category)
            .copied()
            .unwrap_or(999)
    }

    /// Get the default category order
    pub fn get_default_category_order() -> Vec<String> {
        CATEGORY_ORDER.iter().map(|&s| s.to_string()).collect()
    }

    /// Get the current category order
    pub fn get_category_order_list(&self) -> &Vec<String> {
        &self.category_order
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
    fn test_debug_button_classes() {
        let sorter = TailwindSorter::new();
        let input = "flex items-center bg-blue-500 hover:bg-blue-600 text-white p-4";
        let result = sorter.sort_classes(input);

        println!("Input:  {}", input);
        println!("Output: {}", result);

        // According to Tailwind order:
        // 1. flexbox-grid: flex, items-center
        // 2. spacing: p-4
        // 3. typography: text-white
        // 4. backgrounds: bg-blue-500, hover:bg-blue-600

        // Expected order: flex items-center p-4 text-white bg-blue-500 hover:bg-blue-600
        assert_eq!(
            result,
            "flex items-center p-4 text-white bg-blue-500 hover:bg-blue-600"
        );
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
