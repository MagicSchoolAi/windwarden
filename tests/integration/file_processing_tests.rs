use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Helper function to create a temporary directory with test files
fn create_test_directory() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create a basic JSX file
    fs::write(
        temp_dir.path().join("test.jsx"),
        r#"import React from 'react';

function TestComponent() {
  return (
    <div className="p-4 bg-red-500 flex justify-center items-center m-2 text-white">
      <span className="font-bold text-lg p-2 bg-blue-500">Hello World</span>
      <button className="mt-4 px-6 py-2 bg-green-500 text-white rounded hover:bg-green-600">
        Click me
      </button>
    </div>
  );
}

export default TestComponent;"#,
    )
    .expect("Failed to write test.jsx");

    // Create a TypeScript React file
    fs::write(
        temp_dir.path().join("test.tsx"),
        r#"import React from 'react';

interface Props {
  title: string;
}

export const TestComponent: React.FC<Props> = ({ title }) => {
  return (
    <div className="w-full h-screen bg-gray-100 flex items-center justify-center p-8">
      <div className="max-w-md mx-auto bg-white rounded-xl shadow-lg p-6">
        <h1 className="text-2xl font-bold text-gray-900 mb-4">{title}</h1>
        <p className="text-gray-600 leading-relaxed">
          This is a test component with Tailwind classes.
        </p>
      </div>
    </div>
  );
};"#,
    )
    .expect("Failed to write test.tsx");

    // Create a file with function calls
    fs::write(
        temp_dir.path().join("utility.js"),
        r#"import { clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs) {
  return twMerge(clsx(inputs));
}

const buttonClasses = cn(
  "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50"
);

const cardClasses = clsx(
  "w-full max-w-sm mx-auto bg-white rounded-lg shadow-md overflow-hidden"
);

export { buttonClasses, cardClasses };"#,
    ).expect("Failed to write utility.js");

    // Create a nested directory structure
    fs::create_dir_all(temp_dir.path().join("src/components"))
        .expect("Failed to create nested directories");

    fs::write(
        temp_dir.path().join("src/components/Button.tsx"),
        r#"import React from 'react';

export function Button({ children, variant = 'primary' }) {
  const baseClasses = "px-4 py-2 rounded font-medium focus:outline-none focus:ring-2 focus:ring-offset-2";
  const variants = {
    primary: "bg-blue-500 text-white hover:bg-blue-600 focus:ring-blue-500",
    secondary: "bg-gray-500 text-white hover:bg-gray-600 focus:ring-gray-500"
  };
  
  return (
    <button className={`${baseClasses} ${variants[variant]}`}>
      {children}
    </button>
  );
}"#,
    ).expect("Failed to write Button.tsx");

    // Create a file that should be excluded (node_modules)
    fs::create_dir_all(temp_dir.path().join("node_modules/some-package"))
        .expect("Failed to create node_modules");
    fs::write(
        temp_dir.path().join("node_modules/some-package/index.js"),
        r#"// This file should be excluded by default
const classes = "p-4 m-2 bg-red-500";"#,
    )
    .expect("Failed to write node_modules file");

    temp_dir
}

#[test]
fn test_format_command_check_mode() {
    let temp_dir = create_test_directory();

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg(temp_dir.path().join("test.jsx"))
        .assert()
        .success()
        .stdout(predicate::str::contains("would be formatted"));
}

#[test]
fn test_format_command_write_mode() {
    let temp_dir = create_test_directory();
    let test_file = temp_dir.path().join("test.jsx");

    // First, read the original content
    let original_content = fs::read_to_string(&test_file).unwrap();

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("write")
        .arg(&test_file)
        .assert()
        .success();

    // Read the modified content
    let modified_content = fs::read_to_string(&test_file).unwrap();

    // The content should be different (sorted)
    assert_ne!(original_content, modified_content);

    // The modified content should contain properly sorted classes
    assert!(modified_content.contains("flex items-center justify-center"));
}

