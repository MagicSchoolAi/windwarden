use oxc_ast::ast::*;
use oxc_ast::Visit;
use oxc_span::Span;

use super::{ClassMatch, PatternType, QuoteStyle};

const DEFAULT_SUPPORTED_FUNCTIONS: &[&str] =
    &["cn", "twMerge", "clsx", "classNames", "classList", "cva"];

pub struct ClassExtractor<'a> {
    source_text: &'a str,
    matches: Vec<ClassMatch>,
    processed_spans: std::collections::HashSet<(usize, usize)>,
    supported_functions: std::collections::HashSet<String>,
}

impl<'a> ClassExtractor<'a> {
    pub fn new(source_text: &'a str) -> Self {
        let mut supported_functions = std::collections::HashSet::new();
        for func in DEFAULT_SUPPORTED_FUNCTIONS {
            supported_functions.insert(func.to_string());
        }

        Self {
            source_text,
            matches: Vec::new(),
            processed_spans: std::collections::HashSet::new(),
            supported_functions,
        }
    }

    pub fn new_with_custom_functions(source_text: &'a str, custom_functions: &[String]) -> Self {
        let mut supported_functions = std::collections::HashSet::new();

        // Add default functions
        for func in DEFAULT_SUPPORTED_FUNCTIONS {
            supported_functions.insert(func.to_string());
        }

        // Add custom functions
        for func in custom_functions {
            supported_functions.insert(func.clone());
        }

        Self {
            source_text,
            matches: Vec::new(),
            processed_spans: std::collections::HashSet::new(),
            supported_functions,
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
                || (first_char == '`' && last_char == '`')
            {
                return full_text[1..full_text.len() - 1].to_string();
            }
        }

        full_text
    }

    fn is_class_attribute(&self, attr_name: &str) -> bool {
        matches!(attr_name, "className" | "class")
    }

    fn is_supported_function(&self, function_name: &str) -> bool {
        self.supported_functions.contains(function_name)
    }

    fn extract_function_name(&self, call_expr: &CallExpression) -> Option<String> {
        match &call_expr.callee {
            Expression::Identifier(ident) => Some(ident.name.to_string()),
            // TODO: Handle member expressions like `utils.cn()` in future phases
            _ => None,
        }
    }

    fn looks_like_tailwind_classes(&self, content: &str) -> bool {
        // Simple heuristic: if it contains spaces and looks like classes
        let trimmed = content.trim();

        // Must have spaces (multiple classes) or known Tailwind patterns
        if trimmed.contains(' ') {
            return true;
        }

        // Single class - check if it matches common Tailwind patterns
        let common_prefixes = [
            "p-", "m-", "text-", "bg-", "flex", "grid", "w-", "h-", "border-", "rounded", "items-",
            "justify-", "gap-",
        ];
        common_prefixes
            .iter()
            .any(|prefix| trimmed.starts_with(prefix))
    }

    fn is_static_template_literal(&self, template: &TemplateLiteral) -> bool {
        // Static template literals have only one quasi (text) and no expressions
        template.expressions.is_empty() && template.quasis.len() == 1
    }

    fn extract_template_content(&self, template: &TemplateLiteral) -> Option<String> {
        // Only process static templates (no interpolations)
        if !self.is_static_template_literal(template) {
            return None;
        }

        // Get the single text segment
        if let Some(quasi) = template.quasis.first() {
            Some(quasi.value.cooked.as_ref()?.to_string())
        } else {
            None
        }
    }

