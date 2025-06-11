use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use std::fs;
use std::path::Path;
use tempfile::TempDir;
use windwarden::file_processor::{FileDiscoveryConfig, FileProcessingPipeline, ProcessingMode};
use windwarden::{ProcessOptions, process_file};

/// Create test files with various complexity levels
fn create_test_files(temp_dir: &Path, count: usize, complexity: &str) -> Vec<String> {
    let mut files = Vec::new();

    for i in 0..count {
        let content = match complexity {
            "simple" => format!(
                r#"import React from 'react';
export const Component{} = () => (
  <div className="p-4 bg-red-500 flex justify-center items-center">
    Simple component {}
  </div>
);"#,
                i, i
            ),
            "medium" => format!(
                r#"import React from 'react';
import {{ cn }} from './utils';

export const Component{} = ({{ isActive, size = 'md' }}) => (
  <div className="w-full h-screen bg-gray-100 flex items-center justify-center p-8">
    <div className="max-w-md mx-auto bg-white rounded-xl shadow-lg p-6">
      <button 
        className={{cn(
          "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50",
          size === 'sm' ? 'text-sm' : size === 'lg' ? 'text-lg' : 'text-base',
          isActive ? 'bg-blue-600 ring-2 ring-blue-300' : 'bg-blue-500'
        )}}
      >
        Button {}
      </button>
    </div>
  </div>
);"#,
                i, i
            ),
            "complex" => format!(
                r#"import React from 'react';
import {{ clsx }} from 'clsx';
import {{ twMerge }} from 'tailwind-merge';

const cn = (...inputs) => twMerge(clsx(inputs));

export const Component{} = ({{ 
  variant = 'primary', 
  size = 'md', 
  disabled = false, 
  loading = false,
  icon,
  children 
}}) => {{
  const baseClasses = "inline-flex items-center justify-center font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background";
  
  const variants = {{
    primary: "bg-primary text-primary-foreground hover:bg-primary/90",
    secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80", 
    destructive: "bg-destructive text-destructive-foreground hover:bg-destructive/90",
    outline: "border border-input hover:bg-accent hover:text-accent-foreground",
    ghost: "hover:bg-accent hover:text-accent-foreground",
    link: "underline-offset-4 hover:underline text-primary"
  }};
  
  const sizes = {{
    sm: "h-9 px-3 rounded-md text-xs",
    md: "h-10 py-2 px-4 rounded-md text-sm", 
    lg: "h-11 px-8 rounded-md",
    icon: "h-10 w-10 rounded-md"
  }};
  
  return (
    <button
      className={{cn(
        baseClasses,
        variants[variant],
        sizes[size],
        disabled && "opacity-50 cursor-not-allowed",
        loading && "cursor-wait",
        "border-2 border-transparent hover:border-gray-300 focus:border-blue-500",
        "shadow-sm hover:shadow-md active:shadow-lg transform hover:scale-105 active:scale-95",
        "transition-all duration-200 ease-in-out"
      )}}
      disabled={{disabled || loading}}
    >
      {{loading && (
        <svg 
          className="animate-spin -ml-1 mr-3 h-5 w-5 text-white" 
          xmlns="http://www.w3.org/2000/svg" 
          fill="none" 
          viewBox="0 0 24 24"
        >
          <circle 
            className="opacity-25" 
            cx="12" 
            cy="12" 
            r="10" 
            stroke="currentColor" 
            strokeWidth="4"
          />
          <path 
            className="opacity-75 fill-current" 
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
          />
        </svg>
      )}}
      {{icon && <span className="mr-2">{{icon}}</span>}}
      {{children}}
    </button>
  );
}};"#,
                i
            ),
            _ => panic!("Unknown complexity level"),
        };

        let file_path = temp_dir.join(format!("component_{}.tsx", i));
        fs::write(&file_path, content).expect("Failed to write test file");
        files.push(file_path.display().to_string());
    }

    files
}

/// Benchmark single file processing
fn bench_single_file_processing(c: &mut Criterion) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let mut group = c.benchmark_group("single_file_processing");

    for complexity in ["simple", "medium", "complex"] {
        let files = create_test_files(temp_dir.path(), 1, complexity);
        let file_path = &files[0];

        let options = ProcessOptions {
            dry_run: true,
            write: false,
            check_formatted: false,
        };

        group.bench_with_input(
            BenchmarkId::new("complexity", complexity),
            file_path,
            |b, file_path| {
                b.iter(|| {
                    process_file(black_box(file_path), black_box(options.clone()))
                        .expect("Failed to process file")
                })
            },
        );
    }

    group.finish();
}

/// Benchmark batch file processing
fn bench_batch_processing(c: &mut Criterion) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let mut group = c.benchmark_group("batch_processing");

    for &file_count in &[10, 50, 100, 200] {
        let files = create_test_files(temp_dir.path(), file_count, "medium");

        group.throughput(Throughput::Elements(file_count as u64));

        // Sequential processing
        group.bench_with_input(
            BenchmarkId::new("sequential", file_count),
            &files,
            |b, _files| {
                b.iter(|| {
                    let config = FileDiscoveryConfig::default();
                    let pipeline = FileProcessingPipeline::sequential(config)
                        .expect("Failed to create pipeline");
                    let options = ProcessOptions {
                        dry_run: true,
                        write: false,
                        check_formatted: false,
                    };

                    pipeline
                        .process_files(
                            black_box(&[temp_dir.path().display().to_string()]),
                            black_box(options),
                        )
                        .expect("Failed to process files")
                })
            },
        );

        // Parallel processing
        group.bench_with_input(
            BenchmarkId::new("parallel", file_count),
            &files,
            |b, _files| {
                b.iter(|| {
                    let config = FileDiscoveryConfig::default();
                    let pipeline = FileProcessingPipeline::parallel(config)
                        .expect("Failed to create pipeline");
                    let options = ProcessOptions {
                        dry_run: true,
                        write: false,
                        check_formatted: false,
                    };

                    pipeline
                        .process_files(
                            black_box(&[temp_dir.path().display().to_string()]),
                            black_box(options),
                        )
                        .expect("Failed to process files")
                })
            },
        );
    }

    group.finish();
}

