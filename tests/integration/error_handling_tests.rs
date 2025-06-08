use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_file_not_found_error() {
    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("nonexistent_file.tsx")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "File not found: nonexistent_file.tsx",
        ))
        .stderr(predicate::str::contains(
            "Check that the file path is correct",
        ));
}

#[test]
fn test_invalid_glob_pattern_error() {
    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("src/**[invalid")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid glob pattern"))
        .stderr(predicate::str::contains("Check glob syntax"));
}

#[test]
fn test_invalid_thread_count_zero() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--threads")
        .arg("0")
        .arg(temp_dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Thread count must be greater than 0",
        ))
        .stderr(predicate::str::contains(
            "Check command line arguments syntax",
        ));
}

#[test]
fn test_invalid_thread_count_too_high() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--threads")
        .arg("2000")
        .arg(temp_dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Thread count cannot exceed 1024"))
        .stderr(predicate::str::contains(
            "Check command line arguments syntax",
        ));
}

#[test]
fn test_syntax_error_handling() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create a file with syntax errors
    fs::write(
        temp_dir.path().join("syntax_error.jsx"),
        r#"import React from 'react';

function SyntaxErrorComponent() {
  return (
    <div className="p-4 bg-red-500">
      <span className="font-bold">Error Test</span>
      {/* Missing closing bracket */}
    <div className="mt-4"
  );
}"#,
    )
    .expect("Failed to write syntax error file");

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg(temp_dir.path().join("syntax_error.jsx"))
        .assert()
        .failure() // Should fail due to parse errors
        .stdout(predicate::str::contains("Failed to process:"))
        .stdout(predicate::str::contains("Parse error"));
}

#[test]
fn test_permission_denied_handling() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let test_file = temp_dir.path().join("permission_test.tsx");

    // Create a test file
    fs::write(
        &test_file,
        r#"export const Test = () => <div className="p-4 m-2">Test</div>;"#,
    )
    .expect("Failed to write permission test file");

    // Remove read permissions (this might not work on all systems, so we check)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&test_file).unwrap().permissions();
        perms.set_mode(0o000); // No permissions
        fs::set_permissions(&test_file, perms).unwrap();

        let mut cmd = Command::cargo_bin("windwarden").unwrap();
        cmd.arg("format")
            .arg("--mode")
            .arg("check")
            .arg(&test_file)
            .assert()
            .failure() // Should fail due to permission issues
            .stdout(predicate::str::contains("Failed to process:"))
            .stdout(predicate::str::contains("Permission denied"));

        // Restore permissions for cleanup
        let mut perms = fs::metadata(&test_file).unwrap().permissions();
        perms.set_mode(0o644);
        fs::set_permissions(&test_file, perms).unwrap();
    }
}

#[test]
fn test_empty_path_error() {
    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        // No paths provided
        .assert()
        .failure(); // Should fail due to missing paths
}

#[test]
fn test_unsupported_file_extension() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create a file with unsupported extension
    fs::write(
        temp_dir.path().join("test.php"),
        r#"<?php
echo '<div class="p-4 bg-red-500 flex justify-center">PHP file</div>';
?>"#,
    )
    .expect("Failed to write PHP file");

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg(temp_dir.path().join("test.php"))
        .assert()
        .success() // Should succeed but not process the file
        .stdout(predicate::str::contains("All files are already formatted!"));
}

#[test]
fn test_malformed_exclude_pattern() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--exclude")
        .arg("**[invalid")
        .arg(temp_dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid glob pattern"))
        .stderr(predicate::str::contains("Check glob syntax"));
}

#[test]
fn test_directory_not_found() {
    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("/nonexistent/directory/path")
        .assert()
        .failure()
        .stderr(predicate::str::contains("File not found"))
        .stderr(predicate::str::contains(
            "Check that the file path is correct",
        ));
}

#[test]
fn test_mixed_valid_and_invalid_paths() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create a valid file
    fs::write(
        temp_dir.path().join("valid.tsx"),
        r#"export const Valid = () => <div className="p-4 m-2">Valid</div>;"#,
    )
    .expect("Failed to write valid file");

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg(temp_dir.path().join("valid.tsx"))
        .arg("nonexistent.tsx")
        .assert()
        .failure() // Should fail due to one invalid path
        .stderr(predicate::str::contains("File not found: nonexistent.tsx"));
}

#[test]
fn test_error_recovery_continues_processing() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create a valid file
    fs::write(
        temp_dir.path().join("valid.tsx"),
        r#"export const Valid = () => <div className="p-4 bg-red-500 flex">Valid</div>;"#,
    )
    .expect("Failed to write valid file");

    // Create a file with syntax errors
    fs::write(
        temp_dir.path().join("invalid.jsx"),
        r#"import React from 'react';
function Invalid() {
  return <div className="p-4"  // Missing closing tag and bracket
}"#,
    )
    .expect("Failed to write invalid file");

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--stats")
        .arg(temp_dir.path())
        .assert()
        .failure() // Should fail due to some processing errors
        .stdout(predicate::str::contains("would be formatted")) // Valid file should be processed
        .stdout(predicate::str::contains("Failed to process:")) // Invalid file should be reported
        .stdout(predicate::str::contains("Statistics:"))
        .stdout(predicate::str::contains("Failed: 1")); // Should report 1 failed file
}
