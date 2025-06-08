#[path = "cli/mod.rs"]
pub mod cli;
pub mod atomic;
pub mod config;
pub mod diff;
pub mod file_processor;
pub mod output;
pub mod parser;
pub mod processor;
pub mod sorter;
pub mod utils;

#[cfg(feature = "performance-profiling")]
pub mod performance_utils;

pub mod optimizations;

use crate::parser::ClassExtractor;
use crate::processor::FileProcessor;
use crate::sorter::TailwindSorter;
use crate::file_processor::{FileProcessingPipeline, FileDiscoveryConfig, BatchProcessingResults, ProcessingMode};
use crate::config::Config;
use std::io::{self, Read};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WindWardenError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("Permission denied accessing: {path}")]
    PermissionDenied { path: String },
    
    #[error("File is not readable: {path}")]
    FileNotReadable { path: String },
    
    #[error("File is not writable: {path}")]
    FileNotWritable { path: String },
    
    #[error("Parse error in {file} at line {line}: {message}")]
    ParseError { 
        file: String, 
        line: usize, 
        message: String 
    },
    
    #[error("Unsupported file type: {extension} (supported: {supported})")]
    UnsupportedFileType { 
        extension: String, 
        supported: String 
    },
    
    #[error("Sort error in {context}: {message}")]
    SortError { 
        context: String, 
        message: String 
    },
    
    #[error("Configuration error: {message}")]
    Config { message: String },
    
    #[error("Thread pool error: {message}")]
    ThreadPool { message: String },
    
    #[error("Glob pattern error: {pattern} - {message}")]
    GlobPattern { 
        pattern: String, 
        message: String 
    },
    
    #[error("Processing failed for {file_count} files: {summary}")]
    BatchProcessing { 
        file_count: usize, 
        summary: String 
    },
    
    #[error("Invalid UTF-8 in file: {path}")]
    InvalidUtf8 { path: String },
    
    #[error("File operation cancelled")]
    Cancelled,
    
    #[error("Internal error: {message}")]
    Internal { message: String },
}

impl WindWardenError {
    /// Create a file not found error
    pub fn file_not_found(path: impl Into<String>) -> Self {
        Self::FileNotFound { path: path.into() }
    }
    
    /// Create a permission denied error
    pub fn permission_denied(path: impl Into<String>) -> Self {
        Self::PermissionDenied { path: path.into() }
    }
    
    /// Create a parse error with context
    pub fn parse_error(file: impl Into<String>, line: usize, message: impl Into<String>) -> Self {
        Self::ParseError { 
            file: file.into(), 
            line, 
            message: message.into() 
        }
    }
    
    /// Create a sort error with context
    pub fn sort_error(context: impl Into<String>, message: impl Into<String>) -> Self {
        Self::SortError { 
            context: context.into(), 
            message: message.into() 
        }
    }
    
    /// Create a configuration error
    pub fn config_error(message: impl Into<String>) -> Self {
        Self::Config { message: message.into() }
    }
    
    /// Create a thread pool error
    pub fn thread_pool_error(message: impl Into<String>) -> Self {
        Self::ThreadPool { message: message.into() }
    }
    
    /// Create a glob pattern error
    pub fn glob_pattern_error(pattern: impl Into<String>, message: impl Into<String>) -> Self {
        Self::GlobPattern { 
            pattern: pattern.into(), 
            message: message.into() 
        }
    }
    
    /// Create an invalid UTF-8 error
    pub fn invalid_utf8(path: impl Into<String>) -> Self {
        Self::InvalidUtf8 { path: path.into() }
    }
    
