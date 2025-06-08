use crate::config::Config;
use crate::output::ProgressTracker;
use crate::processor::FileProcessor as ContentProcessor;
use crate::{ProcessOptions, Result, WindWardenError};
use globset::{Glob, GlobSet, GlobSetBuilder};
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Configuration for file discovery
#[derive(Debug, Clone)]
pub struct FileDiscoveryConfig {
    /// File extensions to process (e.g., "tsx", "jsx", "ts", "js")
    pub extensions: Vec<String>,
    /// Patterns to exclude (gitignore-style)
    pub exclude_patterns: Vec<String>,
    /// Maximum depth for directory traversal
    pub max_depth: Option<usize>,
    /// Follow symbolic links
    pub follow_links: bool,
}

impl Default for FileDiscoveryConfig {
    fn default() -> Self {
        Self {
            extensions: vec![
                "tsx".to_string(),
                "jsx".to_string(),
                "ts".to_string(),
                "js".to_string(),
            ],
            exclude_patterns: vec![
                "node_modules/**".to_string(),
                ".git/**".to_string(),
                "dist/**".to_string(),
                "build/**".to_string(),
                "coverage/**".to_string(),
                ".next/**".to_string(),
                ".nuxt/**".to_string(),
                "target/**".to_string(),
            ],
            max_depth: None,
            follow_links: false,
        }
    }
}

/// File discovery engine for finding files to process
pub struct FileDiscovery {
    config: FileDiscoveryConfig,
    exclude_set: GlobSet,
}

impl FileDiscovery {
    pub fn new(config: FileDiscoveryConfig) -> Result<Self> {
        let exclude_set = Self::build_exclude_set(&config.exclude_patterns)?;

        Ok(Self {
            config,
            exclude_set,
        })
    }

    /// Extract directory names from exclude patterns for direct directory name checking
    fn get_excluded_directories(&self) -> Vec<String> {
        let mut dirs = Vec::new();
        for pattern in &self.config.exclude_patterns {
            // Extract directory name from patterns like "dirname/**"
            if let Some(dir_name) = pattern.strip_suffix("/**") {
                dirs.push(dir_name.to_string());
            } else if !pattern.contains('*') && !pattern.contains('?') && !pattern.contains('[') {
                // Plain directory names without globs
                dirs.push(pattern.clone());
            }
        }
        dirs
    }

    /// Find all files matching the criteria from the given paths
    pub fn discover_files(&self, paths: &[String]) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for path_str in paths {
            if path_str == "--stdin" || path_str == "-" {
                // Handle stdin case - no files to discover
                continue;
            }

            // Check if this is a glob pattern first
            if path_str.contains('*') || path_str.contains('?') || path_str.contains('[') {
                files.extend(self.discover_files_by_glob(path_str)?);
                continue;
            }

            let path = Path::new(path_str);

            if !path.exists() {
                return Err(WindWardenError::file_not_found(path_str));
            }

            if path.is_file() {
                if self.should_process_file(path) {
                    files.push(path.to_path_buf());
                }
            } else if path.is_dir() {
                files.extend(self.discover_files_in_directory(path)?);
            }
        }

        // Remove duplicates and sort for deterministic output
        files.sort();
        files.dedup();

        Ok(files)
    }

    /// Discover files in a directory recursively
    fn discover_files_in_directory(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        let walkdir = WalkDir::new(dir)
            .follow_links(self.config.follow_links)
            .max_depth(self.config.max_depth.unwrap_or(usize::MAX))
            .into_iter()
            .filter_entry(|e| !self.is_excluded(e.path()));

        for entry in walkdir {
            let entry = entry.map_err(|e| WindWardenError::Io(std::io::Error::other(e)))?;

            if entry.file_type().is_file() && self.should_process_file(entry.path()) {
                files.push(entry.path().to_path_buf());
            }
        }

        Ok(files)
    }

    /// Discover files using glob patterns
    fn discover_files_by_glob(&self, pattern: &str) -> Result<Vec<PathBuf>> {
        let glob = Glob::new(pattern)
            .map_err(|e| WindWardenError::glob_pattern_error(pattern, e.to_string()))?;

        let matcher = glob.compile_matcher();
        let mut files = Vec::new();

        // Find all files that match the glob pattern
        // For now, we'll walk the current directory and match
        // In a more sophisticated implementation, we could optimize this
        let walkdir = WalkDir::new(".")
            .follow_links(self.config.follow_links)
            .max_depth(self.config.max_depth.unwrap_or(usize::MAX))
            .into_iter()
            .filter_entry(|e| !self.is_excluded(e.path()));

        for entry in walkdir {
            let entry = entry.map_err(|e| WindWardenError::Io(std::io::Error::other(e)))?;

            let path = entry.path();

            if entry.file_type().is_file() && self.should_process_file(path) {
                // Try matching both the full path and just the relative path without "./"
                let relative_path = path.strip_prefix("./").unwrap_or(path);

                if matcher.is_match(path) || matcher.is_match(relative_path) {
                    files.push(path.to_path_buf());
                }
            }
        }

        Ok(files)
    }

    /// Check if a file should be processed based on extension
    fn should_process_file(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            self.config.extensions.iter().any(|ext| ext == extension)
        } else {
            false
        }
    }

    /// Check if a path is excluded by patterns
    fn is_excluded(&self, path: &Path) -> bool {
        let excluded_dirs = self.get_excluded_directories();

        // Check if any component of the path matches an exclude pattern
        for component in path.components() {
            if let std::path::Component::Normal(name) = component {
                let name_str = name.to_string_lossy();
                // Check if this directory name is in our exclude list
                if excluded_dirs.iter().any(|dir| dir == &*name_str) {
                    return true;
                }
            }
        }

        // Also check the full path against patterns
        self.exclude_set.is_match(path)
    }

    /// Build the exclude glob set from patterns
    fn build_exclude_set(patterns: &[String]) -> Result<GlobSet> {
        let mut builder = GlobSetBuilder::new();

        for pattern in patterns {
            let glob = Glob::new(pattern)
                .map_err(|e| WindWardenError::glob_pattern_error(pattern, e.to_string()))?;
            builder.add(glob);
        }

        builder.build().map_err(|e| {
            WindWardenError::glob_pattern_error(
                "exclude pattern set",
                format!("Failed to build exclude pattern set: {}", e),
            )
        })
    }
}

