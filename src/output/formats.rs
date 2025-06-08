use super::*;
use crate::parser::ClassMatch;

/// Utility functions for creating output format structures

/// Create an issue from a class match that needs sorting
pub fn create_sorting_issue(
    class_match: &ClassMatch,
    line: usize,
    column: usize,
    sorted_classes: &str,
) -> Issue {
    Issue {
        rule_id: "class-order".to_string(),
        severity: Severity::Warning,
        message: "Classes are not sorted according to Tailwind order".to_string(),
        line,
        column,
        end_line: line,
        end_column: column + class_match.original.len(),
        source: class_match.original.clone(),
        suggestions: vec![Suggestion {
            desc: "Sort classes according to Tailwind order".to_string(),
            fix: Fix {
                range: [class_match.start, class_match.end],
                text: sorted_classes.to_string(),
            },
        }],
    }
}

/// Create a modification entry for diff reports
pub fn create_modification(
    line: usize,
    column: usize,
    original_text: &str,
    new_text: &str,
) -> Modification {
    Modification {
        line,
        column,
        original_text: original_text.to_string(),
        new_text: new_text.to_string(),
        reason: "Tailwind class order correction".to_string(),
    }
}

/// Convert byte position to line and column numbers
pub fn position_to_line_col(content: &str, pos: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    
    for (i, ch) in content.char_indices() {
        if i >= pos {
            break;
        }
        
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    
    (line, col)
}

/// Extract file name from path for display
pub fn extract_file_name(path: &str) -> &str {
    path.split('/').last().unwrap_or(path)
}

/// Create an unformatted file entry for check reports
pub fn create_unformatted_file(file_path: &str, issue_count: usize) -> UnformattedFile {
    let issues = if issue_count == 1 {
        vec!["1 class ordering issue".to_string()]
    } else {
        vec![format!("{} class ordering issues", issue_count)]
    };
    
    UnformattedFile {
        file_path: file_path.to_string(),
        issues,
    }
}

/// Determine appropriate severity based on context
pub fn determine_severity(is_error: bool) -> Severity {
    if is_error {
        Severity::Error
    } else {
        Severity::Warning
    }
}

/// Format issue count for human-readable output
pub fn format_issue_count(errors: usize, warnings: usize) -> String {
    match (errors, warnings) {
        (0, 0) => "No issues found".to_string(),
        (0, w) => format!("{} warning{}", w, if w == 1 { "" } else { "s" }),
        (e, 0) => format!("{} error{}", e, if e == 1 { "" } else { "s" }),
        (e, w) => format!("{} error{}, {} warning{}", 
                         e, if e == 1 { "" } else { "s" },
                         w, if w == 1 { "" } else { "s" }),
    }
}

/// Create a progress summary for processing operations
pub fn create_processing_summary(
    files_processed: usize,
    files_changed: usize,
    classes_processed: usize,
    classes_sorted: usize,
) -> ProcessingSummary {
    ProcessingSummary {
        files_processed,
        files_changed,
        classes_processed,
        classes_sorted,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{PatternType, QuoteStyle};
    
    #[test]
    fn test_position_to_line_col() {
        let content = "line 1\nline 2\nline 3";
        
        assert_eq!(position_to_line_col(content, 0), (1, 1));
        assert_eq!(position_to_line_col(content, 6), (1, 7)); // End of line 1
        assert_eq!(position_to_line_col(content, 7), (2, 1)); // Start of line 2
        assert_eq!(position_to_line_col(content, 14), (3, 1)); // Start of line 3
    }
    
    #[test]
    fn test_extract_file_name() {
        assert_eq!(extract_file_name("/path/to/file.tsx"), "file.tsx");
        assert_eq!(extract_file_name("file.tsx"), "file.tsx");
        assert_eq!(extract_file_name("/single"), "single");
        assert_eq!(extract_file_name(""), "");
    }
    
    #[test]
    fn test_format_issue_count() {
        assert_eq!(format_issue_count(0, 0), "No issues found");
        assert_eq!(format_issue_count(1, 0), "1 error");
        assert_eq!(format_issue_count(2, 0), "2 errors");
        assert_eq!(format_issue_count(0, 1), "1 warning");
        assert_eq!(format_issue_count(0, 3), "3 warnings");
        assert_eq!(format_issue_count(1, 1), "1 error, 1 warning");
        assert_eq!(format_issue_count(2, 3), "2 errors, 3 warnings");
    }
    
    #[test]
    fn test_create_sorting_issue() {
        let class_match = ClassMatch {
            original: "p-4 bg-blue-500 flex".to_string(),
            start: 100,
            end: 120,
            pattern_type: PatternType::JSXAttribute,
            quote_style: QuoteStyle::Double,
        };
        
        let issue = create_sorting_issue(&class_match, 10, 5, "flex p-4 bg-blue-500");
        
        assert_eq!(issue.rule_id, "class-order");
        assert_eq!(issue.line, 10);
        assert_eq!(issue.column, 5);
        assert_eq!(issue.source, "p-4 bg-blue-500 flex");
        assert_eq!(issue.suggestions[0].fix.text, "flex p-4 bg-blue-500");
    }
    
    #[test]
    fn test_create_unformatted_file() {
        let file = create_unformatted_file("/test/file.tsx", 1);
        assert_eq!(file.file_path, "/test/file.tsx");
        assert_eq!(file.issues[0], "1 class ordering issue");
        
        let file = create_unformatted_file("/test/file.tsx", 5);
        assert_eq!(file.issues[0], "5 class ordering issues");
    }
}