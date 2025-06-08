use std::fs;

use crate::atomic;
use crate::config::Config;
use crate::parser::{FileParser, PatternType, QuoteStyle};
use crate::sorter::TailwindSorter;
use crate::{ProcessOptions, Result, WindWardenError};

pub struct FileProcessor {
    parser: FileParser,
    sorter: TailwindSorter,
    config: Option<Config>,
}

impl FileProcessor {
    pub fn new() -> Self {
        Self {
            parser: FileParser::new(),
            sorter: TailwindSorter::new(),
            config: None,
        }
    }

    pub fn new_with_config(config: &Config) -> Self {
        // Always use config manager to get effective function names (defaults + custom)
        let temp_manager = crate::config::ConfigManager::new_with_config(config.clone(), None);
        let all_functions = temp_manager.get_function_names();

        let parser = FileParser::new_with_custom_functions(all_functions);

        // Create sorter with custom order if specified
        let sorter = if config.sort_order == "custom" && !config.custom_order.is_empty() {
            TailwindSorter::new_with_custom_order(Some(config.custom_order.clone()))
        } else {
            TailwindSorter::new()
        };

        Self {
            parser,
            sorter,
            config: Some(config.clone()),
        }
    }

    pub fn process_file(&self, file_path: &str, options: ProcessOptions) -> Result<String> {
        let content = fs::read_to_string(file_path).map_err(WindWardenError::Io)?;

        self.process_content(&content, file_path, options)
    }

    pub fn process_content(
        &self,
        content: &str,
        file_path: &str,
        options: ProcessOptions,
    ) -> Result<String> {
        // Parse the file to find class matches
        let matches = self.parser.parse_file(file_path, content)?;

        if matches.is_empty() {
            // No classes found, return original content or empty based on mode
            return if options.dry_run || !options.write {
                Ok(content.to_string())
            } else {
                Ok(String::new())
            };
        }

        // Sort matches by position (start offset) in reverse order
        // This allows us to replace from end to beginning without affecting positions
        let mut sorted_matches = matches;
        sorted_matches.sort_by(|a, b| b.start.cmp(&a.start));

        let mut result = content.to_string();
        let mut changes_made = false;

        for class_match in sorted_matches {
            let sorted_classes = self.sorter.sort_classes(&class_match.original);

            // Check if sorting actually changed anything
            if sorted_classes != class_match.original {
                changes_made = true;

                // Create the replacement string with appropriate quotes
                let quote_char = match class_match.quote_style {
                    QuoteStyle::Single => '\'',
                    QuoteStyle::Double => '"',
                    QuoteStyle::Backtick => '`',
                };

                let replacement = format!("{}{}{}", quote_char, sorted_classes, quote_char);

                // Handle different pattern types differently
                match &class_match.pattern_type {
                    PatternType::JSXAttribute => {
                        // For JSX attributes, use string search as before
                        let search_pattern =
                            format!("{}{}{}", quote_char, class_match.original, quote_char);
                        if let Some(start_pos) = result.find(&search_pattern) {
                            let end_pos = start_pos + search_pattern.len();
                            result.replace_range(start_pos..end_pos, &replacement);
                        }
                    }
                    PatternType::FunctionCall { .. } => {
                        // For function calls, use span positions since they're more accurate
                        // The spans should be correct from the AST parser
                        if class_match.start < result.len() && class_match.end <= result.len() {
                            result.replace_range(class_match.start..class_match.end, &replacement);
                        }
                    }
                    PatternType::TemplateLiteral { .. } => {
                        // For template literals, we need to replace just the content, preserving backticks
                        let template_replacement = format!("`{}`", sorted_classes);

                        // Use span positions for template literals
                        if class_match.start < result.len() && class_match.end <= result.len() {
                            result.replace_range(
                                class_match.start..class_match.end,
                                &template_replacement,
                            );
                        }
                    }
                    PatternType::ArrayElement { .. } => {
                        // For array elements, use span positions
                        if class_match.start < result.len() && class_match.end <= result.len() {
                            result.replace_range(class_match.start..class_match.end, &replacement);
                        }
                    }
                    PatternType::Array { elements: _ } => {
                        // For arrays, sort the combined classes and rebuild the array
                        let element_quote_char = match class_match.quote_style {
                            QuoteStyle::Single => '\'',
                            QuoteStyle::Double => '"',
                            QuoteStyle::Backtick => '`', // Though arrays shouldn't use backticks
                        };

                        let sorted_elements: Vec<String> = sorted_classes
                            .split_whitespace()
                            .map(|s| format!("{}{}{}", element_quote_char, s, element_quote_char))
                            .collect();
                        let array_replacement = format!("[{}]", sorted_elements.join(", "));

                        // Use span positions for arrays
                        if class_match.start < result.len() && class_match.end <= result.len() {
                            result.replace_range(
                                class_match.start..class_match.end,
                                &array_replacement,
                            );
                        }
                    }
                    PatternType::BinaryExpression {
                        left_content,
                        right_content: _,
                    } => {
                        // For binary expressions (string concatenation), rebuild with sorted classes
                        let quote_char = match class_match.quote_style {
                            QuoteStyle::Single => '\'',
                            QuoteStyle::Double => '"',
                            QuoteStyle::Backtick => '`',
                        };

                        // Split the sorted classes back into reasonable chunks
                        // Try to preserve the original split pattern as much as possible
                        let sorted_words: Vec<&str> = sorted_classes.split_whitespace().collect();
                        let left_word_count = left_content.split_whitespace().count();

                        let (left_words, right_words) = if left_word_count <= sorted_words.len() {
                            sorted_words.split_at(left_word_count)
                        } else {
                            // If original left had more words, split roughly in half
                            let split_point = sorted_words.len() / 2;
                            sorted_words.split_at(split_point)
                        };

                        let new_left = left_words.join(" ");
                        let new_right = right_words.join(" ");

                        let binary_replacement = format!(
                            "{}{}{} + {}{}{}",
                            quote_char, new_left, quote_char, quote_char, new_right, quote_char
                        );

                        // Use span positions for binary expressions
                        if class_match.start < result.len() && class_match.end <= result.len() {
                            result.replace_range(
                                class_match.start..class_match.end,
                                &binary_replacement,
                            );
                        }
                    }
                }
            }
        }

        // Handle different processing modes
        if options.check_formatted {
            // For check_formatted mode, we don't return an error for unsorted classes
            // We just return the original content and let the caller handle the result
            return Ok(content.to_string());
        }

        if options.write && changes_made {
            self.write_file_safely(file_path, &result)?;
            Ok(String::new()) // No output needed when writing to file
        } else if options.dry_run || !options.write {
            Ok(result)
        } else {
            Ok(String::new()) // No changes and not in preview mode
        }
    }

