use colored::Colorize;
use std::fmt;

/// A single change in a diff
#[derive(Debug, Clone, PartialEq)]
pub struct DiffLine {
    pub line_number: usize,
    pub change_type: ChangeType,
    pub content: String,
}

/// Type of change in a diff line
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeType {
    /// Line was removed (shown with -)
    Removed,
    /// Line was added (shown with +)
    Added,
    /// Line remained unchanged (shown with space)
    Unchanged,
}

/// A collection of changes for a single file
#[derive(Debug, Clone)]
pub struct FileDiff {
    pub file_path: String,
    pub original_content: String,
    pub modified_content: String,
    pub changes: Vec<DiffLine>,
    pub has_changes: bool,
}

impl FileDiff {
    /// Create a new file diff by comparing original and modified content
    pub fn new(file_path: String, original: String, modified: String) -> Self {
        let has_changes = original != modified;
        let changes = if has_changes {
            generate_diff_lines(&original, &modified)
        } else {
            Vec::new()
        };

        Self {
            file_path,
            original_content: original,
            modified_content: modified,
            changes,
            has_changes,
        }
    }

    /// Get a summary of the changes
    pub fn get_summary(&self) -> DiffSummary {
        if !self.has_changes {
            return DiffSummary {
                lines_added: 0,
                lines_removed: 0,
                lines_changed: 0,
            };
        }

        let mut lines_added = 0;
        let mut lines_removed = 0;

        for change in &self.changes {
            match change.change_type {
                ChangeType::Added => lines_added += 1,
                ChangeType::Removed => lines_removed += 1,
                ChangeType::Unchanged => {}
            }
        }

        DiffSummary {
            lines_added,
            lines_removed,
            lines_changed: lines_added.min(lines_removed),
        }
    }
}

/// Summary statistics for a diff
#[derive(Debug, Clone)]
pub struct DiffSummary {
    pub lines_added: usize,
    pub lines_removed: usize,
    pub lines_changed: usize,
}

/// Format a file diff for display
pub struct DiffFormatter {
    show_context: bool,
    context_lines: usize,
    use_colors: bool,
}

impl DiffFormatter {
    pub fn new() -> Self {
        Self {
            show_context: true,
            context_lines: 3,
            use_colors: true,
        }
    }

    pub fn with_context(mut self, lines: usize) -> Self {
        self.context_lines = lines;
        self
    }

    pub fn with_colors(mut self, use_colors: bool) -> Self {
        self.use_colors = use_colors;
        self
    }

    /// Format a file diff as a string
    pub fn format_diff(&self, diff: &FileDiff) -> String {
        if !diff.has_changes {
            return String::new();
        }

        let mut output = Vec::new();

        // File header
        let header = format!("--- {}", diff.file_path);
        let header2 = format!("+++ {}", diff.file_path);

        if self.use_colors {
            output.push(header.red().bold().to_string());
            output.push(header2.green().bold().to_string());
        } else {
            output.push(header);
            output.push(header2);
        }

        // Group changes into hunks
        let hunks = self.group_into_hunks(&diff.changes);

        for hunk in hunks {
            output.push(self.format_hunk_header(&hunk));

            for line in &hunk.lines {
                output.push(self.format_diff_line(line));
            }
        }

        output.join("\n")
    }

    /// Format a concise summary of changes
    pub fn format_summary(&self, diff: &FileDiff) -> String {
        if !diff.has_changes {
            return "No changes".to_string();
        }

        let summary = diff.get_summary();
        let mut parts = Vec::new();

        if summary.lines_added > 0 {
            let text = format!("+{}", summary.lines_added);
            if self.use_colors {
                parts.push(text.green().to_string());
            } else {
                parts.push(text);
            }
        }

        if summary.lines_removed > 0 {
            let text = format!("-{}", summary.lines_removed);
            if self.use_colors {
                parts.push(text.red().to_string());
            } else {
                parts.push(text);
            }
        }

        if parts.is_empty() {
            "No changes".to_string()
        } else {
            parts.join(" ")
        }
    }

