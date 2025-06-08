use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "windwarden")]
#[command(about = "🌪️  High-performance CLI tool for sorting Tailwind CSS classes")]
#[command(
    long_about = "WindWarden is a blazing fast CLI tool that automatically sorts Tailwind CSS classes according to the official recommended order. It uses AST parsing to handle JSX attributes, utility functions like cn() and clsx(), template literals, and arrays."
)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(
    after_help = "Examples:\n  windwarden format src/                    # Check files in src/ directory\n  windwarden format --mode write src/      # Format files in place\n  windwarden format --mode verify src/     # Exit 1 if any files need formatting\n  windwarden config init                   # Create default configuration\n  windwarden --stdin < file.tsx           # Process from stdin\n\nFor more information, visit: https://github.com/your-org/windwarden"
)]
pub struct Cli {
    /// Process input from stdin instead of files
    #[arg(long, help = "Read code from stdin and output to stdout")]
    pub stdin: bool,


    /// Configuration file path (searches for .windwarden.json by default)
    #[arg(short, long, help = "Path to configuration file", value_name = "FILE")]
    pub config: Option<PathBuf>,

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
    /// 🎨 Format Tailwind CSS classes in files and directories  
    #[command(
        after_help = "Examples:\n  windwarden format src/                         # Preview changes in src/\n  windwarden format --mode write src/ tests/    # Format multiple directories\n  windwarden format --mode verify .             # Check if project is formatted\n  windwarden format --extensions tsx,jsx src/   # Process only specific file types"
    )]
    Format {
        /// Files, directories, or glob patterns to process
        #[arg(
            required = true,
            help = "Paths to files, directories, or glob patterns (e.g., 'src/**/*.tsx')",
            value_name = "PATH"
        )]
        paths: Vec<String>,

        /// What to do with the files
        #[arg(short, long, value_enum, default_value_t = OperationMode::Check, help = "Operation to perform")]
        mode: OperationMode,

        /// How to process files
        #[arg(short = 'j', long, value_enum, default_value_t = ProcessingMode::Parallel, help = "Process files sequentially or in parallel")]
        processing: ProcessingMode,

        /// Number of threads for parallel processing
        #[arg(
            long,
            help = "Number of threads to use (overrides --processing)",
            value_name = "N"
        )]
        threads: Option<usize>,

        /// File extensions to include
        #[arg(
            long,
            value_delimiter = ',',
            help = "Comma-separated list of file extensions",
            value_name = "EXT"
        )]
        extensions: Option<Vec<String>>,

        /// Patterns to exclude from processing
        #[arg(
            long,
            value_delimiter = ',',
            help = "Comma-separated glob patterns to exclude",
            value_name = "PATTERN"
        )]
        exclude: Option<Vec<String>>,

        /// Maximum directory traversal depth
        #[arg(
            long,
            help = "Maximum depth when traversing directories",
            value_name = "DEPTH"
        )]
        max_depth: Option<usize>,

        /// Follow symbolic links during traversal
        #[arg(long, help = "Follow symbolic links when traversing directories")]
        follow_links: bool,

        /// Show detailed processing statistics
        #[arg(long, help = "Display detailed statistics about processed files")]
        stats: bool,

        /// Show progress bar for large operations
        #[arg(long, help = "Display progress bar when processing many files")]
        progress: bool,

        /// Show diff of changes that would be made
        #[arg(long, help = "Show a diff of the changes that would be made")]
        diff: bool,
    },

    /// ✅ Check if files are properly formatted (alias for 'format --mode verify')
    #[command(
        after_help = "Examples:\n  windwarden check src/           # Check if files in src/ are formatted\n  windwarden check .              # Check entire project\n  windwarden check --diff src/    # Show what changes would be needed"
    )]
    Check {
        /// Files, directories, or glob patterns to check
        #[arg(
            required = true,
            help = "Paths to check for proper formatting",
            value_name = "PATH"
        )]
        paths: Vec<String>,

        /// How to process files
        #[arg(short = 'j', long, value_enum, default_value_t = ProcessingMode::Parallel, help = "Process files sequentially or in parallel")]
        processing: ProcessingMode,

        /// Number of threads for parallel processing
        #[arg(long, help = "Number of threads to use", value_name = "N")]
        threads: Option<usize>,

        /// File extensions to include
        #[arg(
            long,
            value_delimiter = ',',
            help = "Comma-separated list of file extensions",
            value_name = "EXT"
        )]
        extensions: Option<Vec<String>>,

        /// Patterns to exclude from checking
        #[arg(
            long,
            value_delimiter = ',',
            help = "Comma-separated glob patterns to exclude",
            value_name = "PATTERN"
        )]
        exclude: Option<Vec<String>>,

        /// Show detailed checking statistics
        #[arg(long, help = "Display detailed statistics about checked files")]
        stats: bool,

        /// Show progress bar for large operations
        #[arg(long, help = "Display progress bar when checking many files")]
        progress: bool,

        /// Show diff of changes that would be needed
        #[arg(long, help = "Show a diff of the changes that would be needed")]
        diff: bool,
    },

    /// ⚙️  Configuration file management
    #[command(
        after_help = "Examples:\n  windwarden config init             # Create .windwarden.json in current directory\n  windwarden config show             # Display current configuration\n  windwarden config validate         # Check configuration file syntax"
    )]
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// 🐚 Generate shell completion scripts
    #[command(
        after_help = "Examples:\n  windwarden completions bash > /etc/bash_completion.d/windwarden\n  windwarden completions zsh > ~/.zsh/completions/_windwarden\n  windwarden completions fish > ~/.config/fish/completions/windwarden.fish"
    )]
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// 📝 Create a default configuration file
    Init {
        /// Where to create the configuration file
        #[arg(
            long,
            default_value = ".windwarden.json",
            help = "Path for the new configuration file",
            value_name = "FILE"
        )]
        path: PathBuf,
    },

    /// 📋 Show current effective configuration
    Show,

    /// ✅ Validate configuration file syntax and settings
    Validate {
        /// Configuration file to validate (searches for default if not provided)
        #[arg(help = "Path to configuration file", value_name = "FILE")]
        path: Option<PathBuf>,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Shell {
    /// Bash shell completions
    Bash,
    /// Zsh shell completions
    Zsh,
    /// Fish shell completions
    Fish,
    /// PowerShell completions
    PowerShell,
}