/// File processing results for a single file
#[derive(Debug, Clone)]
pub struct FileProcessingResult {
    pub file_path: PathBuf,
    pub success: bool,
    pub changes_made: bool,
    pub original_content: Option<String>,
    pub processed_content: Option<String>,
    pub error: Option<String>,
}

impl FileProcessingResult {
    pub fn success(
        file_path: PathBuf,
        changes_made: bool,
        original_content: String,
        processed_content: String,
    ) -> Self {
        Self {
            file_path,
            success: true,
            changes_made,
            original_content: Some(original_content),
            processed_content: Some(processed_content),
            error: None,
        }
    }

    pub fn error(file_path: PathBuf, error: String) -> Self {
        Self {
            file_path,
            success: false,
            changes_made: false,
            original_content: None,
            processed_content: None,
            error: Some(error),
        }
    }
}

/// Batch file processing results
#[derive(Debug)]
pub struct BatchProcessingResults {
    pub total_files: usize,
    pub processed_files: usize,
    pub files_with_changes: usize,
    pub failed_files: usize,
    pub results: Vec<FileProcessingResult>,
}

impl Default for BatchProcessingResults {
    fn default() -> Self {
        Self::new()
    }
}

impl BatchProcessingResults {
    pub fn new() -> Self {
        Self {
            total_files: 0,
            processed_files: 0,
            files_with_changes: 0,
            failed_files: 0,
            results: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: FileProcessingResult) {
        self.total_files += 1;

        if result.success {
            self.processed_files += 1;
            if result.changes_made {
                self.files_with_changes += 1;
            }
        } else {
            self.failed_files += 1;
        }

        self.results.push(result);
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_files == 0 {
            1.0
        } else {
            self.processed_files as f64 / self.total_files as f64
        }
    }
}

/// Processing mode configuration
#[derive(Debug, Clone, Copy, Default)]
pub enum ProcessingMode {
    /// Process files sequentially (single-threaded)
    Sequential,
    /// Process files in parallel using all available CPU cores
    #[default]
    Parallel,
    /// Process files in parallel with a specific number of threads
    ParallelWithThreads(usize),
}

/// File processing pipeline with support for both sequential and parallel processing
pub struct FileProcessingPipeline {
    discovery: FileDiscovery,
    content_processor: ContentProcessor,
    processing_mode: ProcessingMode,
    windwarden_config: Option<Config>,
}

impl FileProcessingPipeline {
    pub fn new(config: FileDiscoveryConfig) -> Result<Self> {
        Self::new_with_mode(config, ProcessingMode::default())
    }

    pub fn new_with_mode(
        config: FileDiscoveryConfig,
        processing_mode: ProcessingMode,
    ) -> Result<Self> {
        Ok(Self {
            discovery: FileDiscovery::new(config)?,
            content_processor: ContentProcessor::new(),
            processing_mode,
            windwarden_config: None,
        })
    }

    pub fn new_with_windwarden_config(
        file_config: FileDiscoveryConfig,
        windwarden_config: &Config,
        processing_mode: ProcessingMode,
    ) -> Result<Self> {
        Ok(Self {
            discovery: FileDiscovery::new(file_config)?,
            content_processor: ContentProcessor::new_with_config(windwarden_config),
            processing_mode,
            windwarden_config: Some(windwarden_config.clone()),
        })
    }

    /// Process multiple files or paths using the configured processing mode
    pub fn process_files(
        &self,
        paths: &[String],
        options: ProcessOptions,
    ) -> Result<BatchProcessingResults> {
        self.process_files_with_progress(paths, options, None)
    }