    /// Format a single diff line
    fn format_diff_line(&self, line: &DiffLine) -> String {
        let prefix = match line.change_type {
            ChangeType::Added => "+",
            ChangeType::Removed => "-",
            ChangeType::Unchanged => " ",
        };

        let formatted_line = format!("{}{}", prefix, line.content);

        if self.use_colors {
            match line.change_type {
                ChangeType::Added => formatted_line.green().to_string(),
                ChangeType::Removed => formatted_line.red().to_string(),
                ChangeType::Unchanged => formatted_line.normal().to_string(),
            }
        } else {
            formatted_line
        }
    }

    /// Group diff lines into hunks
    fn group_into_hunks(&self, lines: &[DiffLine]) -> Vec<DiffHunk> {
        let mut hunks = Vec::new();
        let mut current_hunk: Option<DiffHunk> = None;

        for (i, line) in lines.iter().enumerate() {
            let is_change = matches!(line.change_type, ChangeType::Added | ChangeType::Removed);

            if is_change {
                if current_hunk.is_none() {
                    // Start new hunk with context
                    let start_context = i.saturating_sub(self.context_lines);
                    current_hunk = Some(DiffHunk {
                        start_line: lines[start_context].line_number,
                        lines: lines[start_context..i].to_vec(),
                    });
                }

                if let Some(ref mut hunk) = current_hunk {
                    hunk.lines.push(line.clone());
                }
            } else if let Some(ref mut hunk) = current_hunk {
                // Add context line to current hunk
                hunk.lines.push(line.clone());

                // Check if we should end the hunk
                let context_after = lines[i..]
                    .iter()
                    .take(self.context_lines + 1)
                    .all(|l| matches!(l.change_type, ChangeType::Unchanged));

                if context_after {
                    // Add trailing context and finish hunk
                    let end_context = (i + self.context_lines + 1).min(lines.len());
                    hunk.lines.extend_from_slice(&lines[i + 1..end_context]);
                    hunks.push(current_hunk.take().unwrap());
                }
            }
        }

        // Finish any remaining hunk
        if let Some(hunk) = current_hunk {
            hunks.push(hunk);
        }

        hunks
    }

    /// Format a hunk header
    fn format_hunk_header(&self, hunk: &DiffHunk) -> String {
        let start = hunk.start_line;
        let count = hunk.lines.len();
        let header = format!("@@ -{},{} +{},{} @@", start, count, start, count);

        if self.use_colors {
            header.cyan().bold().to_string()
        } else {
            header
        }
    }
}

impl Default for DiffFormatter {
    fn default() -> Self {
        Self::new()
    }
}

/// A group of related diff lines
#[derive(Debug, Clone)]
struct DiffHunk {
    start_line: usize,
    lines: Vec<DiffLine>,
}

/// Generate diff lines by comparing two strings line by line
fn generate_diff_lines(original: &str, modified: &str) -> Vec<DiffLine> {
    let original_lines: Vec<&str> = original.lines().collect();
    let modified_lines: Vec<&str> = modified.lines().collect();

    // Use a simple line-by-line diff algorithm
    // This is basic but effective for most code formatting changes
    simple_diff(&original_lines, &modified_lines)
}