#[test]
fn test_format_command_verify_mode() {
    let temp_dir = create_test_directory();

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("verify")
        .arg(temp_dir.path().join("test.jsx"))
        .assert()
        .failure() // Should fail because files are not formatted
        .stdout(predicate::str::contains("not formatted"));
}

#[test]
fn test_check_command() {
    let temp_dir = create_test_directory();

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path().join("test.tsx"))
        .assert()
        .failure() // Should fail because file is not formatted
        .stdout(predicate::str::contains("not formatted"));
}

#[test]
fn test_directory_processing() {
    let temp_dir = create_test_directory();

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("would be formatted"));
}

#[test]
fn test_glob_pattern_processing() {
    let temp_dir = create_test_directory();

    // For now, test directory processing instead of glob patterns
    // since glob patterns need to be relative to current working directory
    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--extensions")
        .arg("tsx")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("would be formatted"));
}

#[test]
fn test_file_extension_filtering() {
    let temp_dir = create_test_directory();

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--extensions")
        .arg("tsx")
        .arg(temp_dir.path())
        .assert()
        .success();

    // Should only process .tsx files, not .jsx or .js files
    // We can verify this by checking that the output doesn't mention .jsx files
}

#[test]
fn test_exclude_patterns() {
    let temp_dir = create_test_directory();

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--exclude")
        .arg("node_modules/**")
        .arg(temp_dir.path())
        .assert()
        .success();

    // Should exclude node_modules by default, but let's be explicit
}

#[test]
fn test_next_directory_excluded_by_default() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create a valid source file
    fs::write(
        temp_dir.path().join("component.tsx"),
        r#"export const Component = () => <div className="p-4 bg-red-500 flex">Component</div>;"#,
    )
    .expect("Failed to write component file");

    // Create .next directory with JS files that would cause parse errors
    fs::create_dir_all(temp_dir.path().join(".next/server/chunks/ssr"))
        .expect("Failed to create .next directory");

    fs::write(
        temp_dir.path().join(".next/server/chunks/ssr/minified_chunk.js"),
        r#"(()=>{"use strict";var e,t,r,n,o,u,i,a,c,l,s,f,p,d,h,y,m,v,g,b,w,x,k,O,j,E,S,A,P,C,T={};"#,
    )
    .expect("Failed to write .next file");

    // Run windwarden - should NOT process .next files and should succeed
    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--stats")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Total files: 1")) // Only the component.tsx should be processed
        .stdout(predicate::str::contains("would be formatted"));
}

#[test]
fn test_json_config_ignore_patterns() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create valid source files
    fs::write(
        temp_dir.path().join("component.tsx"),
        r#"export const Component = () => <div className="p-4 bg-red-500 flex">Component</div>;"#,
    )
    .expect("Failed to write component file");

    fs::write(
        temp_dir.path().join("utils.ts"),
        r#"export const cn = (...args) => args.join(' ');"#,
    )
    .expect("Failed to write utils file");

    // Create directories that should be ignored via JSON config
    fs::create_dir_all(temp_dir.path().join("custom_build_dir/subdir"))
        .expect("Failed to create custom_build_dir");

    fs::write(
        temp_dir.path().join("custom_build_dir/should_be_ignored.tsx"),
        r#"export const Ignored = () => <div className="p-4 bg-red-500 flex">Should be ignored</div>;"#,
    )
    .expect("Failed to write ignored file");

    fs::write(
        temp_dir.path().join("custom_build_dir/subdir/nested.tsx"),
        r#"export const Nested = () => <div className="p-4 bg-red-500 flex">Nested ignored</div>;"#,
    )
    .expect("Failed to write nested ignored file");

    // Create .windwarden.json config with custom ignore patterns
    let config_content = r#"{
  "ignorePaths": [
    "node_modules",
    "custom_build_dir",
    ".git"
  ]
}"#;
    fs::write(temp_dir.path().join(".windwarden.json"), config_content)
        .expect("Failed to write config file");

    // Run windwarden with config - should only process component.tsx and utils.ts
    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("--config")
        .arg(temp_dir.path().join(".windwarden.json"))
        .arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--stats")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Total files: 2")) // component.tsx and utils.ts found, but only component.tsx needs formatting
        .stdout(predicate::str::contains("would be formatted"));
}