    /// Process multiple files with optional progress tracking
    pub fn process_files_with_progress(
        &self,
        paths: &[String],
        options: ProcessOptions,
        progress_tracker: Option<ProgressTracker>,
    ) -> Result<BatchProcessingResults> {
        // Discover all files to process
        let files = self.discovery.discover_files(paths)?;

        match self.processing_mode {
            ProcessingMode::Sequential => {
                self.process_files_sequential(files, options, progress_tracker)
            }
            ProcessingMode::Parallel => {
                self.process_files_parallel(files, options, progress_tracker)
            }
            ProcessingMode::ParallelWithThreads(num_threads) => self
                .process_files_parallel_with_threads(files, options, num_threads, progress_tracker),
        }
    }

    /// Process files sequentially (single-threaded)
    fn process_files_sequential(
        &self,
        files: Vec<PathBuf>,
        options: ProcessOptions,
        progress_tracker: Option<ProgressTracker>,
    ) -> Result<BatchProcessingResults> {
        let mut results = BatchProcessingResults::new();

        // Process each file sequentially
        for file_path in files {
            let result = self.process_single_file(&file_path, &options);
            results.add_result(result);

            // Update progress if tracker is provided
            if let Some(ref tracker) = progress_tracker {
                tracker.increment();
            }
        }

        Ok(results)
    }

    /// Process files in parallel using all available CPU cores
    fn process_files_parallel(
        &self,
        files: Vec<PathBuf>,
        options: ProcessOptions,
        progress_tracker: Option<ProgressTracker>,
    ) -> Result<BatchProcessingResults> {
        let mut results = BatchProcessingResults::new();

        // Clone the config outside the parallel block to avoid Sync issues
        let config_clone = self.windwarden_config.clone();

        // Process files in parallel and collect results
        // Each thread gets its own ContentProcessor to avoid Sync issues with Oxc allocator
        let file_results: Vec<FileProcessingResult> = files
            .par_iter()
            .map(|file_path| {
                let thread_processor = if let Some(ref config) = config_clone {
                    ContentProcessor::new_with_config(config)
                } else {
                    ContentProcessor::new()
                };
                let result = Self::process_single_file_with_processor(
                    &thread_processor,
                    file_path,
                    &options,
                );

                // Update progress if tracker is provided
                if let Some(ref tracker) = progress_tracker {
                    tracker.increment();
                }

                result
            })
            .collect();

        // Add all results to the batch
        for result in file_results {
            results.add_result(result);
        }

        Ok(results)
    }

    /// Process files in parallel with a specific number of threads
    fn process_files_parallel_with_threads(
        &self,
        files: Vec<PathBuf>,
        options: ProcessOptions,
        num_threads: usize,
        progress_tracker: Option<ProgressTracker>,
    ) -> Result<BatchProcessingResults> {
        // Configure Rayon thread pool
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .map_err(|e| {
                WindWardenError::thread_pool_error(format!(
                    "Failed to create thread pool with {} threads: {}",
                    num_threads, e
                ))
            })?;

        let mut results = BatchProcessingResults::new();

        // Clone the config outside the parallel block to avoid Sync issues
        let config_clone = self.windwarden_config.clone();

        // Process files in parallel with the configured thread pool
        // Each thread gets its own ContentProcessor to avoid Sync issues with Oxc allocator
        let file_results: Vec<FileProcessingResult> = pool.install(|| {
            files
                .par_iter()
                .map(|file_path| {
                    // Create a new ContentProcessor for this thread
                    let thread_processor = if let Some(ref config) = config_clone {
                        ContentProcessor::new_with_config(config)
                    } else {
                        ContentProcessor::new()
                    };
                    let result = Self::process_single_file_with_processor(
                        &thread_processor,
                        file_path,
                        &options,
                    );

                    // Update progress if tracker is provided
                    if let Some(ref tracker) = progress_tracker {
                        tracker.increment();
                    }

                    result
                })
                .collect()
        });

        // Add all results to the batch
        for result in file_results {
            results.add_result(result);
        }

        Ok(results)
    }

    /// Process a single file and return detailed result
    fn process_single_file(
        &self,
        file_path: &Path,
        options: &ProcessOptions,
    ) -> FileProcessingResult {
        Self::process_single_file_with_processor(&self.content_processor, file_path, options)
    }

    /// Process a single file with a specific processor (for parallel processing)
    fn process_single_file_with_processor(
        processor: &ContentProcessor,
        file_path: &Path,
        options: &ProcessOptions,
    ) -> FileProcessingResult {
        // Read file content
        let original_content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => {
                let path_str = file_path.display().to_string();
                let error_msg = match WindWardenError::from_io_error(e, Some(&path_str)) {
                    WindWardenError::FileNotFound { .. } => {
                        format!("File not found: {}", path_str)
                    }
                    WindWardenError::PermissionDenied { .. } => {
                        format!("Permission denied: {}", path_str)
                    }
                    WindWardenError::InvalidUtf8 { .. } => {
                        format!("File contains invalid UTF-8: {}", path_str)
                    }
                    err => format!("Failed to read file {}: {}", path_str, err),
                };

                return FileProcessingResult::error(file_path.to_path_buf(), error_msg);
            }
        };

