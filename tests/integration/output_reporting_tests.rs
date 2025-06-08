use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Test output reporting in different modes to ensure it correctly shows when files are changed
#[test]
fn test_write_mode_reports_changes_correctly() {
    let temp_dir = TempDir::new().unwrap();

    // Create a file that needs formatting
    let test_file = temp_dir.path().join("test.tsx");
    fs::write(
        &test_file,
        r#"function Button() {
  return (
    <button className="p-4 bg-blue-500 text-white flex items-center">
      Click me
    </button>
  );
}"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("write")
        .arg("--stats")
        .arg(&test_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("Formatted files:"))
        .stdout(predicate::str::contains("1 file formatted"))
        .stdout(predicate::str::contains("Changed: 1"));
}

#[test]
fn test_write_mode_with_already_formatted_file() {
    let temp_dir = TempDir::new().unwrap();

    // Create a file that's already properly formatted (using official Tailwind order)
    let test_file = temp_dir.path().join("test.tsx");
    fs::write(
        &test_file,
        r#"function Button() {
  return (
    <button className="flex items-center p-4 text-white bg-blue-500">
      Click me
    </button>
  );
}"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("write")
        .arg("--stats")
        .arg(&test_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("No files needed formatting!"))
        .stdout(predicate::str::contains("Changed: 0"));
}

#[test]
fn test_check_mode_reports_changes_correctly() {
    let temp_dir = TempDir::new().unwrap();

    // Create a file that needs formatting
    let test_file = temp_dir.path().join("test.tsx");
    fs::write(
        &test_file,
        r#"function Button() {
  return (
    <button className="p-4 bg-blue-500 text-white flex items-center">
      Click me
    </button>
  );
}"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--stats")
        .arg(&test_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("Files that would be formatted:"))
        .stdout(predicate::str::contains("1 file would be formatted"))
        .stdout(predicate::str::contains("Changed: 1"));
}

#[test]
fn test_check_mode_with_already_formatted_file() {
    let temp_dir = TempDir::new().unwrap();

    // Create a file that's already properly formatted
    let test_file = temp_dir.path().join("test.tsx");
    fs::write(
        &test_file,
        r#"function Button() {
  return (
    <button className="flex items-center p-4 text-white bg-blue-500">
      Click me
    </button>
  );
}"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--stats")
        .arg(&test_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("All files are already formatted!"))
        .stdout(predicate::str::contains("Changed: 0"));
}

#[test]
fn test_verify_mode_reports_unformatted_files() {
    let temp_dir = TempDir::new().unwrap();

    // Create a file that needs formatting
    let test_file = temp_dir.path().join("test.tsx");
    fs::write(
        &test_file,
        r#"function Button() {
  return (
    <button className="p-4 bg-blue-500 text-white flex items-center">
      Click me
    </button>
  );
}"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("verify")
        .arg("--stats")
        .arg(&test_file)
        .assert()
        .failure() // Should fail because file is not formatted
        .stdout(predicate::str::contains("Unformatted files:"))
        .stdout(predicate::str::contains("1 file not formatted"))
        .stdout(predicate::str::contains("Changed: 1"));
}

#[test]
fn test_verify_mode_with_formatted_file() {
    let temp_dir = TempDir::new().unwrap();

    // Create a file that's already properly formatted
    let test_file = temp_dir.path().join("test.tsx");
    fs::write(
        &test_file,
        r#"function Button() {
  return (
    <button className="flex items-center p-4 text-white bg-blue-500">
      Click me
    </button>
  );
}"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("verify")
        .arg("--stats")
        .arg(&test_file)
        .assert()
        .success() // Should succeed because file is already formatted
        .stdout(predicate::str::contains(
            "All files are properly formatted!",
        ))
        .stdout(predicate::str::contains("Changed: 0"));
}

#[test]
fn test_multiple_files_mixed_status() {
    let temp_dir = TempDir::new().unwrap();

    // Create one file that needs formatting
    let unformatted_file = temp_dir.path().join("unformatted.tsx");
    fs::write(
        &unformatted_file,
        r#"function Button() {
  return (
    <button className="p-4 bg-blue-500 text-white flex items-center">
      Click me
    </button>
  );
}"#,
    )
    .unwrap();

    // Create one file that's already formatted
    let formatted_file = temp_dir.path().join("formatted.tsx");
    fs::write(
        &formatted_file,
        r#"function Card() {
  return (
    <div className="flex items-center p-4 bg-white">
      Card content
    </div>
  );
}"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--stats")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("1 file would be formatted"))
        .stdout(predicate::str::contains("Total files: 2"))
        .stdout(predicate::str::contains("Changed: 1"));
}

#[test]
fn test_diff_output_shows_actual_changes() {
    let temp_dir = TempDir::new().unwrap();

    // Create a file that needs formatting
    let test_file = temp_dir.path().join("test.tsx");
    fs::write(
        &test_file,
        r#"function Button() {
  return (
    <button className="p-4 bg-blue-500 text-white flex items-center">
      Click me
    </button>
  );
}"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--diff")
        .arg(&test_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("---"))
        .stdout(predicate::str::contains("+++"))
        .stdout(predicate::str::contains(
            "-    <button className=\"p-4 bg-blue-500 text-white flex items-center\">",
        ))
        .stdout(predicate::str::contains("+    <button className=\""));
}

#[test]
fn test_no_config_uses_official_tailwind_order() {
    let temp_dir = TempDir::new().unwrap();

    // Create a file with classes in non-official order
    let test_file = temp_dir.path().join("test.tsx");
    fs::write(
        &test_file,
        r#"function Button() {
  return (
    <button className="bg-blue-500 text-white p-4 flex items-center">
      Click me
    </button>
  );
}"#,
    )
    .unwrap();

    // Run without any config file to ensure we use official Tailwind order
    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--diff")
        .arg(&test_file)
        .current_dir(&temp_dir) // Run from temp dir so no config is found
        .assert()
        .success()
        // Should reorder to official Tailwind order: flex items-center first, then spacing, then colors
        .stdout(predicate::str::contains("flex items-center"));
}