    /// Create an internal error
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::Internal { message: message.into() }
    }
    
    /// Check if this error is recoverable (processing can continue)
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::ParseError { .. } |
            Self::UnsupportedFileType { .. } |
            Self::SortError { .. } |
            Self::InvalidUtf8 { .. } => true,
            
            Self::FileNotFound { .. } |
            Self::PermissionDenied { .. } |
            Self::FileNotReadable { .. } |
            Self::FileNotWritable { .. } => true,
            
            Self::Config { .. } |
            Self::ThreadPool { .. } |
            Self::GlobPattern { .. } |
            Self::BatchProcessing { .. } |
            Self::Cancelled |
            Self::Internal { .. } |
            Self::Io(_) => false,
        }
    }
    
    /// Get a user-friendly error message with helpful suggestions
    pub fn user_message(&self) -> String {
        match self {
            Self::FileNotFound { path } => {
                format!("❌ File not found: {}\n\n💡 Suggestions:\n   • Check that the file path is correct\n   • Ensure the file exists\n   • Try using an absolute path", path)
            }
            Self::PermissionDenied { path } => {
                format!("❌ Permission denied: {}\n\n💡 Suggestions:\n   • Check file permissions with 'ls -la {}'\n   • Run with appropriate privileges (sudo)\n   • Ensure you have write access to the directory", path, path)
            }
            Self::ParseError { file, line, message } => {
                format!("❌ Parse error in {} at line {}: {}\n\n💡 Suggestions:\n   • Check the syntax around line {}\n   • Ensure proper quote matching\n   • Verify JSX/TSX syntax is valid", file, line, message, line)
            }
            Self::UnsupportedFileType { extension, supported } => {
                format!("❌ Unsupported file type: .{}\n   Supported extensions: {}\n\n💡 Suggestions:\n   • Use --extensions {} to include this file type\n   • Add extension to .windwarden.json configuration\n   • Check if the file is actually a supported format", extension, supported, extension)
            }
            Self::Config { message } => {
                format!("❌ Configuration error: {}\n\n💡 Suggestions:\n   • Run 'windwarden config validate' to check your config\n   • Use 'windwarden config init' to create a default config\n   • Check command line arguments syntax", message)
            }
            Self::ThreadPool { message } => {
                format!("❌ Threading error: {}\n\n💡 Suggestions:\n   • Try --processing sequential for single-threaded processing\n   • Reduce thread count with --threads 1\n   • Check system resource availability", message)
            }
            Self::GlobPattern { pattern, message } => {
                format!("❌ Invalid glob pattern '{}': {}\n\n💡 Suggestions:\n   • Check glob syntax (use * for wildcards, ** for recursive)\n   • Escape special characters if needed\n   • Examples: 'src/**/*.tsx', '*.{{js,ts}}'", pattern, message)
            }
            Self::BatchProcessing { file_count, summary } => {
                format!("❌ Processing failed for {} files: {}\n\n💡 Suggestions:\n   • Check individual file errors above\n   • Try processing files one by one to isolate issues\n   • Use --stats to see detailed information", file_count, summary)
            }
            Self::InvalidUtf8 { path } => {
                format!("❌ Invalid UTF-8 encoding in file: {}\n\n💡 Suggestions:\n   • Check file encoding and convert to UTF-8\n   • Use a text editor to fix encoding issues\n   • Skip this file with --exclude pattern", path)
            }
            _ => format!("❌ Error: {}\n\n💡 For help, run: windwarden --help", self.to_string()),
        }
    }
    
    /// Convert an IO error to a more specific WindWardenError based on error kind
    pub fn from_io_error(err: io::Error, path: Option<&str>) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => {
                if let Some(path) = path {
                    Self::file_not_found(path)
                } else {
                    Self::Io(err)
                }
            }
            io::ErrorKind::PermissionDenied => {
                if let Some(path) = path {
                    Self::permission_denied(path)
                } else {
                    Self::Io(err)
                }
            }
            io::ErrorKind::InvalidData => {
                if let Some(path) = path {
                    Self::invalid_utf8(path)
                } else {
                    Self::Io(err)
                }
            }
            _ => Self::Io(err),
        }
    }
}

pub type Result<T> = std::result::Result<T, WindWardenError>;

#[derive(Debug, Clone)]
pub struct ProcessOptions {
    pub dry_run: bool,
    pub write: bool,
    pub check_formatted: bool,
}

impl Default for ProcessOptions {
    fn default() -> Self {
        Self {
            dry_run: false,
            write: false,
            check_formatted: false,
        }
    }
}

pub fn process_file(file_path: &str, options: ProcessOptions) -> Result<String> {
    let processor = FileProcessor::new();
    processor.process_file(file_path, options)
}

pub fn process_file_with_config(file_path: &str, options: ProcessOptions, config: &crate::config::Config) -> Result<String> {
    let processor = FileProcessor::new_with_config(config);
    processor.process_file(file_path, options)
}

pub fn process_stdin(options: ProcessOptions) -> Result<String> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let processor = FileProcessor::new();
    processor.process_content(&input, "stdin.tsx", options)
}

pub fn process_stdin_with_config(options: ProcessOptions, config: &crate::config::Config) -> Result<String> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let processor = FileProcessor::new_with_config(config);
    processor.process_content(&input, "stdin.tsx", options)
}

pub fn process_file_content(content: &str, file_path: &str) -> Result<String> {
    let processor = FileProcessor::new();
    processor.process_content(content, file_path, ProcessOptions::default())
}

/// Process multiple files or directories using the file processing pipeline
pub fn process_files(paths: &[String], options: ProcessOptions) -> Result<BatchProcessingResults> {
    let config = FileDiscoveryConfig::default();
    let pipeline = FileProcessingPipeline::new(config)?;
    pipeline.process_files(paths, options)
}

/// Process multiple files with custom discovery configuration
pub fn process_files_with_config(
    paths: &[String], 
    options: ProcessOptions, 
    config: FileDiscoveryConfig
) -> Result<BatchProcessingResults> {
    let pipeline = FileProcessingPipeline::new(config)?;
    pipeline.process_files(paths, options)
}

/// Process multiple files with custom configuration and processing mode
pub fn process_files_with_mode(
    paths: &[String], 
    options: ProcessOptions, 
    config: FileDiscoveryConfig,
    mode: ProcessingMode
) -> Result<BatchProcessingResults> {
    let pipeline = FileProcessingPipeline::new_with_mode(config, mode)?;
    pipeline.process_files(paths, options)
}

/// Process multiple files with WindWarden configuration
pub fn process_files_with_windwarden_config(
    paths: &[String], 
    options: ProcessOptions, 
    file_config: FileDiscoveryConfig,
    windwarden_config: &Config,
    mode: ProcessingMode
) -> Result<BatchProcessingResults> {
    let pipeline = FileProcessingPipeline::new_with_windwarden_config(file_config, windwarden_config, mode)?;
    pipeline.process_files(paths, options)
}

/// Process multiple files sequentially (single-threaded)
pub fn process_files_sequential(paths: &[String], options: ProcessOptions) -> Result<BatchProcessingResults> {
    let config = FileDiscoveryConfig::default();
    let pipeline = FileProcessingPipeline::sequential(config)?;
    pipeline.process_files(paths, options)
}

/// Process multiple files in parallel using all available CPU cores
pub fn process_files_parallel(paths: &[String], options: ProcessOptions) -> Result<BatchProcessingResults> {
    let config = FileDiscoveryConfig::default();
    let pipeline = FileProcessingPipeline::parallel(config)?;
    pipeline.process_files(paths, options)
}