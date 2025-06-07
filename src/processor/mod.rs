use std::fs;

use crate::parser::{FileParser, QuoteStyle};
use crate::sorter::TailwindSorter;
use crate::{ProcessOptions, Result, WindWardenError};

pub struct FileProcessor {
    parser: FileParser,
    sorter: TailwindSorter,
}

impl FileProcessor {
    pub fn new() -> Self {
        Self {
            parser: FileParser::new(),
            sorter: TailwindSorter::new(),
        }
    }

    pub fn process_file(&self, file_path: &str, options: ProcessOptions) -> Result<String> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| WindWardenError::Io(e))?;
        
        self.process_content(&content, file_path, options)
    }

    pub fn process_content(&self, content: &str, file_path: &str, options: ProcessOptions) -> Result<String> {
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
                
                // Find the className attribute in the original content using string search
                // This is more reliable than span positions for our current use case
                let search_pattern = format!("{}{}{}", quote_char, class_match.original, quote_char);
                if let Some(start_pos) = result.find(&search_pattern) {
                    let end_pos = start_pos + search_pattern.len();
                    result.replace_range(start_pos..end_pos, &replacement);
                }
            }
        }

        // Handle different processing modes
        if options.check_formatted {
            if changes_made {
                return Err(WindWardenError::Sort("Classes are not sorted".to_string()));
            } else {
                return Ok(String::new()); // Success case - no output needed
            }
        }

        if options.write && changes_made {
            fs::write(file_path, &result)
                .map_err(|e| WindWardenError::Io(e))?;
            Ok(String::new()) // No output needed when writing to file
        } else if options.dry_run || !options.write {
            Ok(result)
        } else {
            Ok(String::new()) // No changes and not in preview mode
        }
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
        
        let result = processor.process_content(input, "test.tsx", ProcessOptions::default()).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_no_changes_needed() {
        let processor = FileProcessor::new();
        let input = r#"<div className="flex m-2 p-4">"#;
        
        let result = processor.process_content(input, "test.tsx", ProcessOptions::default()).unwrap();
        assert_eq!(result, input);
    }

    #[test]
    fn test_preserve_quote_style() {
        let processor = FileProcessor::new();
        let input = r#"<div className='p-4 flex m-2'>"#;
        let expected = r#"<div className='flex m-2 p-4'>"#;
        
        let result = processor.process_content(input, "test.tsx", ProcessOptions::default()).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_attributes() {
        let processor = FileProcessor::new();
        let input = r#"<div className="p-4 flex" id="test"><span className="m-2 text-sm">Text</span></div>"#;
        let expected = r#"<div className="flex p-4" id="test"><span className="m-2 text-sm">Text</span></div>"#;
        
        let result = processor.process_content(input, "test.tsx", ProcessOptions::default()).unwrap();
        assert_eq!(result, expected);
    }
}