    /// Write file content using the configured safety settings
    fn write_file_safely(&self, file_path: &str, content: &str) -> Result<()> {
        // Use configuration if available, otherwise use defaults
        let safety_config = self
            .config
            .as_ref()
            .map(|c| &c.safety)
            .cloned()
            .unwrap_or_default();

        if safety_config.atomic_writes {
            if safety_config.create_backups {
                atomic::operations::write_file_with_backup(file_path, content)?;
            } else {
                atomic::operations::write_file(file_path, content)?;
            }

            // Optionally verify the write
            if safety_config.verify_writes {
                let written_content = fs::read_to_string(file_path)
                    .map_err(|e| WindWardenError::from_io_error(e, Some(file_path)))?;

                if written_content != content {
                    return Err(WindWardenError::internal_error(format!(
                        "File verification failed for {}: content mismatch",
                        file_path
                    )));
                }
            }
        } else {
            // Fall back to direct write if atomic writes are disabled
            fs::write(file_path, content)
                .map_err(|e| WindWardenError::from_io_error(e, Some(file_path)))?;
        }

        Ok(())
    }
}

impl Default for FileProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_processing() {
        let processor = FileProcessor::new();
        let input = r#"<div className="p-4 flex m-2">"#;
        let expected = r#"<div className="flex m-2 p-4">"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_no_changes_needed() {
        let processor = FileProcessor::new();
        let input = r#"<div className="flex m-2 p-4">"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, input);
    }

    #[test]
    fn test_preserve_quote_style() {
        let processor = FileProcessor::new();
        let input = r#"<div className='p-4 flex m-2'>"#;
        let expected = r#"<div className='flex m-2 p-4'>"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_attributes() {
        let processor = FileProcessor::new();
        let input = r#"<div className="p-4 flex" id="test"><span className="m-2 text-sm">Text</span></div>"#;
        let expected = r#"<div className="flex p-4" id="test"><span className="m-2 text-sm">Text</span></div>"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_basic_cn_function() {
        let processor = FileProcessor::new();
        let input = r#"cn("p-4 flex m-2")"#;
        let expected = r#"cn("flex m-2 p-4")"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_cn_args() {
        let processor = FileProcessor::new();
        let input = r#"cn("p-4 flex", "m-2 items-center")"#;
        let expected = r#"cn("flex p-4", "items-center m-2")"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tw_merge_function() {
        // Test multiline JSX with twMerge
        let processor = FileProcessor::new();
        let input = r#"twMerge("p-4 flex m-2", "p-2 items-center")"#;
        let expected = r#"twMerge("flex m-2 p-4", "items-center p-2")"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_clsx_function() {
        let processor = FileProcessor::new();
        let input = r#"clsx("p-4 flex m-2 items-center")"#;
        let expected = r#"clsx("flex items-center m-2 p-4")"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_class_names_function() {
        // Test multiline JSX with classNames
        let processor = FileProcessor::new();
        let input = r#"classNames("p-4 flex m-2 items-center")"#;
        let expected = r#"classNames("flex items-center m-2 p-4")"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_class_list_function() {
        // Test multiline JSX with classList
        let processor = FileProcessor::new();
        let input = r#"classList("p-4 flex m-2 items-center")"#;
        let expected = r#"classList("flex items-center m-2 p-4")"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_cn_with_conditionals() {
        let processor = FileProcessor::new();
        let input = r#"cn("p-4 flex", isActive && "bg-blue-500 text-white", "m-2")"#;
        // Note: String inside conditional should ALSO be sorted
        let expected = r#"cn("flex p-4", isActive && "text-white bg-blue-500", "m-2")"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_cn_with_objects() {
        let processor = FileProcessor::new();
        let input = r#"cn("p-4 flex", { "bg-blue-500": isActive }, "m-2")"#;
        let expected = r#"cn("flex p-4", { "bg-blue-500": isActive }, "m-2")"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_static_template_literal() {
        let processor = FileProcessor::new();
        let input = r#"const x = `p-4 flex m-2 items-center`"#;
        let expected = r#"const x = `flex items-center m-2 p-4`"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tagged_template_literal() {
        let processor = FileProcessor::new();
        let input = r#"const styles = tw`p-4 flex m-2 items-center`"#;
        let expected = r#"const styles = tw`flex items-center m-2 p-4`"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_dynamic_template_literal_skipped() {
        let processor = FileProcessor::new();
        let input = r#"const x = `p-4 ${baseStyles} m-2 items-center`"#;
        let expected = r#"const x = `p-4 ${baseStyles} m-2 items-center`"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_basic_array() {
        let processor = FileProcessor::new();
        let input = r#"const arr = ["p-4", "flex", "m-2", "items-center"]"#;
        let expected = r#"const arr = ["flex", "items-center", "m-2", "p-4"]"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_cva_with_arrays() {
        let processor = FileProcessor::new();
        let input = r#"cva(['p-4', 'flex'], { variants: {} })"#;
        let expected = r#"cva(['flex', 'p-4'], { variants: {} })"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    // ===== COMPREHENSIVE PHASE 3 TESTS =====

    #[test]
    fn test_template_literal_in_jsx() {
        let processor = FileProcessor::new();
        let input = r#"<div className={`p-4 flex m-2 items-center`}>"#;
        let expected = r#"<div className={`flex items-center m-2 p-4`}>"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_template_literals() {
        let processor = FileProcessor::new();
        let input = r#"
const styles1 = `p-4 flex m-2 items-center`;
const styles2 = `p-4 text-lg border-2 bg-white rounded-lg`;
"#;
        let expected = r#"
const styles1 = `flex items-center m-2 p-4`;
const styles2 = `p-4 text-lg bg-white border-2 rounded-lg`;
"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tagged_templates_multiple_tags() {
        let processor = FileProcessor::new();
        let input = r#"
const tailwind = tw`p-4 flex m-2 items-center`;
const styles = css`p-4 flex m-2 items-center`;
"#;
        let expected = r#"
const tailwind = tw`flex items-center m-2 p-4`;
const styles = css`flex items-center m-2 p-4`;
"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_array_with_join() {
        let processor = FileProcessor::new();
        let input = r#"const classes = ["p-4", "flex", "m-2", "items-center"]"#;
        let expected = r#"const classes = ["flex", "items-center", "m-2", "p-4"]"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mixed_array_quotes() {
        let processor = FileProcessor::new();
        let input = r#"
const doubleQuotes = ["p-4", "flex", "m-2"];
const singleQuotes = ['p-2', 'text-lg', 'bg-white', 'border-2'];
"#;
        let expected = r#"
const doubleQuotes = ["flex", "m-2", "p-4"];
const singleQuotes = ['p-2', 'text-lg', 'bg-white', 'border-2'];
"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_complex_responsive_variants() {
        let processor = FileProcessor::new();
        let input =
            r#"<div className="p-4 hover:bg-blue-500 flex md:flex-row flex-col hover:p-6 sm:p-2">"#;
        let expected =
            r#"<div className="flex flex-col md:flex-row sm:p-2 p-4 hover:p-6 hover:bg-blue-500">"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_arbitrary_values_and_important() {
        let processor = FileProcessor::new();
        let input = r#"<div className="!p-4 flex !m-2 items-center p-[23px] m-[1.5rem]">"#;
        let expected = r#"<div className="flex items-center !m-2 !p-4 m-[1.5rem] p-[23px]">"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_negative_values() {
        let processor = FileProcessor::new();
        let input = r#"<div className="-m-4 flex p-2 -translate-x-2 items-center">"#;
        let expected = r#"<div className="flex items-center -m-4 p-2 -translate-x-2">"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_template_literal_with_function_call() {
        let processor = FileProcessor::new();
        let input = r#"cn(`p-4 flex m-2 items-center`)"#;
        let expected = r#"cn(`flex items-center m-2 p-4`)"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_cva_basic_comprehensive() {
        let processor = FileProcessor::new();
        let input = r#"cva(['p-4', 'flex', 'm-2', 'items-center'], { variants: {} })"#;
        let expected = r#"cva(['flex', 'items-center', 'm-2', 'p-4'], { variants: {} })"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mixed_patterns_in_same_file() {
        let processor = FileProcessor::new();
        let input = r#"
export function Button({ className, ...props }) {
  return (
    <button
      className={cn(
        "p-4 flex items-center bg-blue-500 text-white font-semibold rounded-lg",
        className
      )}
      {...props}
    />
  );
}

const baseStyles = `p-2 border-2 rounded text-sm`;
const variants = ['hover:bg-gray-100', 'focus:ring-2', 'active:bg-gray-200'];
"#;
        let expected = r#"
export function Button({ className, ...props }) {
  return (
    <button
      className={cn(
        "flex items-center p-4 font-semibold text-white bg-blue-500 rounded-lg",
        className
      )}
      {...props}
    />
  );
}

const baseStyles = `p-2 text-sm border-2 rounded`;
const variants = ['hover:bg-gray-100', 'focus:ring-2', 'active:bg-gray-200'];
"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_whitespace_normalization() {
        let processor = FileProcessor::new();
        let input = r#"<div className="  p-4   flex    m-2  items-center  ">"#;
        let expected = r#"<div className="flex items-center m-2 p-4">"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_duplicate_removal() {
        let processor = FileProcessor::new();
        let input = r#"<div className="flex p-4 flex items-center p-4">"#;
        let expected = r#"<div className="flex items-center p-4">"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    // ===== SKIP CASES (should not be modified) =====

    #[test]
    fn test_dynamic_template_literal_with_interpolation() {
        let processor = FileProcessor::new();
        let input = r#"const classes = `p-4 ${dynamic} flex m-2 ${otherVar} items-center`;"#;
        let expected = r#"const classes = `p-4 ${dynamic} flex m-2 ${otherVar} items-center`;"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mixed_array_with_non_classes() {
        let processor = FileProcessor::new();
        let input = r#"const mixed = ["p-4", someVariable, "flex"];"#;
        let expected = r#"const mixed = ["p-4", someVariable, "flex"];"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_non_tailwind_strings() {
        let processor = FileProcessor::new();
        let input = r#"const notClasses = ["hello", "world", "test"];"#;
        let expected = r#"const notClasses = ["hello", "world", "test"];"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_cva_with_nested_variants_basic() {
        let processor = FileProcessor::new();
        let input =
            r#"cva(['p-4', 'flex'], { variants: { size: { sm: ['text-sm', 'p-2', 'gap-1'] } } })"#;
        let expected =
            r#"cva(['flex', 'p-4'], { variants: { size: { sm: ['gap-1', 'p-2', 'text-sm'] } } })"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_cva_with_multiple_nested_variants() {
        let processor = FileProcessor::new();
        let input = r#"cva(['p-4', 'flex'], {
  variants: {
    size: {
      sm: ['text-sm', 'p-2', 'gap-1'],
      lg: ['text-lg', 'p-6', 'gap-4']
    }
  }
})"#;
        let expected = r#"cva(['flex', 'p-4'], {
  variants: {
    size: {
      sm: ['gap-1', 'p-2', 'text-sm'],
      lg: ['gap-4', 'p-6', 'text-lg']
    }
  }
})"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_cva_with_complex_nested_variants() {
        let processor = FileProcessor::new();
        let input = r#"cva(['p-4', 'flex'], {
  variants: {
    size: {
      sm: ['text-sm', 'p-2', 'gap-1'],
      lg: ['text-lg', 'p-6', 'gap-4']
    },
    variant: {
      primary: ['bg-blue-500', 'text-white', 'hover:bg-blue-600'],
      secondary: ['bg-gray-200', 'text-gray-900', 'hover:bg-gray-300']
    }
  }
})"#;
        let expected = r#"cva(['flex', 'p-4'], {
  variants: {
    size: {
      sm: ['gap-1', 'p-2', 'text-sm'],
      lg: ['gap-4', 'p-6', 'text-lg']
    },
    variant: {
      primary: ['bg-blue-500', 'text-white', 'hover:bg-blue-600'],
      secondary: ['bg-gray-200', 'text-gray-900', 'hover:bg-gray-300']
    }
  }
})"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    // ===== COMPREHENSIVE COMPLEX CVA TESTS =====

    #[test]
    fn test_cva_from_testcase_reference() {
        let processor = FileProcessor::new();
        let input = r#"cva(['p-4', 'flex'], {
  variants: {
    size: {
      sm: ['text-sm', 'p-2', 'gap-1'],
      lg: ['text-lg', 'p-6', 'gap-4']
    }
  }
})"#;
        let expected = r#"cva(['flex', 'p-4'], {
  variants: {
    size: {
      sm: ['gap-1', 'p-2', 'text-sm'],
      lg: ['gap-4', 'p-6', 'text-lg']
    }
  }
})"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_cva_deeply_nested_variants() {
        let processor = FileProcessor::new();
        let input = r#"cva(['p-4', 'flex'], {
  variants: {
    size: {
      sm: ['text-sm', 'p-2'],
      lg: ['text-lg', 'p-6']
    },
    state: {
      default: ['bg-white', 'text-black'],
      active: ['bg-blue-500', 'text-white'],
      disabled: ['bg-gray-100', 'text-gray-400']
    }
  },
  compoundVariants: [
    {
      size: 'sm',
      state: 'active',
      class: ['font-bold', 'shadow-sm', 'border-2']
    }
  ]
})"#;
        let expected = r#"cva(['flex', 'p-4'], {
  variants: {
    size: {
      sm: ['p-2', 'text-sm'],
      lg: ['p-6', 'text-lg']
    },
    state: {
      default: ['text-black', 'bg-white'],
      active: ['text-white', 'bg-blue-500'],
      disabled: ['text-gray-400', 'bg-gray-100']
    }
  },
  compoundVariants: [
    {
      size: 'sm',
      state: 'active',
      class: ['font-bold', 'shadow-sm', 'border-2']
    }
  ]
})"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_cva_definitions() {
        let processor = FileProcessor::new();
        let input = r#"