        // Process content
        let file_path_str = file_path.to_string_lossy();
        let processed_content =
            match processor.process_content(&original_content, &file_path_str, options.clone()) {
                Ok(content) => content,
                Err(e) => {
                    let error_msg = match &e {
                        WindWardenError::ParseError {
                            file,
                            line,
                            message,
                        } => {
                            format!("Parse error in {} at line {}: {}", file, line, message)
                        }
                        WindWardenError::SortError { context, message } => {
                            format!("Sort error in {}: {}", context, message)
                        }
                        WindWardenError::UnsupportedFileType {
                            extension,
                            supported,
                        } => {
                            format!(
                                "Unsupported file type .{} (supported: {})",
                                extension, supported
                            )
                        }
                        _ => format!("Processing failed: {}", e),
                    };

                    return FileProcessingResult::error(file_path.to_path_buf(), error_msg);
                }
            };

        // Determine if changes were made
        let changes_made = if options.check_formatted {
            // For verify mode, we need to check if the content would change
            // Process in dry-run mode to see what would change
            let temp_options = ProcessOptions {
                dry_run: true,
                write: false,
                check_formatted: false,
            };
            match processor.process_content(&original_content, &file_path_str, temp_options) {
                Ok(temp_processed) => {
                    original_content != temp_processed && !temp_processed.is_empty()
                }
                Err(_) => false, // If processing fails, assume no changes
            }
        } else {
            // Compare original and processed content
            original_content != processed_content && !processed_content.is_empty()
        };

        FileProcessingResult::success(
            file_path.to_path_buf(),
            changes_made,
            original_content,
            processed_content,
        )
    }

    /// Get the underlying file discovery configuration
    pub fn discovery_config(&self) -> &FileDiscoveryConfig {
        &self.discovery.config
    }

    /// Get the current processing mode
    pub fn processing_mode(&self) -> ProcessingMode {
        self.processing_mode
    }

    /// Set the processing mode
    pub fn set_processing_mode(&mut self, mode: ProcessingMode) {
        self.processing_mode = mode;
    }

    /// Create a new pipeline with sequential processing
    pub fn sequential(config: FileDiscoveryConfig) -> Result<Self> {
        Self::new_with_mode(config, ProcessingMode::Sequential)
    }

    /// Create a new pipeline with parallel processing
    pub fn parallel(config: FileDiscoveryConfig) -> Result<Self> {
        Self::new_with_mode(config, ProcessingMode::Parallel)
    }

    /// Create a new pipeline with parallel processing using a specific number of threads
    pub fn parallel_with_threads(config: FileDiscoveryConfig, num_threads: usize) -> Result<Self> {
        Self::new_with_mode(config, ProcessingMode::ParallelWithThreads(num_threads))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_files(temp_dir: &TempDir) -> Result<()> {
        let base = temp_dir.path();

        // Create directory structure
        fs::create_dir_all(base.join("src/components"))?;
        fs::create_dir_all(base.join("src/pages"))?;
        fs::create_dir_all(base.join("node_modules/some-package"))?;
        fs::create_dir_all(base.join("dist"))?;

        // Create test files
        fs::write(base.join("src/App.tsx"), "// test tsx")?;
        fs::write(base.join("src/App.jsx"), "// test jsx")?;
        fs::write(base.join("src/components/Button.tsx"), "// button")?;
        fs::write(base.join("src/components/Card.jsx"), "// card")?;
        fs::write(base.join("src/pages/Home.ts"), "// home")?;
        fs::write(base.join("src/pages/About.js"), "// about")?;
        fs::write(base.join("package.json"), "{}")?;
        fs::write(base.join("README.md"), "# readme")?;
        fs::write(
            base.join("node_modules/some-package/index.js"),
            "// node_modules",
        )?;
        fs::write(base.join("dist/bundle.js"), "// dist")?;

        Ok(())
    }

    #[test]
    fn test_discover_files_in_directory() {
        let temp_dir = TempDir::new().unwrap();
        create_test_files(&temp_dir).unwrap();

        let config = FileDiscoveryConfig::default();
        let discovery = FileDiscovery::new(config).unwrap();

        let files = discovery
            .discover_files(&[temp_dir.path().to_string_lossy().to_string()])
            .unwrap();

        // Should find TypeScript/JavaScript files but exclude node_modules and dist
        assert!(!files.is_empty());

        // Check that we found the expected files
        let file_names: Vec<String> = files
            .iter()
            .filter_map(|p| p.file_name()?.to_str())
            .map(|s| s.to_string())
            .collect();

        // Verify we found files and they're in the expected directories
        assert!(!files.is_empty());

        assert!(file_names.contains(&"App.tsx".to_string()));
        assert!(file_names.contains(&"App.jsx".to_string()));
        assert!(file_names.contains(&"Button.tsx".to_string()));
        assert!(file_names.contains(&"Card.jsx".to_string()));
        assert!(file_names.contains(&"Home.ts".to_string()));
        assert!(file_names.contains(&"About.js".to_string()));

        // Should not include non-JS/TS files or excluded directories
        assert!(!file_names.contains(&"package.json".to_string()));
        assert!(!file_names.contains(&"README.md".to_string()));
        assert!(!file_names.contains(&"index.js".to_string())); // from node_modules
        assert!(!file_names.contains(&"bundle.js".to_string())); // from dist
    }

    #[test]
    fn test_discover_single_file() {
        let temp_dir = TempDir::new().unwrap();
        create_test_files(&temp_dir).unwrap();

        let config = FileDiscoveryConfig::default();
        let discovery = FileDiscovery::new(config).unwrap();

        let file_path = temp_dir.path().join("src/App.tsx");
        let files = discovery
            .discover_files(&[file_path.to_string_lossy().to_string()])
            .unwrap();

        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("App.tsx"));
    }

