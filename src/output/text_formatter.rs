// This file contains the text formatting functionality from the original output.rs
use crate::diff::{DiffFormatter, FileDiff};
use crate::file_processor::BatchProcessingResults;
use colored::Colorize;
use std::time::Duration;

/// Output formatting for CLI results
pub struct OutputFormatter {
    show_stats: bool,
    show_diff: bool,
    diff_formatter: DiffFormatter,
}

impl OutputFormatter {
    pub fn new(show_stats: bool) -> Self {
        Self {
            show_stats,
            show_diff: false,
            diff_formatter: DiffFormatter::new(),
        }
    }

    pub fn with_diff(mut self, show_diff: bool) -> Self {
        self.show_diff = show_diff;
        self
    }

    /// Format results for check mode (preview)
    pub fn format_check_results(
        &self,
        results: &BatchProcessingResults,
        duration: Option<Duration>,
    ) -> String {
        let mut output = Vec::new();

        // Show diffs if requested
        if self.show_diff {
            let changed_files: Vec<_> = results
                .results
                .iter()
                .filter(|r| r.changes_made && r.success)
                .collect();

            for result in &changed_files {
                if let (Some(original), Some(processed)) =
                    (&result.original_content, &result.processed_content)
                {
                    let diff = FileDiff::new(
                        result.file_path.display().to_string(),
                        original.clone(),
                        processed.clone(),
                    );

                    if diff.has_changes {
                        output.push(self.diff_formatter.format_diff(&diff));
                        output.push(String::new()); // Empty line between files
                    }
                }
            }
        }

        // Show changed files summary
        let changed_files: Vec<_> = results
            .results
            .iter()
            .filter(|r| r.changes_made && r.success)
            .collect();

        if !changed_files.is_empty() && !self.show_diff {
            // Only show file list if we're not already showing diffs
            output.push("Files that would be formatted:".green().bold().to_string());
            for result in &changed_files {
                let file_path = result.file_path.display();
                output.push(format!("  {}", file_path.to_string().cyan()));
            }
            output.push(String::new());
        }

        // Show failed files
        let failed_files: Vec<_> = results.results.iter().filter(|r| !r.success).collect();

        if !failed_files.is_empty() {
            output.push("Failed to process:".red().bold().to_string());
            for result in &failed_files {
                let file_path = result.file_path.display();
                let error = result.error.as_deref().unwrap_or("Unknown error");
                output.push(format!(
                    "  {}: {}",
                    file_path.to_string().cyan(),
                    error.red()
                ));
            }
            output.push(String::new());
        }

        // Summary
        if results.files_with_changes > 0 {
            output.push(format!(
                "{} {} would be formatted",
                results.files_with_changes.to_string().yellow().bold(),
                if results.files_with_changes == 1 {
                    "file"
                } else {
                    "files"
                }
            ));
        } else {
            output.push("All files are already formatted!".green().to_string());
        }

        if self.show_stats {
            output.push(String::new());
            output.push(self.format_statistics(results, duration));
        }

        output.join("\n")
    }

    /// Format results for write mode
    pub fn format_write_results(
        &self,
        results: &BatchProcessingResults,
        duration: Option<Duration>,
    ) -> String {
        let mut output = Vec::new();

        // Show formatted files
        let formatted_files: Vec<_> = results
            .results
            .iter()
            .filter(|r| r.changes_made && r.success)
            .collect();

        if !formatted_files.is_empty() {
            output.push("Formatted files:".green().bold().to_string());
            for result in &formatted_files {
                let file_path = result.file_path.display();
                output.push(format!("  {}", file_path.to_string().cyan()));
            }
            output.push(String::new());
        }

        // Show failed files
        let failed_files: Vec<_> = results.results.iter().filter(|r| !r.success).collect();

        if !failed_files.is_empty() {
            output.push("Failed to process:".red().bold().to_string());
            for result in &failed_files {
                let file_path = result.file_path.display();
                let error = result.error.as_deref().unwrap_or("Unknown error");
                output.push(format!(
                    "  {}: {}",
                    file_path.to_string().cyan(),
                    error.red()
                ));
            }
            output.push(String::new());
        }

        // Summary
        if results.files_with_changes > 0 {
            output.push(format!(
                "{} {} formatted",
                results.files_with_changes.to_string().green().bold(),
                if results.files_with_changes == 1 {
                    "file"
                } else {
                    "files"
                }
            ));
        } else {
            output.push("No files needed formatting!".green().to_string());
        }

        if self.show_stats {
            output.push(String::new());
            output.push(self.format_statistics(results, duration));
        }

        output.join("\n")
    }

