use oxc_ast::ast::*;
use oxc_ast::{Visit, VisitMut};
use oxc_span::Span;

use super::{ClassMatch, PatternType, QuoteStyle};

pub struct ClassExtractor<'a> {
    source_text: &'a str,
    matches: Vec<ClassMatch>,
}

impl<'a> ClassExtractor<'a> {
    pub fn new(source_text: &'a str) -> Self {
        Self {
            source_text,
            matches: Vec::new(),
        }
    }

    pub fn into_matches(self) -> Vec<ClassMatch> {
        self.matches
    }

    fn extract_string_value(&self, span: Span) -> String {
        let start = span.start as usize;
        let end = span.end as usize;
        
        if start >= self.source_text.len() || end > self.source_text.len() || start >= end {
            return String::new();
        }
        
        self.source_text[start..end].to_string()
    }

    fn detect_quote_style(&self, span: Span) -> QuoteStyle {
        let full_text = self.extract_string_value(span);
        
        if full_text.starts_with('\'') {
            QuoteStyle::Single
        } else if full_text.starts_with('"') {
            QuoteStyle::Double
        } else if full_text.starts_with('`') {
            QuoteStyle::Backtick
        } else {
            QuoteStyle::Double // Default fallback
        }
    }

    fn extract_class_string_content(&self, span: Span) -> String {
        let full_text = self.extract_string_value(span);
        
        // Remove surrounding quotes
        if full_text.len() >= 2 {
            let first_char = full_text.chars().next().unwrap();
            let last_char = full_text.chars().last().unwrap();
            
            if (first_char == '"' && last_char == '"') 
                || (first_char == '\'' && last_char == '\'')
                || (first_char == '`' && last_char == '`') {
                return full_text[1..full_text.len()-1].to_string();
            }
        }
        
        full_text
    }

    fn is_class_attribute(&self, attr_name: &str) -> bool {
        matches!(attr_name, "className" | "class")
    }
}

impl<'a> Visit<'a> for ClassExtractor<'a> {
    fn visit_jsx_attribute(&mut self, attr: &JSXAttribute<'a>) {
        if let JSXAttributeName::Identifier(ident) = &attr.name {
            if self.is_class_attribute(&ident.name) {
                if let Some(JSXAttributeValue::StringLiteral(string_lit)) = &attr.value {
                    let quote_style = self.detect_quote_style(string_lit.span);
                    let class_content = self.extract_class_string_content(string_lit.span);
                    
                    
                    // Only process if there are actually classes to sort
                    if !class_content.trim().is_empty() {
                        let class_match = ClassMatch::new(
                            string_lit.span.start as usize,
                            string_lit.span.end as usize,
                            class_content,
                            quote_style,
                            PatternType::JSXAttribute,
                        );
                        self.matches.push(class_match);
                    }
                }
            }
        }
        
        // Continue visiting child nodes
        self.visit_jsx_attribute_name(&attr.name);
        if let Some(value) = &attr.value {
            self.visit_jsx_attribute_value(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::FileParser;
    use oxc_span::SourceType;

    fn parse_and_extract(source: &str) -> Vec<ClassMatch> {
        let parser = FileParser::new();
        let source_type = SourceType::default().with_jsx(true).with_typescript(true);
        parser.parse_source(source, source_type).unwrap_or_default()
    }

    #[test]
    fn test_basic_jsx_classname() {
        let source = r#"<div className="p-4 flex m-2">"#;
        let matches = parse_and_extract(source);
        
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].original, "p-4 flex m-2");
        assert_eq!(matches[0].quote_style, QuoteStyle::Double);
        assert_eq!(matches[0].pattern_type, PatternType::JSXAttribute);
    }

    #[test]
    fn test_jsx_class_attribute() {
        let source = r#"<div class="p-4 flex m-2">"#;
        let matches = parse_and_extract(source);
        
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].original, "p-4 flex m-2");
    }

    #[test]
    fn test_single_quotes() {
        let source = r#"<div className='p-4 flex m-2'>"#;
        let matches = parse_and_extract(source);
        
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].original, "p-4 flex m-2");
        assert_eq!(matches[0].quote_style, QuoteStyle::Single);
    }

    #[test]
    fn test_empty_classname() {
        let source = r#"<div className="">"#;
        let matches = parse_and_extract(source);
        
        // Empty class strings should not be matched
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_no_class_attributes() {
        let source = r#"<div id="test" data-value="something">"#;
        let matches = parse_and_extract(source);
        
        assert_eq!(matches.len(), 0);
    }
}