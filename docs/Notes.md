# Publishing crates
```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"
```
With the above metadata included in the Cargo.toml file, we can publish the crate as,
```bash
cargo publish
```

When we want to update the crate, we change the *version* value and republish the crate.

# Deprecating crates
Since we cannot delete published crates, we can make it so that new projects cannot add them as a dependency by,
```bash
cargo yank --vers 1.0.1
```
This makes it so that new projects cannot add v1.0.1 of this crate as dependency while the projects already using them can continue to use without any problems.

We can also undo this by,
```bash
cargo yank --vers 1.0.1 --undo
```
