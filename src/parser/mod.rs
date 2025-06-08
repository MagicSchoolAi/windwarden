use oxc_allocator::Allocator;
use oxc_ast::Visit;
use oxc_parser::{Parser, ParserReturn};
use oxc_span::SourceType;
use std::path::Path;

use crate::{Result, WindWardenError};

pub use visitor::ClassExtractor;

mod visitor;

pub struct FileParser {
    allocator: Allocator,
    custom_functions: Option<Vec<String>>,
}

impl FileParser {
    pub fn new() -> Self {
        Self {
            allocator: Allocator::default(),
            custom_functions: None,
        }
    }
    
    pub fn new_with_custom_functions(custom_functions: Vec<String>) -> Self {
        Self {
            allocator: Allocator::default(),
            custom_functions: Some(custom_functions),
        }
    }

    pub fn parse_file(&self, file_path: &str, source_text: &str) -> Result<Vec<ClassMatch>> {
        let source_type = self.detect_source_type(file_path);
        self.parse_source_with_path(source_text, source_type, file_path)
    }

    pub fn parse_source(&self, source_text: &str, source_type: SourceType) -> Result<Vec<ClassMatch>> {
        self.parse_source_with_path(source_text, source_type, "unknown")
    }
    
    pub fn parse_source_with_path(&self, source_text: &str, source_type: SourceType, file_path: &str) -> Result<Vec<ClassMatch>> {
        // Wrap incomplete JSX in a component for parsing
        let (wrapped_source, offset) = self.wrap_jsx_if_needed(source_text);
        
        let ParserReturn {
            program,
            errors,
            ..
        } = Parser::new(&self.allocator, &wrapped_source, source_type).parse();

        if !errors.is_empty() {
            // For now, provide a simplified error message since extracting line numbers
            // from Oxc diagnostics is complex and would require more dependencies
            let error_count = errors.len();
            let message = if error_count == 1 {
                "Syntax error in file".to_string()
            } else {
                format!("{} syntax errors in file", error_count)
            };
            
            return Err(WindWardenError::parse_error(
                file_path, 
                1, // Default to line 1 for now
                message
            ));
        }

        let mut extractor = if let Some(ref custom_functions) = self.custom_functions {
            ClassExtractor::new_with_custom_functions(&wrapped_source, custom_functions)
        } else {
            ClassExtractor::new(&wrapped_source)
        };
        extractor.visit_program(&program);
        
        let mut matches = extractor.into_matches();
        
        // Adjust spans back to original source if we wrapped it
        if offset > 0 {
            for class_match in &mut matches {
                if class_match.start >= offset && class_match.end >= offset {
                    class_match.start -= offset;
                    class_match.end -= offset;
                }
            }
        }
        
        Ok(matches)
    }

    fn wrap_jsx_if_needed(&self, source_text: &str) -> (String, usize) {
        let trimmed = source_text.trim();
        if trimmed.starts_with('<') && !trimmed.contains("function") && !trimmed.contains("const") && !trimmed.contains("export") {
            // Make JSX element self-closing if it's not already closed
            let closed_source = if !trimmed.contains('>') {
                format!("{} />", trimmed)
            } else if trimmed.ends_with('>') && !trimmed.ends_with("/>") && !trimmed.contains("</") {
                // Convert to self-closing if it's an opening tag
                trimmed.replace('>', " />")
            } else {
                trimmed.to_string()
            };
            
            let wrapped = format!("const Component = () => ({});", closed_source);
            let offset = wrapped.find(&closed_source).unwrap_or(0);
            (wrapped, offset)
        } else {
            (source_text.to_string(), 0)
        }
    }

    fn detect_source_type(&self, file_path: &str) -> SourceType {
        let path = Path::new(file_path);
        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
        
        match extension {
            "ts" => SourceType::default().with_typescript(true),
            "tsx" => SourceType::default().with_typescript(true).with_jsx(true),
            "jsx" => SourceType::default().with_jsx(true),
            "js" => SourceType::default(),
            _ => SourceType::default().with_jsx(true), // Default to JSX for unknown types
        }
    }
}

impl Default for FileParser {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ClassMatch {
    pub start: usize,
    pub end: usize,
    pub original: String,
    pub quote_style: QuoteStyle,
    pub pattern_type: PatternType,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum QuoteStyle {
    Single,
    Double,
    Backtick,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternType {
    JSXAttribute,
    FunctionCall {
        function_name: String,
        arg_index: usize,
    },
    TemplateLiteral {
        tag: Option<String>, // None for regular `...`, Some("tw") for tw`...`
    },
    ArrayElement {
        array_index: usize,
    },
    Array {
        elements: Vec<String>, // All the string elements from the array
    },
    BinaryExpression {
        left_content: String,
        right_content: String,
    },
}

impl ClassMatch {
    pub fn new(
        start: usize,
        end: usize,
        original: String,
        quote_style: QuoteStyle,
        pattern_type: PatternType,
    ) -> Self {
        Self {
            start,
            end,
            original,
            quote_style,
            pattern_type,
        }
    }
}