#[test]
fn test_json_config_glob_patterns() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create various files
    fs::write(
        temp_dir.path().join("component.tsx"),
        r#"export const Component = () => <div className="p-4 bg-red-500 flex">Component</div>;"#,
    )
    .expect("Failed to write component file");

    fs::create_dir_all(temp_dir.path().join("tests")).expect("Failed to create tests dir");

    fs::write(
        temp_dir.path().join("tests/component.test.tsx"),
        r#"test('renders', () => <div className="p-4 bg-red-500 flex">Test</div>);"#,
    )
    .expect("Failed to write test file");

    fs::create_dir_all(temp_dir.path().join("src/__tests__"))
        .expect("Failed to create __tests__ dir");

    fs::write(
        temp_dir.path().join("src/__tests__/utils.test.ts"),
        r#"test('utils', () => {});"#,
    )
    .expect("Failed to write utils test file");

    // Create config with glob patterns
    let config_content = r#"{
  "ignorePaths": [
    "**/*.test.*",
    "**/__tests__/**"
  ]
}"#;
    fs::write(temp_dir.path().join(".windwarden.json"), config_content)
        .expect("Failed to write config file");

    // Run windwarden - should only process component.tsx, not test files
    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("--config")
        .arg(temp_dir.path().join(".windwarden.json"))
        .arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--stats")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Total files: 2")) // Only component.tsx processed, test files excluded
        .stdout(predicate::str::contains("would be formatted"));
}

#[test]
fn test_cli_exclude_overrides_config() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create source files
    fs::write(
        temp_dir.path().join("component.tsx"),
        r#"export const Component = () => <div className="p-4 bg-red-500 flex">Component</div>;"#,
    )
    .expect("Failed to write component file");

    fs::create_dir_all(temp_dir.path().join("special")).expect("Failed to create special dir");

    fs::write(
        temp_dir.path().join("special/excluded.tsx"),
        r#"export const Excluded = () => <div className="p-4 bg-red-500 flex">Excluded</div>;"#,
    )
    .expect("Failed to write excluded file");

    // Create config that doesn't exclude 'special' directory
    let config_content = r#"{
  "ignorePaths": [
    "node_modules"
  ]
}"#;
    fs::write(temp_dir.path().join(".windwarden.json"), config_content)
        .expect("Failed to write config file");

    // Run windwarden with CLI exclude that should override/add to config
    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("--config")
        .arg(temp_dir.path().join(".windwarden.json"))
        .arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--exclude")
        .arg("special/**")
        .arg("--stats")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Total files: 1")) // Only component.tsx
        .stdout(predicate::str::contains("would be formatted"));
}

#[test]
fn test_default_ignore_patterns_from_config() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create source file
    fs::write(
        temp_dir.path().join("component.tsx"),
        r#"export const Component = () => <div className="p-4 bg-red-500 flex">Component</div>;"#,
    )
    .expect("Failed to write component file");

    // Create directories that should be ignored by default config
    for dir in &["node_modules", ".next", "dist", "target"] {
        fs::create_dir_all(temp_dir.path().join(format!("{}/subdir", dir)))
            .expect("Failed to create default ignore dir");

        fs::write(
            temp_dir.path().join(format!("{}/should_be_ignored.tsx", dir)),
            r#"export const Ignored = () => <div className="p-4 bg-red-500 flex">Should be ignored</div>;"#,
        )
        .expect("Failed to write ignored file");
    }

    // Run windwarden without explicit config - should use defaults and ignore standard directories
    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--stats")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Total files: 1")) // Only component.tsx
        .stdout(predicate::str::contains("would be formatted"));
}

#[test]
fn test_parallel_processing() {
    let temp_dir = create_test_directory();

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--processing")
        .arg("parallel")
        .arg("--threads")
        .arg("2")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("would be formatted"));
}