/// Simple diff algorithm - compares line by line
fn simple_diff(original: &[&str], modified: &[&str]) -> Vec<DiffLine> {
    let mut result = Vec::new();
    let mut orig_idx = 0;
    let mut mod_idx = 0;
    let mut line_num = 1;

    while orig_idx < original.len() || mod_idx < modified.len() {
        if orig_idx < original.len() && mod_idx < modified.len() {
            let orig_line = original[orig_idx];
            let mod_line = modified[mod_idx];

            if orig_line == mod_line {
                // Lines are identical
                result.push(DiffLine {
                    line_number: line_num,
                    change_type: ChangeType::Unchanged,
                    content: orig_line.to_string(),
                });
                orig_idx += 1;
                mod_idx += 1;
                line_num += 1;
            } else {
                // Lines differ - look ahead to see if we can find a match
                let orig_match = modified[mod_idx..].iter().position(|&l| l == orig_line);
                let mod_match = original[orig_idx..].iter().position(|&l| l == mod_line);

                match (orig_match, mod_match) {
                    (Some(0), _) => {
                        // Current original line matches next modified line - modified line was added
                        result.push(DiffLine {
                            line_number: line_num,
                            change_type: ChangeType::Added,
                            content: mod_line.to_string(),
                        });
                        mod_idx += 1;
                        line_num += 1;
                    }
                    (_, Some(0)) => {
                        // Current modified line matches next original line - original line was removed
                        result.push(DiffLine {
                            line_number: line_num,
                            change_type: ChangeType::Removed,
                            content: orig_line.to_string(),
                        });
                        orig_idx += 1;
                        line_num += 1;
                    }
                    _ => {
                        // Lines are different - treat as remove + add
                        result.push(DiffLine {
                            line_number: line_num,
                            change_type: ChangeType::Removed,
                            content: orig_line.to_string(),
                        });
                        result.push(DiffLine {
                            line_number: line_num,
                            change_type: ChangeType::Added,
                            content: mod_line.to_string(),
                        });
                        orig_idx += 1;
                        mod_idx += 1;
                        line_num += 1;
                    }
                }
            }
        } else if orig_idx < original.len() {
            // Remaining original lines were removed
            result.push(DiffLine {
                line_number: line_num,
                change_type: ChangeType::Removed,
                content: original[orig_idx].to_string(),
            });
            orig_idx += 1;
            line_num += 1;
        } else {
            // Remaining modified lines were added
            result.push(DiffLine {
                line_number: line_num,
                change_type: ChangeType::Added,
                content: modified[mod_idx].to_string(),
            });
            mod_idx += 1;
            line_num += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_changes() {
        let original = "const x = 1;\nconst y = 2;";
        let modified = "const x = 1;\nconst y = 2;";

        let diff = FileDiff::new(
            "test.js".to_string(),
            original.to_string(),
            modified.to_string(),
        );
        assert!(!diff.has_changes);
        assert!(diff.changes.is_empty());
    }

    #[test]
    fn test_simple_change() {
        let original = r#"<div className="p-4 flex m-2">Test</div>"#;
        let modified = r#"<div className="flex m-2 p-4">Test</div>"#;

        let diff = FileDiff::new(
            "test.jsx".to_string(),
            original.to_string(),
            modified.to_string(),
        );
        assert!(diff.has_changes);

        let formatter = DiffFormatter::new().with_colors(false);
        let output = formatter.format_diff(&diff);

        assert!(output.contains("--- test.jsx"));
        assert!(output.contains("+++ test.jsx"));
        assert!(output.contains(r#"-<div className="p-4 flex m-2">Test</div>"#));
        assert!(output.contains(r#"+<div className="flex m-2 p-4">Test</div>"#));
    }

    #[test]
    fn test_diff_summary() {
        let original = "line1\nline2\nline3";
        let modified = "line1\nmodified2\nline3\nnew_line";

        let diff = FileDiff::new(
            "test.txt".to_string(),
            original.to_string(),
            modified.to_string(),
        );
        let summary = diff.get_summary();

        assert_eq!(summary.lines_added, 2); // modified2 + new_line
        assert_eq!(summary.lines_removed, 1); // line2
    }

    #[test]
    fn test_format_summary() {
        let original = "old line";
        let modified = "new line";

        let diff = FileDiff::new(
            "test.txt".to_string(),
            original.to_string(),
            modified.to_string(),
        );
        let formatter = DiffFormatter::new().with_colors(false);
        let summary = formatter.format_summary(&diff);

        assert_eq!(summary, "+1 -1");
    }
}