/// Benchmark parallel processing with different thread counts
fn bench_thread_scaling(c: &mut Criterion) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let _files = create_test_files(temp_dir.path(), 100, "medium");

    let mut group = c.benchmark_group("thread_scaling");

    for &thread_count in &[1, 2, 4, 8, 16] {
        group.bench_with_input(
            BenchmarkId::new("threads", thread_count),
            &thread_count,
            |b, &thread_count| {
                b.iter(|| {
                    let config = FileDiscoveryConfig::default();
                    let mode = ProcessingMode::ParallelWithThreads(thread_count);
                    let pipeline = FileProcessingPipeline::new_with_mode(config, mode)
                        .expect("Failed to create pipeline");
                    let options = ProcessOptions {
                        dry_run: true,
                        write: false,
                        check_formatted: false,
                    };

                    pipeline
                        .process_files(
                            black_box(&[temp_dir.path().display().to_string()]),
                            black_box(options),
                        )
                        .expect("Failed to process files")
                })
            },
        );
    }

    group.finish();
}

/// Benchmark file discovery performance
fn bench_file_discovery(c: &mut Criterion) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let mut group = c.benchmark_group("file_discovery");

    // Create nested directory structure
    for depth in 1..=5 {
        let mut current_path = temp_dir.path().to_path_buf();
        for i in 0..depth {
            current_path.push(format!("level_{}", i));
            fs::create_dir_all(&current_path).expect("Failed to create directory");

            // Add files at each level
            for j in 0..5 {
                let file_path = current_path.join(format!("file_{}_{}.tsx", i, j));
                fs::write(
                    &file_path,
                    "export const Test = () => <div className=\"p-4 m-2\">Test</div>;",
                )
                .expect("Failed to write file");
            }
        }
    }

    group.bench_function("directory_traversal", |b| {
        b.iter(|| {
            let config = FileDiscoveryConfig::default();
            let pipeline =
                FileProcessingPipeline::sequential(config).expect("Failed to create pipeline");

            // Just do file discovery, not processing
            let discovery = pipeline.discovery_config();
            let file_discovery = windwarden::file_processor::FileDiscovery::new(discovery.clone())
                .expect("Failed to create file discovery");

            file_discovery
                .discover_files(black_box(&[temp_dir.path().display().to_string()]))
                .expect("Failed to discover files")
        })
    });

    group.finish();
}

/// Benchmark memory usage patterns
fn bench_memory_patterns(c: &mut Criterion) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let mut group = c.benchmark_group("memory_patterns");

    // Test with one very large file vs many small files

    // One large file
    let large_content = (0..1000)
        .map(|i| format!(
            "    <div className=\"p-{} bg-red-{} flex justify-center items-center m-{} text-white border-{} rounded-{}\">Item {}</div>",
            i % 8 + 1, i % 9 + 1, i % 6 + 1, i % 4 + 1, i % 4, i
        ))
        .collect::<Vec<_>>()
        .join("\n");

    let large_file_content = format!(
        r#"import React from 'react';
export const LargeComponent = () => (
  <div>
{}
  </div>
);"#,
        large_content
    );

    let large_file_path = temp_dir.path().join("large_file.tsx");
    fs::write(&large_file_path, large_file_content).expect("Failed to write large file");

    group.bench_function("large_single_file", |b| {
        b.iter(|| {
            let options = ProcessOptions {
                dry_run: true,
                write: false,
                check_formatted: false,
            };
            process_file(
                black_box(&large_file_path.display().to_string()),
                black_box(options),
            )
            .expect("Failed to process large file")
        })
    });

    // Many small files
    let _small_files =
        create_test_files(temp_dir.path().join("small_files").as_path(), 200, "simple");
    fs::create_dir_all(temp_dir.path().join("small_files"))
        .expect("Failed to create small files dir");

    for i in 0..200 {
        let content = format!(
            r#"import React from 'react';
export const Small{} = () => <div className="p-4 bg-red-500 flex">Small {}</div>;"#,
            i, i
        );
        fs::write(
            temp_dir
                .path()
                .join("small_files")
                .join(format!("small_{}.tsx", i)),
            content,
        )
        .expect("Failed to write small file");
    }

    group.bench_function("many_small_files", |b| {
        b.iter(|| {
            let config = FileDiscoveryConfig::default();
            let pipeline =
                FileProcessingPipeline::parallel(config).expect("Failed to create pipeline");
            let options = ProcessOptions {
                dry_run: true,
                write: false,
                check_formatted: false,
            };

            pipeline
                .process_files(
                    black_box(&[temp_dir.path().join("small_files").display().to_string()]),
                    black_box(options),
                )
                .expect("Failed to process small files")
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_single_file_processing,
    bench_batch_processing,
    bench_thread_scaling,
    bench_file_discovery,
    bench_memory_patterns
);
criterion_main!(benches);