    #[test]
    fn test_custom_extensions() {
        let temp_dir = TempDir::new().unwrap();
        create_test_files(&temp_dir).unwrap();

        let config = FileDiscoveryConfig {
            extensions: vec!["tsx".to_string()], // Only TypeScript React files
            ..Default::default()
        };
        let discovery = FileDiscovery::new(config).unwrap();

        let files = discovery
            .discover_files(&[temp_dir.path().to_string_lossy().to_string()])
            .unwrap();

        // Should only find .tsx files
        for file in &files {
            assert!(file.extension().unwrap() == "tsx");
        }

        let file_names: Vec<String> = files
            .iter()
            .filter_map(|p| p.file_name()?.to_str())
            .map(|s| s.to_string())
            .collect();

        assert!(file_names.contains(&"App.tsx".to_string()));
        assert!(file_names.contains(&"Button.tsx".to_string()));
        assert!(!file_names.contains(&"App.jsx".to_string()));
        assert!(!file_names.contains(&"Card.jsx".to_string()));
    }

    #[test]
    fn test_glob_patterns() {
        let temp_dir = TempDir::new().unwrap();
        create_test_files(&temp_dir).unwrap();

        let config = FileDiscoveryConfig::default();
        let discovery = FileDiscovery::new(config).unwrap();

        // Change to the temp directory for relative glob patterns
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let files = discovery
            .discover_files(&["src/**/*.tsx".to_string()])
            .unwrap();

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();

        // Should find only .tsx files in src/
        assert!(!files.is_empty());
        for file in &files {
            assert_eq!(file.extension().unwrap(), "tsx");
            assert!(file.to_string_lossy().contains("src"));
        }
    }

    #[test]
    fn test_nonexistent_path() {
        let config = FileDiscoveryConfig::default();
        let discovery = FileDiscovery::new(config).unwrap();

        let result = discovery.discover_files(&["/nonexistent/path".to_string()]);
        assert!(result.is_err());
    }

    // ===== FILE PROCESSING PIPELINE TESTS =====

    fn create_test_content_files(temp_dir: &TempDir) -> Result<()> {
        let base = temp_dir.path();

        // Create directory structure
        fs::create_dir_all(base.join("src/components"))?;
        fs::create_dir_all(base.join("src/pages"))?;
        fs::create_dir_all(base.join("node_modules/some-package"))?;

        // Create test files with Tailwind classes that need sorting
        fs::write(
            base.join("src/App.tsx"),
            r#"
import React from "react";

export function App() {
  return <div className="p-4 flex m-2 items-center">Hello</div>;
}
"#,
        )?;

        fs::write(
            base.join("src/components/Button.tsx"),
            r#"
export function Button() {
  return (
    <button className="p-4 bg-blue-500 text-white rounded hover:bg-blue-600">
      Click me
    </button>
  );
}
"#,
        )?;

        // File with multiple patterns
        fs::write(
            base.join("src/components/Card.jsx"),
            r#"
import { cn } from "lib/utils";

export function Card() {
  const baseStyles = `p-4 border-2 rounded shadow`;
  return (
    <div className={cn("bg-white m-2 flex", "items-center justify-between")}>
      <span className="text-lg font-bold">Card</span>
    </div>
  );
}
"#,
        )?;

        // File with already sorted classes
        fs::write(
            base.join("src/pages/Home.ts"),
            r#"
const styles = "flex items-center m-2 p-4";
"#,
        )?;

        // File with no Tailwind classes
        fs::write(
            base.join("src/pages/About.js"),
            r#"
function About() {
  return "About page";
}

export default About;
"#,
        )?;

        // Excluded file (should not be processed)
        fs::write(
            base.join("node_modules/some-package/index.js"),
            r#"
export const classes = "p-4 flex m-2";
"#,
        )?;

        Ok(())
    }

