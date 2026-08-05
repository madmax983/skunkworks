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

## 2025-05-24 - [Fixing Workspace Dependency Version Conflicts in README]
**Confusion:** In `crates/tui-shared/README.md`, it originally instructed users to depend on `ratatui = "0.30"` when the workspace explicitly was set to `ratatui = "0.29"` or another version, causing standalone examples to fail with `unicode-width` dependency resolution conflicts.
**Clarification:** Always ensure that `README.md` standalone dependency examples reflect the exact version constraint of the workspace's root `Cargo.toml`.
## 2024-05-24 - [Fixing DX Audit Log Friction Points]
**Confusion:** Many README files lacked installation instructions, had unused variables generating warnings, or contained commands that didn't work from the workspace root.
**Clarification:** Added explicit Installation sections with version constraints to crates, prefixed unused variables in doctests with underscores, and updated cargo run examples to specify the package name.

## 2025-06-23 - [Strict Missing Docs and Macro Generated Code]
**Confusion:** Strict `missing_docs` lints (`RUSTDOCFLAGS="-W missing_docs"`) apply to macro-generated code, including `#[derive(Parser)]` from both `clap` and `pest_derive`.
**Clarification:** To resolve `missing_docs` errors for `pest_derive` generated code, wrap the struct in an inline submodule and apply `#![allow(missing_docs)]` at the top of the inner module file, then `pub use` the items. For `clap`, applying `#[allow(missing_docs)]` directly on the struct also helps, or adding regular `///` doc comments directly inside the `enum` variants in the Rust code!
## 2024-05-24 - [Simplifying Jargon in AGENTS.md]
**Confusion:** The DX Audit Log flagged `AGENTS.md` for having excessive jargon ("Stigmergy", "Pheromone Trails", "Emergent Standards"), confusing users on how to contribute.
**Clarification:** Replaced jargon in `AGENTS.md` with clearer terms ("Indirect Coordination", "Status Updates", "Adopted Conventions") to make the human interface of agent guidelines more accessible.

## 2026-07-03 - [Fixing Cargo Run Arguments in README]
**Confusion:** The Quick Start command in `experiments/neuro-physics/README.md` was `cargo run -p neuro-physics --headless`, which caused an error because cargo thought `--headless` was a cargo argument instead of an argument to the binary.
**Clarification:** You must use the `--` separator when passing arguments to the underlying binary instead of cargo itself, e.g. `cargo run -p neuro-physics -- --headless`.

## 2026-07-03 - [Fixing Quick Start Code Blocks in README]
**Confusion:** The Quick Start code block in `crates/ferrous-core/README.md` was missing a `fn main() { ... }` wrapper, causing compilation errors when users copy-pasted it.
**Clarification:** Always wrap example code blocks inside a `fn main() { ... }` block in READMEs so they are valid, runnable Rust programs.

## 2026-07-06 - [Fixing Headless Flag in Examples and Private Facade imports]
**Confusion:** TUI examples might hang indefinitely in headless/CI environments if they don't explicitly parse and handle the `--headless` flag. Also, users were directed to import private facade modules directly.
**Clarification:** Examples that launch TUIs should explicitly handle the `--headless` argument (e.g. `if std::env::args().any(|arg| arg == "--headless")`) by either running their logic directly without `run_tui` or exiting immediately, to prevent hanging in non-interactive environments. Additionally, example code in READMEs should import from the public facade instead of private internal modules.
## 2026-07-06 - [Module-Level Docs and README Sync for Executables]
**Confusion:** Building documentation for a binary crate using strict rustdoc flags (`-W rustdoc::missing_crate_level_docs -D warnings`) will fail if the `src/main.rs` file does not include a `//!` crate-level doc comment block.
**Clarification:** To satisfy `cargo doc` for binary crates, parse the `README.md` contents and inject them as `//!` block comments at the very top of `src/main.rs`. This ensures the overarching story for the executable is documented and the documentation build passes.
## 2024-07-27 - [Noisy PrologueProgram docs]
**Confusion:** The `prologue_compiler.rs` file had auto-generated noisy documentation, e.g. "The `dna` field." repeated 100+ times, and "Represents a `PrologueProgram`." repeated 100+ times, and noisy compile examples.
**Clarification:** I removed the auto-generated noisy lines using a Python script.
## 2026-07-28 - [Pest Derive Macro Missing Docs Mitigation]
**Confusion:** Using `pest_derive` directly on a public struct triggers `missing_docs` lints which cannot be easily suppressed with just `#[allow(missing_docs)]` on the struct itself due to macro expansion intricacies.
**Clarification:** Wrap the pest-generated struct inside an inline submodule (`pub(crate) mod parser_impl`) adorned with `#![allow(missing_docs)]` at the top of the module, and then `pub use` the items to satisfy strict missing docs checking while hiding the macro internals.

## 2026-08-05 - [Module-Level Docs and README Sync for Executables]
**Confusion:** Building documentation for a binary crate using strict rustdoc flags (`-W rustdoc::missing_crate_level_docs -D warnings`) will fail if the `src/main.rs` file does not include a `//!` crate-level doc comment block.
**Clarification:** To satisfy `cargo doc` for binary crates, parse the `README.md` contents and inject them as `//!` block comments at the very top of `src/main.rs`. This ensures the overarching story for the executable is documented and the documentation build passes.