    /// Format results for verify mode
    pub fn format_verify_results(
        &self,
        results: &BatchProcessingResults,
        duration: Option<Duration>,
    ) -> String {
        let mut output = Vec::new();

        // Show unformatted files
        let unformatted_files: Vec<_> = results
            .results
            .iter()
            .filter(|r| r.changes_made && r.success)
            .collect();

        if !unformatted_files.is_empty() {
            output.push("Unformatted files:".red().bold().to_string());
            for result in &unformatted_files {
                let file_path = result.file_path.display();
                output.push(format!("  {}", file_path.to_string().cyan()));
            }
            output.push(String::new());
        }

        // Show failed files
        let failed_files: Vec<_> = results.results.iter().filter(|r| !r.success).collect();

        if !failed_files.is_empty() {
            output.push("Failed to process:".red().bold().to_string());
            for result in &failed_files {
                let file_path = result.file_path.display();
                let error = result.error.as_deref().unwrap_or("Unknown error");
                output.push(format!(
                    "  {}: {}",
                    file_path.to_string().cyan(),
                    error.red()
                ));
            }
            output.push(String::new());
        }

        // Summary
        if results.files_with_changes > 0 {
            output.push(format!(
                "{} {} not formatted",
                results.files_with_changes.to_string().red().bold(),
                if results.files_with_changes == 1 {
                    "file"
                } else {
                    "files"
                }
            ));
        } else {
            output.push("All files are properly formatted!".green().to_string());
        }

        if self.show_stats {
            output.push(String::new());
            output.push(self.format_statistics(results, duration));
        }

        output.join("\n")
    }

    /// Format processing statistics
    fn format_statistics(
        &self,
        results: &BatchProcessingResults,
        duration: Option<Duration>,
    ) -> String {
        let mut stats = Vec::new();

        stats.push("Statistics:".bold().to_string());
        stats.push(format!("  Total files: {}", results.total_files));
        stats.push(format!("  Processed: {}", results.processed_files));
        stats.push(format!("  Changed: {}", results.files_with_changes));
        stats.push(format!("  Failed: {}", results.failed_files));
        stats.push(format!(
            "  Success rate: {:.1}%",
            results.success_rate() * 100.0
        ));

        if let Some(duration) = duration {
            stats.push(format!("  Duration: {:.2}s", duration.as_secs_f64()));

            if results.total_files > 0 {
                let files_per_sec = results.total_files as f64 / duration.as_secs_f64();
                stats.push(format!("  Files/sec: {:.1}", files_per_sec));
            }
        }

        stats.join("\n")
    }

    /// Determine exit code based on operation mode and results
    pub fn get_exit_code(
        &self,
        operation_mode: &crate::cli::OperationMode,
        results: &BatchProcessingResults,
    ) -> i32 {
        match operation_mode {
            crate::cli::OperationMode::Check | crate::cli::OperationMode::Write => {
                // For check and write modes, exit with error only if there were failures
                if results.failed_files > 0 {
                    1
                } else {
                    0
                }
            }
            crate::cli::OperationMode::Verify => {
                // For verify mode, exit with error if files need formatting or there were failures
                if results.files_with_changes > 0 || results.failed_files > 0 {
                    1
                } else {
                    0
                }
            }
        }
    }
}

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;