const buttonVariants = cva(['p-4', 'flex'], {
  variants: {
    size: { sm: ['text-sm', 'p-2'] }
  }
});

const cardVariants = cva(['bg-white', 'rounded', 'shadow'], {
  variants: {
    padding: { lg: ['p-6', 'gap-4'] }
  }
});
"#;
        let expected = r#"
const buttonVariants = cva(['flex', 'p-4'], {
  variants: {
    size: { sm: ['p-2', 'text-sm'] }
  }
});

const cardVariants = cva(['bg-white', 'rounded', 'shadow'], {
  variants: {
    padding: { lg: ['gap-4', 'p-6'] }
  }
});
"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_cva_with_mixed_quote_styles() {
        let processor = FileProcessor::new();
        let input = r#"cva(["p-4", "flex"], {
  variants: {
    size: {
      sm: ['text-sm', 'p-2'],
      lg: ["text-lg", "p-6"]
    }
  }
})"#;
        let expected = r#"cva(["flex", "p-4"], {
  variants: {
    size: {
      sm: ['p-2', 'text-sm'],
      lg: ["p-6", "text-lg"]
    }
  }
})"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_cva_with_empty_and_mixed_arrays() {
        let processor = FileProcessor::new();
        let input = r#"cva(['p-4', 'flex'], {
  variants: {
    size: {
      sm: ['text-sm', 'p-2'],
      md: [],
      lg: ['text-lg', someVariable, 'p-6']
    }
  }
})"#;
        let expected = r#"cva(['flex', 'p-4'], {
  variants: {
    size: {
      sm: ['p-2', 'text-sm'],
      md: [],
      lg: ['text-lg', someVariable, 'p-6']
    }
  }
})"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    // ===== NESTED FUNCTION CALLS TESTS =====

    #[test]
    fn test_nested_cn_calls_basic() {
        let processor = FileProcessor::new();
        let input = r#"cn("p-4", cn("m-2 flex", "items-center"))"#;
        let expected = r#"cn("p-4", cn("flex m-2", "items-center"))"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_deeply_nested_cn_calls() {
        let processor = FileProcessor::new();
        let input = r#"cn("p-4 flex", cn("m-2 items-center", cn("bg-white text-black")))"#;
        let expected = r#"cn("flex p-4", cn("items-center m-2", cn("text-black bg-white")))"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_mixed_function_calls() {
        let processor = FileProcessor::new();
        let input = r#"cn("p-4 flex", twMerge("m-2 items-center", "bg-blue-500 text-white"))"#;
        let expected = r#"cn("flex p-4", twMerge("items-center m-2", "text-white bg-blue-500"))"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_calls_from_testcase_reference() {
        let processor = FileProcessor::new();
        // This test matches the pattern from TESTCASEREFERENCE.ts
        let input = r#"cn("p-4", cn("flex m-2", "items-center"))"#;
        let expected = r#"cn("p-4", cn("flex m-2", "items-center"))"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_complex_nested_with_jsx() {
        let processor = FileProcessor::new();
        let input = r#"<div className={cn("p-4 flex", cn("m-2 items-center", isActive && "bg-blue-500 text-white"))}>"#;
        let expected = r#"<div className={cn("flex p-4", cn("items-center m-2", isActive && "text-white bg-blue-500"))}>"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_object_property_classname() {
        let processor = FileProcessor::new();
        let input = r#"const props = { className: "p-4 flex m-2 items-center" }"#;
        let expected = r#"const props = { className: "flex items-center m-2 p-4" }"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_object_property_class() {
        let processor = FileProcessor::new();
        let input = r#"const props = { class: "text-sm font-bold p-2 bg-blue-500" }"#;
        let expected = r#"const props = { class: "p-2 font-bold text-sm bg-blue-500" }"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_string_concatenation_simple() {
        let processor = FileProcessor::new();
        let input = r#""p-4 flex m-2" + "items-center bg-white""#;
        // The algorithm redistributes sorted classes maintaining the original split ratio
        // Original left: 3 words, right: 2 words -> left gets first 3, right gets rest
        let expected = r#""flex items-center m-2" + "p-4 bg-white""#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiline_jsx_class_name() {
        // Test multiline JSX with className
        let processor = FileProcessor::new();
        let input = r#"className={"p-4 flex m-2" + "items-center bg-white hover:bg-gray-100"}"#;
        // Original left: 3 words, right: 3 words -> algorithm keeps that split
        let expected = r#"className={"flex items-center m-2" + "p-4 bg-white hover:bg-gray-100"}"#;

        let result = processor
            .process_content(input, "test.tsx", ProcessOptions::default())
            .unwrap();
        assert_eq!(result, expected);
    }
}
