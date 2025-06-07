use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::time::Instant;
use tempfile::TempDir;

/// Create a large test directory with many files for performance testing
fn create_large_test_directory(file_count: usize) -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    for i in 0..file_count {
        let content = format!(
            r#"import React from 'react';

export const Component{} = ({{ isActive, size = 'md' }}) => {{
  return (
    <div className="w-full h-screen bg-gray-{} flex items-center justify-center p-8">
      <div className="max-w-md mx-auto bg-white rounded-xl shadow-lg p-6">
        <h1 className="text-2xl font-bold text-gray-900 mb-4">Component {}</h1>
        <button 
          className={{`px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 ${{
            size === 'sm' ? 'text-sm' : size === 'lg' ? 'text-lg' : 'text-base'
          }} ${{
            isActive ? 'bg-blue-600 ring-2 ring-blue-300' : 'bg-blue-500'
          }}`}}
        >
          Button {}
        </button>
        <div className="mt-6 p-4 border border-gray-200 rounded-lg">
          <p className="text-gray-600 leading-relaxed">
            This is component {} with various Tailwind classes that need sorting.
          </p>
        </div>
      </div>
    </div>
  );
}};
"#,
            i, i % 9 + 1, i, i, i
        );
        
        fs::write(
            temp_dir.path().join(format!("Component{}.tsx", i)),
            content,
        ).expect("Failed to write test file");
    }
    
    temp_dir
}

#[test]
fn test_small_scale_performance() {
    let temp_dir = create_large_test_directory(10);
    
    let start = Instant::now();
    
    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode").arg("check")
        .arg("--stats")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Statistics:"))
        .stdout(predicate::str::contains("Total files: 10"));
    
    let duration = start.elapsed();
    println!("Processed 10 files in {:?}", duration);
    
    // Should complete quickly for small file count
    assert!(duration.as_secs() < 5, "Processing took too long: {:?}", duration);
}

#[test]
fn test_medium_scale_performance() {
    let temp_dir = create_large_test_directory(50);
    
    let start = Instant::now();
    
    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode").arg("check")
        .arg("--stats")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Total files: 50"));
    
    let duration = start.elapsed();
    println!("Processed 50 files in {:?}", duration);
    
    // Should complete in reasonable time
    assert!(duration.as_secs() < 10, "Processing took too long: {:?}", duration);
}

#[test]
fn test_parallel_vs_sequential_performance() {
    let temp_dir = create_large_test_directory(20);
    
    // Test sequential processing
    let start_sequential = Instant::now();
    let mut cmd_seq = Command::cargo_bin("windwarden").unwrap();
    cmd_seq.arg("format")
        .arg("--mode").arg("check")
        .arg("--processing").arg("sequential")
        .arg(temp_dir.path())
        .assert()
        .success();
    let sequential_duration = start_sequential.elapsed();
    
    // Test parallel processing
    let start_parallel = Instant::now();
    let mut cmd_par = Command::cargo_bin("windwarden").unwrap();
    cmd_par.arg("format")
        .arg("--mode").arg("check")
        .arg("--processing").arg("parallel")
        .arg(temp_dir.path())
        .assert()
        .success();
    let parallel_duration = start_parallel.elapsed();
    
    println!("Sequential: {:?}, Parallel: {:?}", sequential_duration, parallel_duration);
    
    // On multi-core systems, parallel should generally be faster or similar
    // We'll just ensure both complete in reasonable time
    assert!(sequential_duration.as_secs() < 15);
    assert!(parallel_duration.as_secs() < 15);
}

#[test]
fn test_thread_count_scaling() {
    let temp_dir = create_large_test_directory(30);
    
    // Test with different thread counts
    for thread_count in [1, 2, 4] {
        let start = Instant::now();
        
        let mut cmd = Command::cargo_bin("windwarden").unwrap();
        cmd.arg("format")
            .arg("--mode").arg("check")
            .arg("--threads").arg(thread_count.to_string())
            .arg(temp_dir.path())
            .assert()
            .success();
        
        let duration = start.elapsed();
        println!("Processed with {} threads in {:?}", thread_count, duration);
        
        // Should complete in reasonable time regardless of thread count
        assert!(duration.as_secs() < 20);
    }
}