/// Progress reporting for large file processing operations
pub struct ProgressReporter {
    total: usize,
    processed: Arc<AtomicUsize>,
    start_time: Instant,
    show_progress: bool,
    last_update: std::time::Instant,
    update_interval: Duration,
}

impl ProgressReporter {
    pub fn new(total: usize, show_progress: bool) -> Self {
        Self {
            total,
            processed: Arc::new(AtomicUsize::new(0)),
            start_time: Instant::now(),
            show_progress,
            last_update: Instant::now(),
            update_interval: Duration::from_millis(100), // Update every 100ms
        }
    }

    /// Get a thread-safe counter for tracking progress
    pub fn get_counter(&self) -> Arc<AtomicUsize> {
        self.processed.clone()
    }

    /// Update and display progress if enough time has passed
    pub fn update(&mut self) {
        if !self.show_progress || self.total == 0 {
            return;
        }

        let now = Instant::now();
        if now.duration_since(self.last_update) < self.update_interval
            && self.get_current() < self.total
        {
            return;
        }

        self.last_update = now;
        self.display_progress();
    }

    /// Force display the current progress
    pub fn display_progress(&self) {
        if !self.show_progress {
            return;
        }

        let current = self.get_current();
        let elapsed = self.start_time.elapsed();

        if self.total < 10 {
            // For small file counts, just show simple progress
            eprint!("\rProcessing files: {}/{}", current, self.total);
        } else {
            // For larger file counts, show detailed progress with ETA
            let percentage = (current as f64 / self.total as f64) * 100.0;
            let progress_bar = self.create_progress_bar(percentage);

            if current > 0 && current < self.total {
                // Estimate time remaining
                let rate = current as f64 / elapsed.as_secs_f64();
                let remaining = (self.total - current) as f64 / rate;
                let eta = Duration::from_secs_f64(remaining);

                eprint!(
                    "\r{} {}/{} ({:.1}%) ETA: {}",
                    progress_bar,
                    current,
                    self.total,
                    percentage,
                    self.format_duration(eta)
                );
            } else {
                eprint!(
                    "\r{} {}/{} ({:.1}%)",
                    progress_bar, current, self.total, percentage
                );
            }
        }

        if current >= self.total {
            // Show completion message
            let total_time = self.start_time.elapsed();
            let rate = self.total as f64 / total_time.as_secs_f64();
            eprintln!(
                "\r✅ Processed {} files in {} ({:.1} files/sec)",
                self.total,
                self.format_duration(total_time),
                rate
            );
        }
    }

    /// Get current progress count
    fn get_current(&self) -> usize {
        self.processed.load(Ordering::Relaxed)
    }

    /// Create a visual progress bar
    fn create_progress_bar(&self, percentage: f64) -> String {
        let width = 20;
        let filled = ((percentage / 100.0) * width as f64) as usize;
        let empty = width - filled;

        format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
    }

    /// Format duration in a human-readable way
    fn format_duration(&self, duration: Duration) -> String {
        let secs = duration.as_secs();
        if secs < 60 {
            format!("{}s", secs)
        } else if secs < 3600 {
            format!("{}m{}s", secs / 60, secs % 60)
        } else {
            format!("{}h{}m{}s", secs / 3600, (secs % 3600) / 60, secs % 60)
        }
    }

    /// Finish progress reporting
    pub fn finish(&self) {
        if self.show_progress {
            self.display_progress();
        }
    }
}

/// Thread-safe progress tracker for parallel processing
#[derive(Clone)]
pub struct ProgressTracker {
    counter: Arc<AtomicUsize>,
}

impl ProgressTracker {
    pub fn new(counter: Arc<AtomicUsize>) -> Self {
        Self { counter }
    }

    /// Increment the progress counter
    pub fn increment(&self) {
        self.counter.fetch_add(1, Ordering::Relaxed);
    }

    /// Get current count
    pub fn get(&self) -> usize {
        self.counter.load(Ordering::Relaxed)
    }
}