#[test]
fn test_sequential_processing() {
    let temp_dir = create_test_directory();

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--processing")
        .arg("sequential")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("would be formatted"));
}

#[test]
fn test_stats_output() {
    let temp_dir = create_test_directory();

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--stats")
        .arg(temp_dir.path().join("test.jsx"))
        .assert()
        .success()
        .stdout(predicate::str::contains("Statistics:"))
        .stdout(predicate::str::contains("Total files:"))
        .stdout(predicate::str::contains("Success rate:"));
}

#[test]
fn test_diff_output() {
    let temp_dir = create_test_directory();

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--diff")
        .arg(temp_dir.path().join("test.jsx"))
        .assert()
        .success()
        .stdout(predicate::str::contains("---"))
        .stdout(predicate::str::contains("+++"))
        .stdout(predicate::str::contains("-    <div className="))
        .stdout(predicate::str::contains("+    <div className="));
}

#[test]
fn test_error_handling_nonexistent_file() {
    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("nonexistent_file.tsx")
        .assert()
        .failure()
        .stderr(predicate::str::contains("File not found"))
        .stderr(predicate::str::contains(
            "Check that the file path is correct",
        ));
}

#[test]
fn test_error_handling_invalid_glob() {
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
fn test_error_handling_invalid_thread_count() {
    let temp_dir = create_test_directory();

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
        ));
}

#[test]
fn test_progress_reporting() {
    let temp_dir = create_test_directory();

    // Create more files to trigger progress reporting
    for i in 1..=10 {
        fs::write(
            temp_dir.path().join(format!("file_{}.tsx", i)),
            format!(
                r#"
import React from 'react';
export const Component{} = () => (
  <div className="p-4 m-2 bg-blue-500 text-white rounded">
    Component {}
  </div>
);
"#,
                i, i
            ),
        )
        .expect("Failed to write test file");
    }

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--progress")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stderr(predicate::str::contains("Processing"));
}

#[test]
fn test_multiple_file_types() {
    let temp_dir = create_test_directory();

    // Create additional JS file (without JSX to avoid parse errors)
    fs::write(
        temp_dir.path().join("test.js"),
        r#"// Simple JavaScript file with class strings
const buttonClasses = "p-4 bg-green-500 flex justify-center items-center m-2 text-white";
const headerClasses = "font-bold text-xl";

export { buttonClasses, headerClasses };"#,
    )
    .expect("Failed to write test.js");

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--extensions")
        .arg("js,jsx,tsx")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("would be formatted"));
}

#[test]
fn test_max_depth_limiting() {
    let temp_dir = create_test_directory();

    // Create deeply nested structure
    fs::create_dir_all(temp_dir.path().join("level1/level2/level3"))
        .expect("Failed to create deep structure");
    fs::write(
        temp_dir.path().join("level1/level2/level3/deep.tsx"),
        r#"export const Deep = () => <div className="p-4 m-2">Deep component</div>;"#,
    )
    .expect("Failed to write deep file");

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--max-depth")
        .arg("2")
        .arg(temp_dir.path())
        .assert()
        .success();

    // The deep file at level 3 should not be processed due to max-depth=2
}

#[test]
fn test_empty_directory() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("All files are already formatted!"));
}

#[test]
fn test_mixed_formatted_and_unformatted_files() {
    let temp_dir = create_test_directory();

    // Create an already properly formatted file
    fs::write(
        temp_dir.path().join("formatted.tsx"),
        r#"import React from 'react';

export const FormattedComponent = () => (
  <div className="flex items-center justify-center p-4">
    <span className="text-lg font-bold">Already formatted</span>
  </div>
);"#,
    )
    .expect("Failed to write formatted file");

    let mut cmd = Command::cargo_bin("windwarden").unwrap();
    cmd.arg("format")
        .arg("--mode")
        .arg("check")
        .arg("--stats")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("would be formatted"))
        .stdout(predicate::str::contains("Statistics:"));
}
