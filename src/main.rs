use clap::{CommandFactory, Parser};
use std::io;
use std::process;
use std::time::Instant;
use windwarden::cli::{Cli, Commands, ConfigAction, OperationMode, ProcessingMode, Shell};
use windwarden::config::ConfigManager;
use windwarden::file_processor::{FileDiscovery, FileDiscoveryConfig, FileProcessingPipeline};
use windwarden::output::{OutputFormatter, ProgressReporter, ProgressTracker};
use windwarden::{ProcessOptions, WindWardenError, process_stdin};

#[derive(Debug, Clone)]
struct CommandOptions {
    processing_mode: ProcessingMode,
    threads: Option<usize>,
    extensions: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
    max_depth: Option<usize>,
    follow_links: bool,
    show_stats: bool,
    show_progress: bool,
    show_diff: bool,
}

fn main() {
    let cli = Cli::parse();

    // Load configuration
    let config_manager = match load_configuration(&cli) {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("{}", e.user_message());
            process::exit(1);
        }
    };

    let result = match &cli.command {
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
            diff,
        }) => {
            let options = CommandOptions {
                processing_mode: *processing,
                threads: *threads,
                extensions: extensions.clone(),
                exclude: exclude.clone(),
                max_depth: *max_depth,
                follow_links: *follow_links,
                show_stats: *stats,
                show_progress: *progress,
                show_diff: *diff,
            };
            handle_format_command(&config_manager, paths, *mode, &options)
        }

        Some(Commands::Check {
            paths,
            processing,
            threads,
            extensions,
            exclude,
            stats,
            progress,
            diff,
        }) => {
            let options = CommandOptions {
                processing_mode: *processing,
                threads: *threads,
                extensions: extensions.clone(),
                exclude: exclude.clone(),
                max_depth: None,
                follow_links: false,
                show_stats: *stats,
                show_progress: *progress,
                show_diff: *diff,
            };
            handle_check_command(&config_manager, paths, &options)
        }

        Some(Commands::Config { action }) => handle_config_command(action, &config_manager),

        Some(Commands::Completions { shell }) => handle_completions_command(*shell),

        None => {
            if cli.stdin {
                // Stdin processing
                let options = ProcessOptions {
                    dry_run: false,
                    write: false, // stdin always outputs to stdout
                    check_formatted: false,
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
                eprintln!("Error: Must specify a command or use --stdin");
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
    config_manager: &ConfigManager,
    paths: &[String],
    mode: OperationMode,
    options: &CommandOptions,
) -> Result<i32, Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    // Build file discovery config
    let mut config = FileDiscoveryConfig::default();

    if let Some(exts) = &options.extensions {
        config.extensions = exts.clone();
    }

    if let Some(patterns) = &options.exclude {
        config.exclude_patterns.extend(patterns.clone());
    }

    // Add ignore patterns from JSON configuration
    config.exclude_patterns.extend(
        config_manager
            .get_ignore_patterns()
            .iter()
            .map(|p| format!("{}/**", p)), // Convert directory names to glob patterns
    );

    config.max_depth = options.max_depth;
    config.follow_links = options.follow_links;

    // Create processing pipeline
    let pipeline_mode = match (options.processing_mode, options.threads) {
        (_, Some(n)) => windwarden::file_processor::ProcessingMode::ParallelWithThreads(n),
        (ProcessingMode::Sequential, None) => {
            windwarden::file_processor::ProcessingMode::Sequential
        }
        (ProcessingMode::Parallel, None) => windwarden::file_processor::ProcessingMode::Parallel,
    };

    let pipeline = FileProcessingPipeline::new_with_windwarden_config(
        config.clone(),
        config_manager.config(),
        pipeline_mode,
    )?;

    // Validate inputs
    if paths.is_empty() {
        return Err(Box::new(WindWardenError::config_error(
            "No paths specified",
        )));
    }

    if let Some(thread_count) = options.threads {
        if thread_count == 0 {
            return Err(Box::new(WindWardenError::config_error(
                "Thread count must be greater than 0",
            )));
        }
        if thread_count > 1024 {
            return Err(Box::new(WindWardenError::config_error(
                "Thread count cannot exceed 1024",
            )));
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
    let (results, duration) = if options.show_progress {
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

            let results = pipeline.process_files_with_progress(
                paths,
                process_options,
                Some(progress_tracker),
            )?;
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
    let formatter = OutputFormatter::new(options.show_stats).with_diff(options.show_diff);
    let output = match mode {
        OperationMode::Check => formatter.format_check_results(&results, Some(duration)),
        OperationMode::Write => formatter.format_write_results(&results, Some(duration)),
        OperationMode::Verify => formatter.format_verify_results(&results, Some(duration)),
    };

    println!("{}", output);

    Ok(formatter.get_exit_code(&mode, &results))
}

fn handle_check_command(
    config_manager: &ConfigManager,
    paths: &[String],
    options: &CommandOptions,
) -> Result<i32, Box<dyn std::error::Error>> {
    // Check command is equivalent to format with verify mode
    let check_options = CommandOptions {
        max_depth: None,     // max_depth not used in check
        follow_links: false, // follow_links not used in check
        ..options.clone()
    };

    handle_format_command(config_manager, paths, OperationMode::Verify, &check_options)
}

fn load_configuration(cli: &Cli) -> Result<ConfigManager, WindWardenError> {
    match &cli.config {
        Some(config_path) => {
            // Load from specific path
            if !config_path.exists() {
                return Err(WindWardenError::config_error(format!(
                    "Configuration file not found: {}",
                    config_path.display()
                )));
            }
            let config = ConfigManager::load_config_file(config_path)?;
            let manager = ConfigManager::new_with_config(config, Some(config_path.clone()));
            Ok(manager)
        }
        None => {
            // Search for config file in current directory and parents
            let current_dir =
                std::env::current_dir().map_err(|e| WindWardenError::from_io_error(e, None))?;
            ConfigManager::load_from_directory(&current_dir)
        }
    }
}

fn handle_config_command(
    action: &ConfigAction,
    config_manager: &ConfigManager,
) -> Result<i32, Box<dyn std::error::Error>> {
    match action {
        ConfigAction::Init { path } => {
            if path.exists() {
                eprintln!("Configuration file already exists: {}", path.display());
                eprintln!("Use --force to overwrite (not implemented yet)");
                return Ok(1);
            }

            ConfigManager::create_default_config(path)?;
            println!("Created default configuration file: {}", path.display());
            println!("\nTo customize your configuration, edit the file and modify settings like:");
            println!("  - sortOrder: \"official\" or \"custom\"");
            println!("  - customOrder: [\"layout\", \"flexbox-grid\", \"spacing\", ...]");
            println!("  - functionNames: [\"cn\", \"clsx\", \"yourCustomFunction\"]");
            println!("  - ignorePaths: [\"node_modules\", \"dist\"]");
            println!("  - fileExtensions: [\"tsx\", \"jsx\", \"ts\", \"js\"]");
            println!("\nAvailable categories for customOrder: ");
            let categories = ConfigManager::get_available_categories();
            println!("  [{}]", categories.join(", "));
            Ok(0)
        }

        ConfigAction::Show => {
            let config = config_manager.config();
            let json = serde_json::to_string_pretty(config).map_err(|e| {
                WindWardenError::config_error(format!("Failed to serialize config: {}", e))
            })?;

            println!("Current configuration:");
            if let Some(path) = config_manager.config_path() {
                println!("Loaded from: {}", path.display());
            } else {
                println!("Using default configuration (no config file found)");
            }
            println!("\n{}", json);
            Ok(0)
        }

        ConfigAction::Validate { path } => {
            let config_path = match path {
                Some(p) => p.clone(),
                None => {
                    if let Some(p) = config_manager.config_path() {
                        p.clone()
                    } else {
                        return Err(Box::new(WindWardenError::config_error(
                            "No configuration file specified and none found",
                        )));
                    }
                }
            };

            match ConfigManager::load_config_file(&config_path) {
                Ok(_) => {
                    println!("✓ Configuration file is valid: {}", config_path.display());
                    Ok(0)
                }
                Err(e) => {
                    eprintln!("✗ Configuration file is invalid: {}", config_path.display());
                    eprintln!("{}", e.user_message());
                    Ok(1)
                }
            }
        }
    }
}

fn handle_completions_command(shell: Shell) -> Result<i32, Box<dyn std::error::Error>> {
    let mut cmd = Cli::command();
    let app_name = cmd.get_name().to_string();

    match shell {
        Shell::Bash => {
            clap_complete::generate(
                clap_complete::shells::Bash,
                &mut cmd,
                app_name,
                &mut io::stdout(),
            );
        }
        Shell::Zsh => {
            clap_complete::generate(
                clap_complete::shells::Zsh,
                &mut cmd,
                app_name,
                &mut io::stdout(),
            );
        }
        Shell::Fish => {
            clap_complete::generate(
                clap_complete::shells::Fish,
                &mut cmd,
                app_name,
                &mut io::stdout(),
            );
        }
        Shell::PowerShell => {
            clap_complete::generate(
                clap_complete::shells::PowerShell,
                &mut cmd,
                app_name,
                &mut io::stdout(),
            );
        }
    }

    Ok(0)
}
