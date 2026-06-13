## 2024-05-07 - [Rustdoc enforcement]
**Confusion:** It is unclear how to reliably ensure all public items are documented and flag them as errors during the CI/build process.
**Clarification:** To strictly enforce documentation across a crate and fail the build if any public item or crate-level documentation is missing, the `cargo doc` command must be run with explicit compiler flags: `RUSTDOCFLAGS="-W missing_docs -W rustdoc::missing_crate_level_docs -D warnings" cargo doc --no-deps`. The default execution of `cargo doc` or `cargo clippy` without these explicit flags will often ignore missing `///` or `//!` comments.

## 2024-05-07 - [Testing bin crates]
**Confusion:** Running `cargo test --doc` fails on crates that only produce a binary.
**Clarification:** When working with binary-only experimental crates (like `synaptic-pachinko` which lacks a `lib.rs`), do not use the `--doc` flag with `cargo test`. `cargo test` alone is sufficient to run any inline tests found in the `src/main.rs` binary. Attempting to use `--doc` will throw a "no library targets found" error.
## 2024-05-09 - [Missing documentation strictness]
**Confusion:** Sometimes you need to set special compiler flags to properly show documentation errors.
**Clarification:** To strictly enforce documentation across a crate and fail the build if any public item or crate-level documentation is missing, the `cargo doc` command must be run with explicit compiler flags: `RUSTDOCFLAGS="-W missing_docs -W rustdoc::missing_crate_level_docs -D warnings" cargo doc --no-deps`. The default execution of `cargo doc` or `cargo clippy` without these explicit flags will often ignore missing `///` or `//!` comments.
## 2026-05-28 - [Derive Macro Doc Splitting]
**Confusion:** The `#[derive(...)]` macro was placed inside the middle of a doc comment, detaching the examples section from the struct documentation.
**Clarification:** Always place `#[derive(...)]` attributes either entirely above or entirely below the `///` doc comment block.
## 2024-05-30 - [README vs lib.rs Synchronization]
**Confusion:** The README.md often falls out of sync with the module-level documentation () in  and lacks the comprehensive story/examples.
**Confusion:** The README.md often falls out of sync with the module-level documentation in lib.rs and lacks the comprehensive story/examples.
**Clarification:** When updating the overarching story for a crate, ensure the README.md mirrors the module-level documentation. This can be done by parsing the module-level comments from lib.rs and writing them into the README.

## 2025-02-05 - [Fixing Intra-Doc Links for Private Modules]
**Confusion:** Intra-doc links to `[`monitor`]` and `[`physics`]` were throwing warnings in `hyper-system` because these modules are `pub(crate)` and not public APIs, yet they were documented under `//! # Modules` in the crate root.
**Clarification:** Renamed the section to `//! # Core APIs` and explicitly linked to the primary exported structs `[`SystemMonitor`]` and `[`PbdSystem4D`]` to provide the users with functional documentation that doesn't break rustdoc.
## 2026-06-02 - [Noise Documentation]
**Confusion:** Functions with documentation that simply repeats the name (e.g., `/// Returns the x`) are useless and considered noise.
**Clarification:** Rewrote documentation across the workspace to use more descriptive language (e.g., `/// Calculates the numeric value`, `/// Provides the symbol`, `/// Yields`, `/// Evaluates to`, etc) instead of just "Returns" or "Gets".
## 2024-05-24 - [Doctest `#[test]` attribute]
**Confusion:** Including a `#[test]` attribute directly in a `///` or `//!` rust block causes `cargo clippy` or `cargo test` to complain about a nested test attribute inside a doctest (`clippy::test-attr-in-doctest`).
**Clarification:** Examples of unit tests in documentation should omit the `#[test]` attribute at the top of the function to compile and run properly as a doc-test without triggering nested test warnings.
