// Integration test runner for WindWarden
//
// This file serves as the main entry point for integration tests.
// Run with: cargo test --test integration
//
// Individual test categories can be run with:
// - cargo test --test integration cli_tests
// - cargo test --test integration error_handling_tests
// - cargo test --test integration file_processing_tests
// - cargo test --test integration output_reporting_tests
// - cargo test --test integration performance_tests

mod cli_tests;
mod error_handling_tests;
mod file_processing_tests;
mod output_reporting_tests;
mod performance_tests;

#[cfg(test)]
mod test_runner {
    use std::env;

    #[test]
    fn run_integration_tests() {
        // This test serves as documentation and a way to run all integration tests
        // The actual tests are in the individual modules above

        println!("Running WindWarden Integration Tests");
        println!("====================================");
        println!();
        println!("Test Categories:");
        println!("- CLI Tests: Basic command-line interface functionality");
        println!("- Error Handling Tests: Error scenarios and recovery");
        println!("- File Processing Tests: Core file processing workflows");
        println!("- Output Reporting Tests: Correct reporting of changes and formatting status");
        println!("- Performance Tests: Performance and scalability validation");
        println!();

        if env::var("RUST_LOG").is_err() {
            println!("Tip: Set RUST_LOG=debug for verbose output");
        }
    }
}