    fn process_function_arguments(&mut self, function_name: &str, arguments: &[Argument<'a>]) {
        for (arg_index, arg) in arguments.iter().enumerate() {
            match arg {
                Argument::StringLiteral(string_lit) => {
                    // Direct string literal argument
                    self.process_string_literal(
                        string_lit,
                        PatternType::FunctionCall {
                            function_name: function_name.to_string(),
                            arg_index,
                        },
                    );
                }
                _ => {
                    // For non-string arguments (conditionals, objects, etc.),
                    // we still need to visit them to find nested string literals
                    self.visit_argument(arg);
                }
            }
        }
    }

    fn process_string_literal(
        &mut self,
        string_lit: &StringLiteral<'a>,
        pattern_type: PatternType,
    ) {
        let span_key = (string_lit.span.start as usize, string_lit.span.end as usize);

        // Skip if we've already processed this span
        if self.processed_spans.contains(&span_key) {
            return;
        }

        let quote_style = self.detect_quote_style(string_lit.span);
        let class_content = self.extract_class_string_content(string_lit.span);

        // Only process if there are actually classes to sort
        if !class_content.trim().is_empty() && self.looks_like_tailwind_classes(&class_content) {
            self.processed_spans.insert(span_key);

            let class_match = ClassMatch::new(
                string_lit.span.start as usize,
                string_lit.span.end as usize,
                class_content,
                quote_style,
                pattern_type,
            );
            self.matches.push(class_match);
        }
    }
}

impl<'a> Visit<'a> for ClassExtractor<'a> {
    fn visit_jsx_attribute(&mut self, attr: &JSXAttribute<'a>) {
        if let JSXAttributeName::Identifier(ident) = &attr.name {
            if self.is_class_attribute(&ident.name) {
                if let Some(JSXAttributeValue::StringLiteral(string_lit)) = &attr.value {
                    self.process_string_literal(string_lit, PatternType::JSXAttribute);
                }
            }
        }

        // Continue visiting child nodes
        self.visit_jsx_attribute_name(&attr.name);
        if let Some(value) = &attr.value {
            self.visit_jsx_attribute_value(value);
        }
    }

    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        // Check if this is a supported function call
        if let Some(function_name) = self.extract_function_name(call) {
            if self.is_supported_function(&function_name) {
                // For supported function calls, we DON'T continue normal visiting
                // Instead, we manually process arguments to avoid duplicates
                self.process_function_arguments(&function_name, &call.arguments);
                return;
            } else {
                // For unsupported function calls, don't visit arguments at all
                // This prevents string literals inside them from being processed
                self.visit_expression(&call.callee);
                return;
            }
        }

