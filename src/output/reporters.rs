use super::*;
use crate::{Result, WindWardenError};
use std::io::Write;

/// Main reporter that can output in multiple formats
pub struct Reporter {
    format: OutputFormat,
    writer: Box<dyn Write>,
}

impl Reporter {
    /// Create a new reporter with the specified format and writer
    pub fn new(format: OutputFormat, writer: Box<dyn Write>) -> Self {
        Self { format, writer }
    }

    /// Create a reporter that writes to stdout
    pub fn stdout(format: OutputFormat) -> Self {
        Self::new(format, Box::new(std::io::stdout()))
    }

    /// Create a reporter that writes to stderr
    pub fn stderr(format: OutputFormat) -> Self {
        Self::new(format, Box::new(std::io::stderr()))
    }

    /// Report diagnostic results
    pub fn report_diagnostics(&mut self, report: &DiagnosticReport) -> Result<()> {
        match self.format {
            OutputFormat::Json => {
                let json = serde_json::to_string_pretty(report).map_err(|e| {
                    WindWardenError::internal_error(format!(
                        "Failed to serialize diagnostic report: {}",
                        e
                    ))
                })?;
                writeln!(self.writer, "{}", json).map_err(|e| {
                    WindWardenError::internal_error(format!("Failed to write output: {}", e))
                })?;
            }
            OutputFormat::Text => {
                self.write_text_diagnostics(report)?;
            }
            _ => {
                return Err(WindWardenError::internal_error(
                    "Diagnostic reports are only supported in JSON and Text formats",
                ));
            }
        }
        Ok(())
    }

    /// Report check results
    pub fn report_check(&mut self, report: &CheckReport) -> Result<()> {
        match self.format {
            OutputFormat::Json | OutputFormat::Check => {
                let json = serde_json::to_string_pretty(report).map_err(|e| {
                    WindWardenError::internal_error(format!(
                        "Failed to serialize check report: {}",
                        e
                    ))
                })?;
                writeln!(self.writer, "{}", json).map_err(|e| {
                    WindWardenError::internal_error(format!("Failed to write output: {}", e))
                })?;
            }
            OutputFormat::Text => {
                self.write_text_check(report)?;
            }
            _ => {
                return Err(WindWardenError::internal_error(
                    "Check reports are only supported in JSON, Check, and Text formats",
                ));
            }
        }
        Ok(())
    }

    /// Report diff results
    pub fn report_diff(&mut self, report: &DiffReport) -> Result<()> {
        match self.format {
            OutputFormat::Json | OutputFormat::Diff => {
                let json = serde_json::to_string_pretty(report).map_err(|e| {
                    WindWardenError::internal_error(format!(
                        "Failed to serialize diff report: {}",
                        e
                    ))
                })?;
                writeln!(self.writer, "{}", json).map_err(|e| {
                    WindWardenError::internal_error(format!("Failed to write output: {}", e))
                })?;
            }
            OutputFormat::Text => {
                self.write_text_diff(report)?;
            }
            _ => {
                return Err(WindWardenError::internal_error(
                    "Diff reports are only supported in JSON, Diff, and Text formats",
                ));
            }
        }
        Ok(())
    }

    /// Report summary results
    pub fn report_summary(&mut self, report: &SummaryReport) -> Result<()> {
        match self.format {
            OutputFormat::Json | OutputFormat::Summary => {
                let json = serde_json::to_string_pretty(report).map_err(|e| {
                    WindWardenError::internal_error(format!(
                        "Failed to serialize summary report: {}",
                        e
                    ))
                })?;
                writeln!(self.writer, "{}", json).map_err(|e| {
                    WindWardenError::internal_error(format!("Failed to write output: {}", e))
                })?;
            }
            OutputFormat::Text => {
                self.write_text_summary(report)?;
            }
            _ => {
                return Err(WindWardenError::internal_error(
                    "Summary reports are only supported in JSON, Summary, and Text formats",
                ));
            }
        }
        Ok(())
    }

    /// Write diagnostic report in human-readable text format
    fn write_text_diagnostics(&mut self, report: &DiagnosticReport) -> Result<()> {
        for file_result in &report.results {
            if !file_result.messages.is_empty() {
                writeln!(self.writer, "\n{}", file_result.file_path).map_err(|e| {
                    WindWardenError::internal_error(format!("Failed to write output: {}", e))
                })?;

                for issue in &file_result.messages {
                    let severity_str = match issue.severity {
                        Severity::Error => "error",
                        Severity::Warning => "warning",
                        Severity::Info => "info",
                    };

                    writeln!(
                        self.writer,
                        "  {}:{} {} {} {}",
                        issue.line, issue.column, severity_str, issue.message, issue.rule_id
                    )
                    .map_err(|e| {
                        WindWardenError::internal_error(format!("Failed to write output: {}", e))
                    })?;

                    if !issue.source.is_empty() {
                        writeln!(self.writer, "    > {}", issue.source).map_err(|e| {
                            WindWardenError::internal_error(format!(
                                "Failed to write output: {}",
                                e
                            ))
                        })?;
                    }
                }
            }
        }

        if report.error_count > 0 || report.warning_count > 0 {
            writeln!(
                self.writer,
                "\n✖ {} problems ({} errors, {} warnings)",
                report.error_count + report.warning_count,
                report.error_count,
                report.warning_count
            )
            .map_err(|e| {
                WindWardenError::internal_error(format!("Failed to write output: {}", e))
            })?;

            if report.fixable_error_count > 0 || report.fixable_warning_count > 0 {
                writeln!(
                    self.writer,
                    "  {} fixable with --fix",
                    report.fixable_error_count + report.fixable_warning_count
                )
                .map_err(|e| {
                    WindWardenError::internal_error(format!("Failed to write output: {}", e))
                })?;
            }
        } else {
            writeln!(self.writer, "✓ No problems found").map_err(|e| {
                WindWardenError::internal_error(format!("Failed to write output: {}", e))
            })?;
        }

        Ok(())
    }

