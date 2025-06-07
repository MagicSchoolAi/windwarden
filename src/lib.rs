pub mod cli;
pub mod parser;
pub mod processor;
pub mod sorter;
pub mod utils;

use crate::parser::ClassExtractor;
use crate::processor::FileProcessor;
use crate::sorter::TailwindSorter;
use std::io::{self, Read};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WindWardenError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Sort error: {0}")]
    Sort(String),
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

pub fn process_stdin(options: ProcessOptions) -> Result<String> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let processor = FileProcessor::new();
    processor.process_content(&input, "stdin.tsx", options)
}

pub fn process_file_content(content: &str, file_path: &str) -> Result<String> {
    let processor = FileProcessor::new();
    processor.process_content(content, file_path, ProcessOptions::default())
}