    #[test]
    fn test_file_processing_pipeline_basic() {
        let temp_dir = TempDir::new().unwrap();
        create_test_content_files(&temp_dir).unwrap();

        let config = FileDiscoveryConfig::default();
        let pipeline = FileProcessingPipeline::new(config).unwrap();

        let options = ProcessOptions {
            dry_run: true,
            write: false,
            check_formatted: false,
        };

        let results = pipeline
            .process_files(&[temp_dir.path().to_string_lossy().to_string()], options)
            .unwrap();

        // Should process TypeScript/JavaScript files but exclude node_modules
        assert!(results.total_files > 0);
        assert_eq!(results.failed_files, 0);
        assert!(results.success_rate() == 1.0);

        // Check that we found files with changes
        assert!(results.files_with_changes > 0);

        // Verify specific files were processed
        let file_names: Vec<String> = results
            .results
            .iter()
            .filter_map(|r| r.file_path.file_name()?.to_str())
            .map(|s| s.to_string())
            .collect();

        assert!(file_names.contains(&"App.tsx".to_string()));
        assert!(file_names.contains(&"Button.tsx".to_string()));
        assert!(file_names.contains(&"Card.jsx".to_string()));
        assert!(file_names.contains(&"Home.ts".to_string()));
        assert!(file_names.contains(&"About.js".to_string()));

        // Should not include excluded files
        assert!(!file_names.contains(&"index.js".to_string())); // from node_modules
    }

    #[test]
    fn test_file_processing_pipeline_with_changes() {
        let temp_dir = TempDir::new().unwrap();
        create_test_content_files(&temp_dir).unwrap();

        let config = FileDiscoveryConfig::default();
        let pipeline = FileProcessingPipeline::new(config).unwrap();

        let options = ProcessOptions {
            dry_run: true,
            write: false,
            check_formatted: false,
        };

        let results = pipeline
            .process_files(&[temp_dir.path().to_string_lossy().to_string()], options)
            .unwrap();

        // Find the App.tsx result (should have changes)
        let app_result = results
            .results
            .iter()
            .find(|r| r.file_path.file_name().unwrap() == "App.tsx")
            .unwrap();

        assert!(app_result.success);
        assert!(app_result.changes_made);
        assert!(app_result.original_content.is_some());
        assert!(app_result.processed_content.is_some());

        // Check that classes were actually sorted
        let processed = app_result.processed_content.as_ref().unwrap();
        assert!(processed.contains("flex items-center m-2 p-4")); // sorted order
    }

    #[test]
    fn test_file_processing_pipeline_check_mode() {
        let temp_dir = TempDir::new().unwrap();
        create_test_content_files(&temp_dir).unwrap();

        let config = FileDiscoveryConfig::default();
        let pipeline = FileProcessingPipeline::new(config).unwrap();

        let options = ProcessOptions {
            dry_run: false,
            write: false,
            check_formatted: true,
        };

        let results = pipeline
            .process_files(&[temp_dir.path().to_string_lossy().to_string()], options)
            .unwrap();

        // In check mode, files with unsorted classes should fail processing
        // Files with already sorted classes should succeed
        assert!(results.total_files > 0);

        // Find the Home.ts result (should have no changes needed)
        let home_result = results
            .results
            .iter()
            .find(|r| r.file_path.file_name().unwrap() == "Home.ts");

        if let Some(home_result) = home_result {
            assert!(home_result.success);
            assert!(!home_result.changes_made);
        }
    }

    #[test]
    fn test_file_processing_pipeline_single_file() {
        let temp_dir = TempDir::new().unwrap();
        create_test_content_files(&temp_dir).unwrap();

        let config = FileDiscoveryConfig::default();
        let pipeline = FileProcessingPipeline::new(config).unwrap();

        let options = ProcessOptions {
            dry_run: true,
            write: false,
            check_formatted: false,
        };

        let file_path = temp_dir.path().join("src/App.tsx");
        let results = pipeline
            .process_files(&[file_path.to_string_lossy().to_string()], options)
            .unwrap();

        assert_eq!(results.total_files, 1);
        assert_eq!(results.processed_files, 1);
        assert_eq!(results.failed_files, 0);
        assert_eq!(results.files_with_changes, 1);

        let result = &results.results[0];
        assert!(result.success);
        assert!(result.changes_made);
    }

    #[test]
    fn test_file_processing_pipeline_custom_extensions() {
        let temp_dir = TempDir::new().unwrap();
        create_test_content_files(&temp_dir).unwrap();

        let config = FileDiscoveryConfig {
            extensions: vec!["tsx".to_string()], // Only TypeScript React files
            ..Default::default()
        };
        let pipeline = FileProcessingPipeline::new(config).unwrap();

        let options = ProcessOptions {
            dry_run: true,
            write: false,
            check_formatted: false,
        };

        let results = pipeline
            .process_files(&[temp_dir.path().to_string_lossy().to_string()], options)
            .unwrap();

        // Should only process .tsx files
        for result in &results.results {
            assert!(result.file_path.extension().unwrap() == "tsx");
        }

        let file_names: Vec<String> = results
            .results
            .iter()
            .filter_map(|r| r.file_path.file_name()?.to_str())
            .map(|s| s.to_string())
            .collect();

        assert!(file_names.contains(&"App.tsx".to_string()));
        assert!(file_names.contains(&"Button.tsx".to_string()));
        assert!(!file_names.contains(&"Card.jsx".to_string()));
        assert!(!file_names.contains(&"Home.ts".to_string()));
    }

