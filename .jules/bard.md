## 2024-05-07 - [Rustdoc enforcement]
**Confusion:** It is unclear how to reliably ensure all public items are documented and flag them as errors during the CI/build process.
**Clarification:** To strictly enforce documentation across a crate and fail the build if any public item or crate-level documentation is missing, the `cargo doc` command must be run with explicit compiler flags: `RUSTDOCFLAGS="-W missing_docs -W rustdoc::missing_crate_level_docs -D warnings" cargo doc --no-deps`. The default execution of `cargo doc` or `cargo clippy` without these explicit flags will often ignore missing `///` or `//!` comments.

## 2024-05-07 - [Testing bin crates]
**Confusion:** Running `cargo test --doc` fails on crates that only produce a binary.
**Clarification:** When working with binary-only experimental crates (like `synaptic-pachinko` which lacks a `lib.rs`), do not use the `--doc` flag with `cargo test`. `cargo test` alone is sufficient to run any inline tests found in the `src/main.rs` binary. Attempting to use `--doc` will throw a "no library targets found" error.
