use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "windwarden")]
#[command(about = "High-performance CLI tool for sorting Tailwind CSS classes")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// Process from stdin
    #[arg(long)]
    pub stdin: bool,

    /// Preview changes without writing (legacy flag)
    #[arg(long)]
    pub dry_run: bool,

    /// Check if files are formatted (exit code 0 if formatted) (legacy flag)
    #[arg(long)]
    pub check_formatted: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum ProcessingMode {
    /// Process files sequentially (single-threaded)
    Sequential,
    /// Process files in parallel using all available CPU cores
    Parallel,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum OperationMode {
    /// Preview changes without writing to files
    Check,
    /// Write changes directly to files
    Write,
    /// Check if files are already formatted (exit code 1 if not)
    Verify,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Process a specific file (legacy command)
    Process {
        /// File to process
        file: String,

        /// Preview changes without writing
        #[arg(long)]
        dry_run: bool,

        /// Write changes to file
        #[arg(long)]
        write: bool,
    },
    
    /// Format files and directories with advanced options
    Format {
        /// Files, directories, or glob patterns to process
        #[arg(required = true)]
        paths: Vec<String>,
        
        /// Operation mode
        #[arg(short, long, value_enum, default_value_t = OperationMode::Check)]
        mode: OperationMode,
        
        /// Processing mode (sequential vs parallel)
        #[arg(short = 'j', long, value_enum, default_value_t = ProcessingMode::Parallel)]
        processing: ProcessingMode,
        
        /// Number of threads for parallel processing (overrides --processing)
        #[arg(long)]
        threads: Option<usize>,
        
        /// File extensions to process
        #[arg(long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        
        /// Patterns to exclude
        #[arg(long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,
        
        /// Maximum directory depth
        #[arg(long)]
        max_depth: Option<usize>,
        
        /// Follow symbolic links
        #[arg(long)]
        follow_links: bool,
        
        /// Show detailed statistics
        #[arg(long)]
        stats: bool,
        
        /// Show progress bar for large operations
        #[arg(long)]
        progress: bool,
        
        /// Show diff of changes that would be made
        #[arg(long)]
        diff: bool,
    },
    
    /// Check if files are properly formatted
    Check {
        /// Files, directories, or glob patterns to check
        #[arg(required = true)]
        paths: Vec<String>,
        
        /// Processing mode (sequential vs parallel)
        #[arg(short = 'j', long, value_enum, default_value_t = ProcessingMode::Parallel)]
        processing: ProcessingMode,
        
        /// Number of threads for parallel processing
        #[arg(long)]
        threads: Option<usize>,
        
        /// File extensions to process
        #[arg(long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        
        /// Patterns to exclude
        #[arg(long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,
        
        /// Show detailed statistics
        #[arg(long)]
        stats: bool,
        
        /// Show progress bar for large operations
        #[arg(long)]
        progress: bool,
        
        /// Show diff of changes that would be made
        #[arg(long)]
        diff: bool,
    },
}