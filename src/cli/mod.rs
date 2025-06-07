use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "windwarden")]
#[command(about = "High-performance CLI tool for sorting Tailwind CSS classes")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// Process from stdin
    #[arg(long)]
    pub stdin: bool,

    /// Preview changes without writing
    #[arg(long)]
    pub dry_run: bool,

    /// Check if files are formatted (exit code 0 if formatted)
    #[arg(long)]
    pub check_formatted: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Process a specific file
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
}