#[test]
fn test_progress_reporting_overhead() {
    let temp_dir = create_large_test_directory(15);
    
    // Test without progress reporting
    let start_no_progress = Instant::now();
    let mut cmd_no_progress = Command::cargo_bin("windwarden").unwrap();
    cmd_no_progress.arg("format")
        .arg("--mode").arg("check")
        .arg(temp_dir.path())
        .assert()
        .success();
    let no_progress_duration = start_no_progress.elapsed();
    
    // Test with progress reporting
    let start_with_progress = Instant::now();
    let mut cmd_with_progress = Command::cargo_bin("windwarden").unwrap();
    cmd_with_progress.arg("format")
        .arg("--mode").arg("check")
        .arg("--progress")
        .arg(temp_dir.path())
        .assert()
        .success();
    let with_progress_duration = start_with_progress.elapsed();
    
    println!("No progress: {:?}, With progress: {:?}", no_progress_duration, with_progress_duration);
    
    // Progress reporting shouldn't add significant overhead
    let overhead_ratio = with_progress_duration.as_millis() as f64 / no_progress_duration.as_millis() as f64;
    assert!(overhead_ratio < 2.0, "Progress reporting adds too much overhead: {}x", overhead_ratio);
}

#[test]
fn test_large_file_processing() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create a single large file with many class attributes
    let mut large_content = String::from("import React from 'react';\n\nexport const LargeComponent = () => (\n  <div>\n");
    
    // Add many div elements with unsortered classes
    for i in 0..1000 {
        large_content.push_str(&format!(
            "    <div className=\"p-{} bg-red-{} flex justify-center items-center m-{} text-white border-{} rounded-{}\">Item {}</div>\n",
            i % 8 + 1, i % 9 + 1, i % 6 + 1, i % 4 + 1, i % 4, i
        ));
    }
    
    large_content.push_str("  </div>\n);\n");
    
    fs::write(
        temp_dir.path().join("large_file.tsx"),
        large_content,
    ).expect("Failed to write large file");
    
    let start = Instant::now();
    
    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode").arg("check")
        .arg("--stats")
        .arg(temp_dir.path().join("large_file.tsx"))
        .assert()
        .success()
        .stdout(predicate::str::contains("would be formatted"));
    
    let duration = start.elapsed();
    println!("Processed large file in {:?}", duration);
    
    // Should handle large files efficiently
    assert!(duration.as_secs() < 10, "Large file processing took too long: {:?}", duration);
}

#[test]
fn test_deep_directory_traversal() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create a deep directory structure
    let mut current_path = temp_dir.path().to_path_buf();
    for i in 0..10 {
        current_path.push(format!("level_{}", i));
        fs::create_dir_all(&current_path).expect("Failed to create directory");
        
        // Add a file at each level
        fs::write(
            current_path.join(format!("component_{}.tsx", i)),
            format!(r#"export const Level{} = () => <div className="p-4 bg-blue-{} flex">Level {}</div>;"#, i, i % 9 + 1, i),
        ).expect("Failed to write file");
    }
    
    let start = Instant::now();
    
    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode").arg("check")
        .arg("--stats")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Total files: 10"));
    
    let duration = start.elapsed();
    println!("Traversed deep directory structure in {:?}", duration);
    
    // Should handle deep structures efficiently
    assert!(duration.as_secs() < 10, "Deep traversal took too long: {:?}", duration);
}

#[test]
fn test_glob_pattern_performance() {
    let temp_dir = create_large_test_directory(25);
    
    // Test specific glob patterns
    let patterns = [
        "**/*.tsx",
        "Component*.tsx",
        "**/Component[0-9].tsx",
    ];
    
    for pattern in patterns {
        let start = Instant::now();
        
        let mut cmd = Command::cargo_bin("windwarden").unwrap();
        cmd.arg("format")
            .arg("--mode").arg("check")
            .arg(format!("{}/{}", temp_dir.path().display(), pattern))
            .assert()
            .success();
        
        let duration = start.elapsed();
        println!("Glob pattern '{}' processed in {:?}", pattern, duration);
        
        // Glob patterns should be processed efficiently
        assert!(duration.as_secs() < 10, "Glob pattern '{}' took too long: {:?}", pattern, duration);
    }
}

#[test]
fn test_memory_efficiency_many_files() {
    let temp_dir = create_large_test_directory(100);
    
    // This test primarily checks that the command completes without running out of memory
    // For a more detailed memory test, we'd need additional tooling
    
    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode").arg("check")
        .arg("--stats")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Total files: 100"))
        .stdout(predicate::str::contains("Success rate:"));
}

#[test]
fn test_concurrent_access_simulation() {
    let temp_dir = create_large_test_directory(20);
    
    // Test that multiple operations can be performed in sequence without issues
    for _run in 0..3 {
        let mut cmd = Command::cargo_bin("windwarden").unwrap();
        cmd.arg("format")
            .arg("--mode").arg("check")
            .arg("--processing").arg("parallel")
            .arg(temp_dir.path())
            .assert()
            .success();
    }
}