    #[test]
    fn test_file_processing_result_constructors() {
        let test_path = PathBuf::from("test.tsx");

        // Test success result
        let success_result = FileProcessingResult::success(
            test_path.clone(),
            true,
            "original".to_string(),
            "processed".to_string(),
        );

        assert!(success_result.success);
        assert!(success_result.changes_made);
        assert_eq!(success_result.original_content.unwrap(), "original");
        assert_eq!(success_result.processed_content.unwrap(), "processed");
        assert!(success_result.error.is_none());

        // Test error result
        let error_result = FileProcessingResult::error(test_path, "test error".to_string());

        assert!(!error_result.success);
        assert!(!error_result.changes_made);
        assert!(error_result.original_content.is_none());
        assert!(error_result.processed_content.is_none());
        assert_eq!(error_result.error.unwrap(), "test error");
    }

    #[test]
    fn test_batch_processing_results_statistics() {
        let mut results = BatchProcessingResults::new();

        // Add successful results
        results.add_result(FileProcessingResult::success(
            PathBuf::from("file1.tsx"),
            true,
            "original1".to_string(),
            "processed1".to_string(),
        ));

        results.add_result(FileProcessingResult::success(
            PathBuf::from("file2.tsx"),
            false, // no changes
            "original2".to_string(),
            "original2".to_string(),
        ));

        // Add error result
        results.add_result(FileProcessingResult::error(
            PathBuf::from("file3.tsx"),
            "error".to_string(),
        ));

        assert_eq!(results.total_files, 3);
        assert_eq!(results.processed_files, 2);
        assert_eq!(results.files_with_changes, 1);
        assert_eq!(results.failed_files, 1);
        assert!((results.success_rate() - 0.6666666666666666).abs() < f64::EPSILON);
    }

    // ===== PARALLEL PROCESSING TESTS =====

    #[test]
    fn test_processing_mode_constructors() {
        let config = FileDiscoveryConfig::default();

        // Test sequential pipeline
        let sequential_pipeline = FileProcessingPipeline::sequential(config.clone()).unwrap();
        assert!(matches!(
            sequential_pipeline.processing_mode(),
            ProcessingMode::Sequential
        ));

        // Test parallel pipeline
        let parallel_pipeline = FileProcessingPipeline::parallel(config.clone()).unwrap();
        assert!(matches!(
            parallel_pipeline.processing_mode(),
            ProcessingMode::Parallel
        ));

        // Test parallel with threads pipeline
        let parallel_threads_pipeline =
            FileProcessingPipeline::parallel_with_threads(config, 4).unwrap();
        assert!(matches!(
            parallel_threads_pipeline.processing_mode(),
            ProcessingMode::ParallelWithThreads(4)
        ));
    }

    #[test]
    fn test_parallel_processing_basic() {
        let temp_dir = TempDir::new().unwrap();
        create_test_content_files(&temp_dir).unwrap();

        let config = FileDiscoveryConfig::default();
        let pipeline = FileProcessingPipeline::parallel(config).unwrap();

        let options = ProcessOptions {
            dry_run: true,
            write: false,
            check_formatted: false,
        };

        let results = pipeline
            .process_files(&[temp_dir.path().to_string_lossy().to_string()], options)
            .unwrap();

        // Should produce the same results as sequential processing
        assert!(results.total_files > 0);
        assert_eq!(results.failed_files, 0);
        assert!(results.success_rate() == 1.0);
        assert!(results.files_with_changes > 0);

        // Verify specific files were processed
        let file_names: Vec<String> = results
            .results
            .iter()
            .filter_map(|r| r.file_path.file_name()?.to_str())
            .map(|s| s.to_string())
            .collect();

        assert!(file_names.contains(&"App.tsx".to_string()));
        assert!(file_names.contains(&"Button.tsx".to_string()));
        assert!(file_names.contains(&"Card.jsx".to_string()));
    }

    #[test]
    fn test_parallel_with_specific_threads() {
        let temp_dir = TempDir::new().unwrap();
        create_test_content_files(&temp_dir).unwrap();

        let config = FileDiscoveryConfig::default();
        let pipeline = FileProcessingPipeline::parallel_with_threads(config, 2).unwrap();

        let options = ProcessOptions {
            dry_run: true,
            write: false,
            check_formatted: false,
        };

        let results = pipeline
            .process_files(&[temp_dir.path().to_string_lossy().to_string()], options)
            .unwrap();

        // Should produce the same results as sequential processing
        assert!(results.total_files > 0);
        assert_eq!(results.failed_files, 0);
        assert!(results.success_rate() == 1.0);
        assert!(results.files_with_changes > 0);
    }

