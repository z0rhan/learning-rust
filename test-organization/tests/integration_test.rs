// This is integration test and hence is completely external to our lib/crate
// Hence, we bring the function we want to test into scope with use
// Also, the integration tests live under tests/ dir alongside src/
// and does not need the cfg(test) attribute
// At the same time, each file in tests/ is a crate
use test_organization::add;

// Now, when we run cargo test it runs the unit test, integration test and doc
// test. If any fails the test stops and does not continue to the next one
// We can still specify the tests we want to run
// To run all tests in a particular integration test file, we can use --test 
// to provide the name of the file
// cargo test --test <filename>

// If we want to share some code between different integration test and we
// create a file inside tests/, cargo treats the new file as a seperate
// integration test.
// So, we use the older Rust file path to declare a module with module_name/mod.rs
// Files in subdir of tests/ don't get treated as new crate
mod common;

// Also, for binary crate since it does not expose functions, we can't have
// integration test for them

#[test]
fn adds_two() -> Result<(), String> {
    common::setup_test();
    let result = add(2, 2);

    if result == 4 {
        Ok(())
    }
    else {
        Err(String::from("addition operation done wrong"))
    }
}
