use clap::Parser;
use std::process;
use std::time::Instant;
use windwarden::cli::{Cli, Commands, OperationMode, ProcessingMode};
use windwarden::file_processor::{FileDiscoveryConfig, FileProcessingPipeline, FileDiscovery};
use windwarden::output::{OutputFormatter, ProgressReporter, ProgressTracker};
use windwarden::{process_file, process_stdin, ProcessOptions, WindWardenError};

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Some(Commands::Process { file, dry_run, write }) => {
            // Legacy single file processing
            let options = ProcessOptions {
                dry_run: *dry_run,
                write: *write,
                check_formatted: false,
            };
            match process_file(file, options) {
                Ok(output) => {
                    if !output.is_empty() {
                        println!("{}", output);
                    }
                    Ok(0)
                }
                Err(e) => {
                    eprintln!("{}", e.user_message());
                    Ok(1)
                }
            }
        }
        
        Some(Commands::Format { 
            paths, 
            mode, 
            processing, 
            threads, 
            extensions, 
            exclude, 
            max_depth, 
            follow_links, 
            stats,
            progress,
            diff
        }) => {
            handle_format_command(paths, *mode, *processing, *threads, extensions, exclude, *max_depth, *follow_links, *stats, *progress, *diff)
        }
        
        Some(Commands::Check { 
            paths, 
            processing, 
            threads, 
            extensions, 
            exclude, 
            stats,
            progress,
            diff
        }) => {
            handle_check_command(paths, *processing, *threads, extensions, exclude, *stats, *progress, *diff)
        }
        
        None => {
            if cli.stdin {
                // Legacy stdin processing
                let options = ProcessOptions {
                    dry_run: cli.dry_run,
                    write: false, // stdin always outputs to stdout
                    check_formatted: cli.check_formatted,
                };
                match process_stdin(options) {
                    Ok(output) => {
                        if !output.is_empty() {
                            println!("{}", output);
                        }
                        Ok(0)
                    }
                    Err(e) => {
                        eprintln!("{}", e.user_message());
                        Ok(1)
                    }
                }
            } else {
                eprintln!("Error: Must specify a command, file, or use --stdin");
                eprintln!("Try 'windwarden --help' for more information.");
                Ok(1)
            }
        }
    };

    match result {
        Ok(exit_code) => process::exit(exit_code),
        Err(e) => {
            // Try to downcast to WindWardenError to get user-friendly message
            if let Some(ww_error) = e.downcast_ref::<WindWardenError>() {
                eprintln!("{}", ww_error.user_message());
            } else {
                eprintln!("Error: {}", e);
            }
            process::exit(1);
        }
    }
}

fn handle_format_command(
    paths: &[String],
    mode: OperationMode,
    processing_mode: ProcessingMode,
    threads: Option<usize>,
    extensions: &Option<Vec<String>>,
    exclude: &Option<Vec<String>>,
    max_depth: Option<usize>,
    follow_links: bool,
    show_stats: bool,
    show_progress: bool,
    show_diff: bool,
) -> Result<i32, Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    
    // Build file discovery config
    let mut config = FileDiscoveryConfig::default();
    
    if let Some(exts) = extensions {
        config.extensions = exts.clone();
    }
    
    if let Some(patterns) = exclude {
        config.exclude_patterns.extend(patterns.clone());
    }
    
    config.max_depth = max_depth;
    config.follow_links = follow_links;
    
    // Create processing pipeline
    let pipeline_mode = match (processing_mode, threads) {
        (_, Some(n)) => windwarden::file_processor::ProcessingMode::ParallelWithThreads(n),
        (ProcessingMode::Sequential, None) => windwarden::file_processor::ProcessingMode::Sequential,
        (ProcessingMode::Parallel, None) => windwarden::file_processor::ProcessingMode::Parallel,
    };
    
    let pipeline = FileProcessingPipeline::new_with_mode(config.clone(), pipeline_mode)?;
    
    // Validate inputs
    if paths.is_empty() {
        return Err(Box::new(WindWardenError::config_error("No paths specified")));
    }
    
    if let Some(thread_count) = threads {
        if thread_count == 0 {
            return Err(Box::new(WindWardenError::config_error("Thread count must be greater than 0")));
        }
        if thread_count > 1024 {
            return Err(Box::new(WindWardenError::config_error("Thread count cannot exceed 1024")));
        }
    }
    
    // Set up process options based on operation mode
    let process_options = match mode {
        OperationMode::Check => ProcessOptions {
            dry_run: true,
            write: false,
            check_formatted: false,
        },
        OperationMode::Write => ProcessOptions {
            dry_run: false,
            write: true,
            check_formatted: false,
        },
        OperationMode::Verify => ProcessOptions {
            dry_run: false,
            write: false,
            check_formatted: true,
        },
    };
    
    // Set up progress reporting if requested
    let (results, duration) = if show_progress {
        // First discover files to get count for progress reporting
        let discovered_files = {
            let temp_discovery = FileDiscovery::new(config.clone())?;
            temp_discovery.discover_files(paths)?
        };
        
        if discovered_files.len() > 5 {
            // Show progress for larger file counts
            let progress_reporter = ProgressReporter::new(discovered_files.len(), true);
            let progress_tracker = ProgressTracker::new(progress_reporter.get_counter());
            
            eprintln!("Processing {} files...", discovered_files.len());
            
            let results = pipeline.process_files_with_progress(paths, process_options, Some(progress_tracker))?;
            let duration = start_time.elapsed();
            
            // Show final progress
            progress_reporter.finish();
            
            (results, duration)
        } else {
            let results = pipeline.process_files(paths, process_options)?;
            let duration = start_time.elapsed();
            (results, duration)
        }
    } else {
        let results = pipeline.process_files(paths, process_options)?;
        let duration = start_time.elapsed();
        (results, duration)
    };
    
    // Format and display results
    let formatter = OutputFormatter::new(show_stats).with_diff(show_diff);
    let output = match mode {
        OperationMode::Check => formatter.format_check_results(&results, Some(duration)),
        OperationMode::Write => formatter.format_write_results(&results, Some(duration)),
        OperationMode::Verify => formatter.format_verify_results(&results, Some(duration)),
    };
    
    println!("{}", output);
    
    Ok(formatter.get_exit_code(&mode, &results))
}

fn handle_check_command(
    paths: &[String],
    processing_mode: ProcessingMode,
    threads: Option<usize>,
    extensions: &Option<Vec<String>>,
    exclude: &Option<Vec<String>>,
    show_stats: bool,
    show_progress: bool,
    show_diff: bool,
) -> Result<i32, Box<dyn std::error::Error>> {
    // Check command is equivalent to format with verify mode
    handle_format_command(
        paths,
        OperationMode::Verify,
        processing_mode,
        threads,
        extensions,
        exclude,
        None, // max_depth
        false, // follow_links
        show_stats,
        show_progress,
        show_diff,
    )
}