    #[test]
    fn test_sequential_vs_parallel_results_consistency() {
        let temp_dir = TempDir::new().unwrap();
        create_test_content_files(&temp_dir).unwrap();

        let config = FileDiscoveryConfig::default();
        let sequential_pipeline = FileProcessingPipeline::sequential(config.clone()).unwrap();
        let parallel_pipeline = FileProcessingPipeline::parallel(config).unwrap();

        let options = ProcessOptions {
            dry_run: true,
            write: false,
            check_formatted: false,
        };

        let path = temp_dir.path().to_string_lossy().to_string();

        let sequential_results = sequential_pipeline
            .process_files(&[path.clone()], options.clone())
            .unwrap();
        let parallel_results = parallel_pipeline.process_files(&[path], options).unwrap();

        // Results should be consistent between sequential and parallel processing
        assert_eq!(sequential_results.total_files, parallel_results.total_files);
        assert_eq!(
            sequential_results.processed_files,
            parallel_results.processed_files
        );
        assert_eq!(
            sequential_results.files_with_changes,
            parallel_results.files_with_changes
        );
        assert_eq!(
            sequential_results.failed_files,
            parallel_results.failed_files
        );

        // Sort results by file path for comparison
        let mut seq_results = sequential_results.results;
        let mut par_results = parallel_results.results;
        seq_results.sort_by(|a, b| a.file_path.cmp(&b.file_path));
        par_results.sort_by(|a, b| a.file_path.cmp(&b.file_path));

        // Compare individual file results
        assert_eq!(seq_results.len(), par_results.len());
        for (seq_result, par_result) in seq_results.iter().zip(par_results.iter()) {
            assert_eq!(seq_result.file_path, par_result.file_path);
            assert_eq!(seq_result.success, par_result.success);
            assert_eq!(seq_result.changes_made, par_result.changes_made);
            // Content should be identical
            assert_eq!(seq_result.processed_content, par_result.processed_content);
        }
    }

    #[test]
    fn test_processing_mode_setting() {
        let config = FileDiscoveryConfig::default();
        let mut pipeline = FileProcessingPipeline::new(config).unwrap();

        // Default should be parallel
        assert!(matches!(
            pipeline.processing_mode(),
            ProcessingMode::Parallel
        ));

        // Test changing mode
        pipeline.set_processing_mode(ProcessingMode::Sequential);
        assert!(matches!(
            pipeline.processing_mode(),
            ProcessingMode::Sequential
        ));

        pipeline.set_processing_mode(ProcessingMode::ParallelWithThreads(8));
        assert!(matches!(
            pipeline.processing_mode(),
            ProcessingMode::ParallelWithThreads(8)
        ));
    }

    #[test]
    fn test_large_number_of_files_parallel() {
        let temp_dir = TempDir::new().unwrap();
        let base = temp_dir.path();

        // Create many files to test parallel processing performance
        fs::create_dir_all(base.join("src")).unwrap();

        // Create 20 test files with different class patterns
        for i in 0..20 {
            let content = format!(
                r#"
export function Component{}() {{
  return <div className="p-{} flex m-{} items-center bg-blue-{} text-white">
    Test Component {}
  </div>;
}}
"#,
                i,
                i % 8 + 1,
                i % 4 + 1,
                i % 10 + 100,
                i
            );
            fs::write(base.join(format!("src/Component{}.tsx", i)), content).unwrap();
        }

        let config = FileDiscoveryConfig::default();
        let pipeline = FileProcessingPipeline::parallel(config).unwrap();

        let options = ProcessOptions {
            dry_run: true,
            write: false,
            check_formatted: false,
        };

        let results = pipeline
            .process_files(&[temp_dir.path().to_string_lossy().to_string()], options)
            .unwrap();

        // Should process all 20 files
        assert_eq!(results.total_files, 20);
        assert_eq!(results.processed_files, 20);
        assert_eq!(results.failed_files, 0);
        assert!(results.success_rate() == 1.0);

        // All files should have changes (classes need sorting)
        assert!(results.files_with_changes > 0);
    }

    #[test]
    fn test_parallel_error_handling() {
        let temp_dir = TempDir::new().unwrap();
        let base = temp_dir.path();

        fs::create_dir_all(base.join("src")).unwrap();

        // Create a valid file
        fs::write(
            base.join("src/Valid.tsx"),
            r#"
export function Valid() {
  return <div className="p-4 flex m-2">Valid</div>;
}
"#,
        )
        .unwrap();

        // Create a file with syntax errors that will fail parsing
        fs::write(
            base.join("src/Invalid.tsx"),
            r#"
export function Invalid() {
  return <div className="p-4 flex m-2">Invalid</>; // Missing closing tag
}
"#,
        )
        .unwrap();

        let config = FileDiscoveryConfig::default();
        let pipeline = FileProcessingPipeline::parallel(config).unwrap();

        let options = ProcessOptions {
            dry_run: true,
            write: false,
            check_formatted: false,
        };

        let results = pipeline
            .process_files(&[temp_dir.path().to_string_lossy().to_string()], options)
            .unwrap();

        // Should process both files, but one should fail
        assert_eq!(results.total_files, 2);
        assert_eq!(results.processed_files, 1); // Only the valid file
        assert_eq!(results.failed_files, 1); // The invalid file

        // Check that we have both success and failure results
        let successes = results.results.iter().filter(|r| r.success).count();
        let failures = results.results.iter().filter(|r| !r.success).count();
        assert_eq!(successes, 1);
        assert_eq!(failures, 1);
    }
}
