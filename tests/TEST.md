All the options for test are provided as,
```bash
cargo test -- --<option>
```

## Running test in one thread to prevent race conditions
```bash
cargo test -- --test-threads=1
```

## Displaying the output from stdout
When running tests, any output to stdout is captured if the test passes. We can display the output with,
```bash
cargo test -- --show-output
```

## We can also target a subset of tests
We can provide the name of the test we want to run as the arguement as,
```bash
cargo test can_hold_smaller
```

We can also provide the prefix for the name of the tests we want to run,
```bash
cargo test merge
```
This runs only the tests that starts with merge (i.e. all merge* tests).

## We can also exclude tests from running
For this, we need to add the ignore attribute to the test we want to ignore as,
```rust
#[test]
#[ignore]
fn test_add() {
    // some test
}
```
The above test will be ignored when we run `cargo run`.

Conversely, we can also run only the tests that were set-up to be ignored as,
```bash
cargo test -- --ignored
```

Finally, we can also run all the tests including the ignored ones as,
```bash
cargo test -- --include-ignored
```