    /// Write check report in human-readable text format
    fn write_text_check(&mut self, report: &CheckReport) -> Result<()> {
        if report.unformatted_files.is_empty() {
            writeln!(self.writer, "All files are properly formatted!").map_err(|e| {
                WindWardenError::internal_error(format!("Failed to write output: {}", e))
            })?;
        } else {
            writeln!(
                self.writer,
                "Found {} unformatted files:",
                report.unformatted_files.len()
            )
            .map_err(|e| {
                WindWardenError::internal_error(format!("Failed to write output: {}", e))
            })?;

            for file in &report.unformatted_files {
                writeln!(self.writer, "  {}", file.file_path).map_err(|e| {
                    WindWardenError::internal_error(format!("Failed to write output: {}", e))
                })?;
            }
        }

        writeln!(
            self.writer,
            "\nChecked {} files, {} formatted, {} unformatted",
            report.summary.total_files,
            report.summary.formatted_files,
            report.summary.unformatted_files
        )
        .map_err(|e| WindWardenError::internal_error(format!("Failed to write output: {}", e)))?;

        Ok(())
    }

    /// Write diff report in human-readable text format
    fn write_text_diff(&mut self, report: &DiffReport) -> Result<()> {
        if report.changes.is_empty() {
            writeln!(
                self.writer,
                "No changes needed - all files are properly formatted!"
            )
            .map_err(|e| {
                WindWardenError::internal_error(format!("Failed to write output: {}", e))
            })?;
        } else {
            for file_changes in &report.changes {
                writeln!(self.writer, "\n{}", file_changes.file_path).map_err(|e| {
                    WindWardenError::internal_error(format!("Failed to write output: {}", e))
                })?;

                for modification in &file_changes.modifications {
                    writeln!(
                        self.writer,
                        "  Line {}:{} - {}",
                        modification.line, modification.column, modification.reason
                    )
                    .map_err(|e| {
                        WindWardenError::internal_error(format!("Failed to write output: {}", e))
                    })?;

                    writeln!(self.writer, "    - {}", modification.original_text).map_err(|e| {
                        WindWardenError::internal_error(format!("Failed to write output: {}", e))
                    })?;
                    writeln!(self.writer, "    + {}", modification.new_text).map_err(|e| {
                        WindWardenError::internal_error(format!("Failed to write output: {}", e))
                    })?;
                }
            }
        }

        writeln!(
            self.writer,
            "\nProcessed {} files, changed {} files, sorted {} classes",
            report.summary.files_processed,
            report.summary.files_changed,
            report.summary.classes_sorted
        )
        .map_err(|e| WindWardenError::internal_error(format!("Failed to write output: {}", e)))?;

        Ok(())
    }

    /// Write summary report in human-readable text format
    fn write_text_summary(&mut self, report: &SummaryReport) -> Result<()> {
        writeln!(
            self.writer,
            "WindWarden Summary:\n  Files processed: {}\n  Files changed: {}\n  Classes processed: {}\n  Classes sorted: {}",
            report.summary.files_processed,
            report.summary.files_changed,
            report.summary.classes_processed,
            report.summary.classes_sorted
        ).map_err(|e| WindWardenError::internal_error(format!("Failed to write output: {}", e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_json_diagnostic_output() {
        let mut report = DiagnosticReport::new();
        let mut file_result = FileResult::new("/test/file.tsx");

        file_result.add_issue(Issue {
            rule_id: "class-order".to_string(),
            severity: Severity::Warning,
            message: "Classes not in Tailwind order".to_string(),
            line: 10,
            column: 5,
            end_line: 10,
            end_column: 25,
            source: "p-4 bg-blue-500 flex".to_string(),
            suggestions: vec![Suggestion {
                desc: "Sort classes".to_string(),
                fix: Fix {
                    range: [120, 140],
                    text: "flex p-4 bg-blue-500".to_string(),
                },
            }],
        });

        report.add_file_result(file_result);

        let buffer = Cursor::new(Vec::new());
        let mut reporter = Reporter::new(OutputFormat::Json, Box::new(buffer));

        reporter.report_diagnostics(&report).unwrap();

        // For testing purposes, we'll just verify the call succeeds
        // In real usage, the output would go to the writer
    }

    #[test]
    fn test_text_summary_output() {
        let mut report = SummaryReport::new();
        report.summary.files_processed = 10;
        report.summary.files_changed = 3;
        report.summary.classes_processed = 150;
        report.summary.classes_sorted = 25;

        let buffer = Cursor::new(Vec::new());
        let mut reporter = Reporter::new(OutputFormat::Text, Box::new(buffer));

        reporter.report_summary(&report).unwrap();

        // For testing purposes, we'll just verify the call succeeds
        // In real usage, the output would go to the writer
    }
}
