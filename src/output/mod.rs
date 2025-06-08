use serde::{Deserialize, Serialize};
use std::path::Path;

pub mod formats;
pub mod reporters;
pub mod text_formatter;

// Re-export the text formatter for backward compatibility
pub use text_formatter::{OutputFormatter, ProgressReporter, ProgressTracker};

/// Output format for machine-readable results
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputFormat {
    /// Human-readable text output (default)
    Text,
    /// ESLint-style JSON diagnostic format
    Json,
    /// Simple check format (like Prettier)
    Check,
    /// Diff format showing before/after changes
    Diff,
    /// Summary statistics only
    Summary,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            "check" => Ok(OutputFormat::Check),
            "diff" => Ok(OutputFormat::Diff),
            "summary" => Ok(OutputFormat::Summary),
            _ => Err(format!("Invalid output format '{}'. Valid options: text, json, check, diff, summary", s)),
        }
    }
}

/// Represents a single issue found during processing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    /// Rule identifier (e.g., "class-order")
    pub rule_id: String,
    /// Severity level
    pub severity: Severity,
    /// Human-readable message
    pub message: String,
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)  
    pub column: usize,
    /// End line number (1-based)
    pub end_line: usize,
    /// End column number (1-based)
    pub end_column: usize,
    /// Original source text
    pub source: String,
    /// Suggested fixes
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub suggestions: Vec<Suggestion>,
}

/// Severity levels for issues
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// Suggested fix for an issue
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Suggestion {
    /// Description of the fix
    pub desc: String,
    /// The actual fix to apply
    pub fix: Fix,
}

/// Represents a code fix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fix {
    /// Character range to replace [start, end)
    pub range: [usize; 2],
    /// Replacement text
    pub text: String,
}

/// Result for a single file
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileResult {
    /// Absolute file path
    pub file_path: String,
    /// Issues found in this file
    pub messages: Vec<Issue>,
    /// Number of errors
    pub error_count: usize,
    /// Number of warnings
    pub warning_count: usize,
    /// Number of fixable errors
    pub fixable_error_count: usize,
    /// Number of fixable warnings
    pub fixable_warning_count: usize,
}

/// Complete diagnostic report (ESLint-style)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticReport {
    /// Format version
    pub version: String,
    /// Tool name
    pub tool: String,
    /// Results per file
    pub results: Vec<FileResult>,
    /// Total error count across all files
    pub error_count: usize,
    /// Total warning count across all files
    pub warning_count: usize,
    /// Total fixable error count
    pub fixable_error_count: usize,
    /// Total fixable warning count
    pub fixable_warning_count: usize,
}

/// Simple check result (Prettier-style)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckReport {
    /// Format version
    pub version: String,
    /// Tool name
    pub tool: String,
    /// Summary statistics
    pub summary: CheckSummary,
    /// Files that need formatting
    pub unformatted_files: Vec<UnformattedFile>,
}

/// Summary for check mode
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckSummary {
    /// Total files checked
    pub total_files: usize,
    /// Files already properly formatted
    pub formatted_files: usize,
    /// Files that need formatting
    pub unformatted_files: usize,
}

/// File that needs formatting
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnformattedFile {
    /// File path
    pub file_path: String,
    /// Brief issues description
    pub issues: Vec<String>,
}

/// Modification made to a file
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Modification {
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
    /// Original text
    pub original_text: String,
    /// New text
    pub new_text: String,
    /// Reason for the change
    pub reason: String,
}

/// Changes made to a file
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileChanges {
    /// File path
    pub file_path: String,
    /// List of modifications
    pub modifications: Vec<Modification>,
}

/// Diff report showing changes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffReport {
    /// Format version
    pub version: String,
    /// Tool name
    pub tool: String,
    /// Processing summary
    pub summary: ProcessingSummary,
    /// Changes made
    pub changes: Vec<FileChanges>,
}

/// Processing statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessingSummary {
    /// Total files processed
    pub files_processed: usize,
    /// Files that were changed
    pub files_changed: usize,
    /// Total classes found and processed
    pub classes_processed: usize,
    /// Classes that were reordered
    pub classes_sorted: usize,
}

/// Summary-only report
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummaryReport {
    /// Format version
    pub version: String,
    /// Tool name
    pub tool: String,
    /// Processing summary
    pub summary: ProcessingSummary,
}

impl FileResult {
    pub fn new(file_path: impl AsRef<Path>) -> Self {
        Self {
            file_path: file_path.as_ref().display().to_string(),
            messages: Vec::new(),
            error_count: 0,
            warning_count: 0,
            fixable_error_count: 0,
            fixable_warning_count: 0,
        }
    }
    
    pub fn add_issue(&mut self, issue: Issue) {
        match issue.severity {
            Severity::Error => {
                self.error_count += 1;
                if !issue.suggestions.is_empty() {
                    self.fixable_error_count += 1;
                }
            }
            Severity::Warning => {
                self.warning_count += 1;
                if !issue.suggestions.is_empty() {
                    self.fixable_warning_count += 1;
                }
            }
            Severity::Info => {
                // Info messages don't count toward error/warning counts
            }
        }
        self.messages.push(issue);
    }
}

impl DiagnosticReport {
    pub fn new() -> Self {
        Self {
            version: "1.0.0".to_string(),
            tool: "windwarden".to_string(),
            results: Vec::new(),
            error_count: 0,
            warning_count: 0,
            fixable_error_count: 0,
            fixable_warning_count: 0,
        }
    }
    
    pub fn add_file_result(&mut self, file_result: FileResult) {
        self.error_count += file_result.error_count;
        self.warning_count += file_result.warning_count;
        self.fixable_error_count += file_result.fixable_error_count;
        self.fixable_warning_count += file_result.fixable_warning_count;
        self.results.push(file_result);
    }
}

impl CheckReport {
    pub fn new() -> Self {
        Self {
            version: "1.0.0".to_string(),
            tool: "windwarden".to_string(),
            summary: CheckSummary {
                total_files: 0,
                formatted_files: 0,
                unformatted_files: 0,
            },
            unformatted_files: Vec::new(),
        }
    }
}

impl DiffReport {
    pub fn new() -> Self {
        Self {
            version: "1.0.0".to_string(),
            tool: "windwarden".to_string(),
            summary: ProcessingSummary {
                files_processed: 0,
                files_changed: 0,
                classes_processed: 0,
                classes_sorted: 0,
            },
            changes: Vec::new(),
        }
    }
}

impl SummaryReport {
    pub fn new() -> Self {
        Self {
            version: "1.0.0".to_string(),
            tool: "windwarden".to_string(),
            summary: ProcessingSummary {
                files_processed: 0,
                files_changed: 0,
                classes_processed: 0,
                classes_sorted: 0,
            },
        }
    }
}