        // For non-function expressions (member expressions, etc.), continue normal visiting
        self.visit_expression(&call.callee);
        for arg in &call.arguments {
            self.visit_argument(arg);
        }
    }

    fn visit_string_literal(&mut self, lit: &StringLiteral<'a>) {
        // This catches string literals that are NOT in supported function calls
        // (e.g., standalone strings, strings in conditionals within non-supported functions)
        self.process_string_literal(
            lit,
            PatternType::FunctionCall {
                function_name: "general".to_string(),
                arg_index: 0,
            },
        );
    }

    fn visit_template_literal(&mut self, template: &TemplateLiteral<'a>) {
        // Only process static template literals (no interpolations)
        if let Some(content) = self.extract_template_content(template) {
            if !content.trim().is_empty() && self.looks_like_tailwind_classes(&content) {
                let span_key = (template.span.start as usize, template.span.end as usize);

                // Skip if already processed
                if !self.processed_spans.contains(&span_key) {
                    self.processed_spans.insert(span_key);

                    let class_match = ClassMatch::new(
                        template.span.start as usize,
                        template.span.end as usize,
                        content,
                        QuoteStyle::Backtick, // Template literals always use backticks
                        PatternType::TemplateLiteral { tag: None },
                    );
                    self.matches.push(class_match);
                }
            }
        }

        // Continue visiting child nodes for dynamic templates
        for expr in &template.expressions {
            self.visit_expression(expr);
        }
    }

    fn visit_tagged_template_expression(&mut self, tagged: &TaggedTemplateExpression<'a>) {
        // Extract tag name if it's a simple identifier
        let tag_name = match &tagged.tag {
            Expression::Identifier(ident) => Some(ident.name.to_string()),
            _ => None,
        };

        // Process the template part
        if let Some(content) = self.extract_template_content(&tagged.quasi) {
            if !content.trim().is_empty() && self.looks_like_tailwind_classes(&content) {
                let span_key = (
                    tagged.quasi.span.start as usize,
                    tagged.quasi.span.end as usize,
                );

                // Skip if already processed
                if !self.processed_spans.contains(&span_key) {
                    self.processed_spans.insert(span_key);

                    let class_match = ClassMatch::new(
                        tagged.quasi.span.start as usize,
                        tagged.quasi.span.end as usize,
                        content,
                        QuoteStyle::Backtick,
                        PatternType::TemplateLiteral { tag: tag_name },
                    );
                    self.matches.push(class_match);
                }
            }
        }

        // Continue visiting
        self.visit_expression(&tagged.tag);
        self.visit_template_literal(&tagged.quasi);
    }

    fn visit_array_expression(&mut self, array: &ArrayExpression<'a>) {
        // Collect all string literals from the array
        let mut string_elements = Vec::new();
        let mut all_strings = true;
        let mut quote_style = QuoteStyle::Double; // Default

        for element in &array.elements {
            match element {
                ArrayExpressionElement::StringLiteral(string_lit) => {
                    let content = self.extract_class_string_content(string_lit.span);
                    if self.looks_like_tailwind_classes(&content) {
                        string_elements.push(content);
                        // Use the quote style from the first element for consistency
                        if string_elements.len() == 1 {
                            quote_style = self.detect_quote_style(string_lit.span);
                        }
                    } else {
                        all_strings = false;
                        break;
                    }
                }
                _ => {
                    all_strings = false;
                    break;
                }
            }
        }

        // If all elements are Tailwind class strings, treat as a sortable array
        if all_strings && !string_elements.is_empty() {
            let span_key = (array.span.start as usize, array.span.end as usize);

            // Skip if already processed
            if !self.processed_spans.contains(&span_key) {
                self.processed_spans.insert(span_key);

                // Join all elements for sorting (like a single class string)
                let combined_classes = string_elements.join(" ");

                let class_match = ClassMatch::new(
                    array.span.start as usize,
                    array.span.end as usize,
                    combined_classes,
                    quote_style,
                    PatternType::Array {
                        elements: string_elements,
                    },
                );
                self.matches.push(class_match);
            }
        } else {
            // For mixed arrays or non-Tailwind arrays, continue normal visiting
            for element in &array.elements {
                if let Some(expr) = element.as_expression() {
                    self.visit_expression(expr);
                }
            }
        }
    }

    fn visit_object_property(&mut self, prop: &ObjectProperty<'a>) {
        // Check if this is a className or class property
        if let PropertyKey::StaticIdentifier(ident) = &prop.key {
            if self.is_class_attribute(&ident.name) {
                // Process the value if it's a string literal
                if let Expression::StringLiteral(string_lit) = &prop.value {
                    let span_key = (string_lit.span.start as usize, string_lit.span.end as usize);

                    // Skip if already processed
                    if !self.processed_spans.contains(&span_key) {
                        self.processed_spans.insert(span_key);

                        let content = self.extract_class_string_content(string_lit.span);
                        if self.looks_like_tailwind_classes(&content) {
                            let quote_style = self.detect_quote_style(string_lit.span);

                            let class_match = ClassMatch::new(
                                string_lit.span.start as usize,
                                string_lit.span.end as usize,
                                content,
                                quote_style,
                                PatternType::JSXAttribute, // Treat similar to JSX attribute
                            );
                            self.matches.push(class_match);
                        }
                    }
                }
            }
        }

        // Continue visiting other parts of the object property
        self.visit_property_key(&prop.key);
        self.visit_expression(&prop.value);
    }

    fn visit_binary_expression(&mut self, expr: &BinaryExpression<'a>) {
        // Handle string concatenation (e.g., "classes " + "more classes")
        if matches!(expr.operator, BinaryOperator::Addition) {
            if let (Expression::StringLiteral(left), Expression::StringLiteral(right)) =
                (&expr.left, &expr.right)
            {
                // Check if both parts look like Tailwind classes
                let left_content = self.extract_class_string_content(left.span);
                let right_content = self.extract_class_string_content(right.span);

                if self.looks_like_tailwind_classes(&left_content)
                    && self.looks_like_tailwind_classes(&right_content)
                {
                    // Create a combined span covering both strings and the operator
                    let span_key = (expr.span.start as usize, expr.span.end as usize);

                    // Skip if already processed
                    if !self.processed_spans.contains(&span_key) {
                        self.processed_spans.insert(span_key);

                        // Combine the class strings
                        let combined_classes =
                            format!("{} {}", left_content.trim(), right_content.trim());

                        if self.looks_like_tailwind_classes(&combined_classes) {
                            let quote_style = self.detect_quote_style(left.span);

                            let class_match = ClassMatch::new(
                                expr.span.start as usize,
                                expr.span.end as usize,
                                combined_classes,
                                quote_style,
                                PatternType::BinaryExpression {
                                    left_content: left_content.clone(),
                                    right_content: right_content.clone(),
                                },
                            );
                            self.matches.push(class_match);
                        }
                    }
                    return; // Don't visit children if we processed this concatenation
                }
            }
        }

        // Continue normal visiting for non-string concatenations
        self.visit_expression(&expr.left);
        self.visit_expression(&expr.right);
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
    fn test_object_property_classname() {
        let source = r#"const props = { className: "p-4 flex m-2 items-center" }"#;
        let matches = parse_and_extract(source);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].original, "p-4 flex m-2 items-center");
        assert_eq!(matches[0].quote_style, QuoteStyle::Double);
        assert_eq!(matches[0].pattern_type, PatternType::JSXAttribute);
    }

    #[test]
    fn test_object_property_class() {
        let source = r#"const props = { class: "p-4 flex m-2 items-center" }"#;
        let matches = parse_and_extract(source);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].original, "p-4 flex m-2 items-center");
    }

    #[test]
    fn test_string_concatenation() {
        let source = r#""p-4 flex m-2" + "items-center bg-white""#;
        let matches = parse_and_extract(source);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].original, "p-4 flex m-2 items-center bg-white");
        assert_eq!(matches[0].quote_style, QuoteStyle::Double);
    }

    #[test]
    fn test_multiline_string_jsx() {
        let source = r#"className={"p-4 flex m-2" + "items-center bg-white"}"#;
        let matches = parse_and_extract(source);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].original, "p-4 flex m-2 items-center bg-white");
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

    #[test]
    fn test_basic_cn_function() {
        let source = r#"cn("p-4 flex m-2")"#;
        let matches = parse_and_extract(source);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].original, "p-4 flex m-2");
        assert_eq!(matches[0].quote_style, QuoteStyle::Double);

        if let PatternType::FunctionCall {
            function_name,
            arg_index,
        } = &matches[0].pattern_type
        {
            assert_eq!(function_name, "cn");
            assert_eq!(*arg_index, 0);
        } else {
            panic!("Expected FunctionCall pattern type");
        }
    }

    #[test]
    fn test_multiple_cn_args() {
        let source = r#"cn("p-4 flex", "m-2 items-center")"#;
        let matches = parse_and_extract(source);

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].original, "p-4 flex");
        assert_eq!(matches[1].original, "m-2 items-center");

        // Check function names and arg indices
        if let PatternType::FunctionCall {
            function_name,
            arg_index,
        } = &matches[0].pattern_type
        {
            assert_eq!(function_name, "cn");
            assert_eq!(*arg_index, 0);
        }

        if let PatternType::FunctionCall {
            function_name,
            arg_index,
        } = &matches[1].pattern_type
        {
            assert_eq!(function_name, "cn");
            assert_eq!(*arg_index, 1);
        }
    }

    #[test]
    fn test_static_template_literal() {
        let source = r#"const x = `p-4 flex m-2`"#;
        let matches = parse_and_extract(source);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].original, "p-4 flex m-2");
        assert_eq!(matches[0].quote_style, QuoteStyle::Backtick);

        if let PatternType::TemplateLiteral { tag } = &matches[0].pattern_type {
            assert_eq!(tag, &None);
        } else {
            panic!("Expected TemplateLiteral pattern type");
        }
    }

    #[test]
    fn test_tagged_template_literal() {
        let source = r#"const styles = tw`p-4 flex m-2`"#;
        let matches = parse_and_extract(source);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].original, "p-4 flex m-2");
        assert_eq!(matches[0].quote_style, QuoteStyle::Backtick);

        if let PatternType::TemplateLiteral { tag } = &matches[0].pattern_type {
            assert_eq!(tag, &Some("tw".to_string()));
        } else {
            panic!("Expected TemplateLiteral pattern type");
        }
    }

    #[test]
    fn test_basic_array() {
        let source = r#"const arr = ["p-4", "flex", "m-2", "items-center"]"#;
        let matches = parse_and_extract(source);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].original, "p-4 flex m-2 items-center");

        // Check that it's detected as an Array pattern
        if let PatternType::Array { elements } = &matches[0].pattern_type {
            assert_eq!(elements.len(), 4);
            assert_eq!(elements[0], "p-4");
            assert_eq!(elements[1], "flex");
            assert_eq!(elements[2], "m-2");
            assert_eq!(elements[3], "items-center");
        } else {
            panic!("Expected Array pattern type");
        }
    }

    #[test]
    fn test_cva_function() {
        let source = r#"cva(['p-4', 'flex'], { variants: {} })"#;
        let matches = parse_and_extract(source);

        // Should find the array as a single unit
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].original, "p-4 flex");

        // Check that it's detected as an Array pattern
        if let PatternType::Array { elements } = &matches[0].pattern_type {
            assert_eq!(elements.len(), 2);
            assert_eq!(elements[0], "p-4");
            assert_eq!(elements[1], "flex");
        } else {
            panic!("Expected Array pattern type");
        }
    }
}
