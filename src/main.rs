use clap::Parser;
use std::process;
use windwarden::cli::{Cli, Commands};
use windwarden::{process_file, process_stdin, ProcessOptions};

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Some(Commands::Process { file, dry_run, write }) => {
            let options = ProcessOptions {
                dry_run: *dry_run,
                write: *write,
                check_formatted: false,
            };
            process_file(file, options)
        }
        None => {
            if cli.stdin {
                let options = ProcessOptions {
                    dry_run: cli.dry_run,
                    write: false, // stdin always outputs to stdout
                    check_formatted: cli.check_formatted,
                };
                process_stdin(options)
            } else {
                eprintln!("Error: Must specify a file or use --stdin");
                process::exit(1);
            }
        }
    };

    match result {
        Ok(output) => {
            if !output.is_empty() {
                println!("{}", output);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}