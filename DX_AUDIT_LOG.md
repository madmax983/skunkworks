# Echo's DX Audit Log 🗣️

**Target:** `crates/tui-shared/README.md`
**Date:** 2024-05-24

## 🔍 Experience - The Walkthrough

I attempted to follow the "Getting Started" instructions for `tui-shared` by creating a new Rust project (`cargo new experiments/echo-test`) and copying the provided code.

## 🚧 Stumble - The Friction Points

1.  **Path Confusion:**
    The README instructs to add:
    ```toml
    tui-shared = { path = "crates/tui-shared" }
    ```
    This path assumes the user is in the root of the repository or has a specific directory structure that is not explained. For a new project created inside `experiments/`, this path is incorrect (`../../crates/tui-shared` is needed).
    -   *Impact:* Build failure `failed to read .../experiments/echo-test/crates/tui-shared/Cargo.toml`.

2.  **Missing Dependency:**
    The example code uses `ratatui` directly:
    ```rust
    use ratatui::{widgets::{Block, Borders, Paragraph}, layout::Alignment};
    ```
    However, the installation instructions only mention adding `tui-shared`.
    -   *Impact:* Compilation error `use of undeclared crate or module ratatui`.
    -   *Fix:* Users must explicitly add `ratatui` to their dependencies to use types from it, even if `tui-shared` uses it internally.

## 📢 Report - The Complaint

**Title:** 🗣️ Echo: Getting Started example is broken

**Description:**
*   🤦 **The Confusion:** "Tried to run the `tui-shared` example. Compiler said `ratatui` not found."
*   🕵️ **The Reality:** "Turns out I needed to add `ratatui` to my dependencies manually, and the path to `tui-shared` was wrong for my setup."
*   💡 **The Fix:** "Update README to include `ratatui` in dependencies and clarify the path usage."

## 🔍 Experience - The Walkthrough

**Scenario:** "I am a new user trying to use the Ghost Mode (Event Replay) feature."
**Action:** Try to use the API based *only* on the public docs in `MARKETPLACE.md`.

## 🚧 Stumble - The Friction Points

1.  **Missing Feature:** The docs say to enable `features = ["nova"]` in `tui-shared`. Cargo failed with: `error: none of the selected packages contains these features: nova`.
2.  **Missing Types:** The docs say to wrap my `SystemEventSource` in `RecordingEventSource`. The compiler couldn't find `RecordingEventSource` or `SystemEventSource` in `tui-shared`.

## 📢 Report - The Complaint

**Title:** 🗣️ Echo: Ghost Mode instructions in MARKETPLACE are hallucinated

**Description:**
*   🤦 **The Confusion:** "Tried to set up Ghost Mode using MARKETPLACE.md. Cargo complained about a missing `nova` feature, and the compiler couldn't find `RecordingEventSource`."
*   🕵️ **The Reality:** "Turns out the `nova` feature, `RecordingEventSource`, and even the entire `ghost` functionality do not actually exist in the `tui-shared` crate."
*   💡 **The Fix:** "Remove the completely hallucinated 'Ghost Mode' entry from `MARKETPLACE.md` to avoid confusing users."

## 🔍 Experience - The Walkthrough

**Scenario:** "I am a new user trying to add `Nova`'s story feature."
**Action:** Try to use the API based *only* on the public docs/examples in `experiments/chimera-lang/README.md`.

## 🚧 Stumble - The Friction Points

1.  **Workspace Dependency Error:** The README's `Library Usage` section tells me to copy `chimera-lang = { path = "../chimera-lang" }` into my `Cargo.toml`. When I run `cargo run` in a fresh project without a parent workspace, it fails with: `error inheriting anyhow from workspace root manifest's workspace.dependencies.anyhow`.
2.  **Programmatic Usage Lie:** The README says "See `examples/story_demo.rs` for a full example of programmatic usage". But when I run `cargo run -p chimera-lang --example story_demo`, instead of running headlessly, it launches an interactive ratatui TUI that blocks the terminal indefinitely unless I press `Q`.

## 📢 Report - The Complaint

**Title:** 🗣️ Echo: Getting Started example is broken

**Description:**
*   🤦 **The Confusion:** "Tried to run the `story_demo` programmatically like the docs said, but it just opened a terminal UI and hung there. And copying the `Cargo.toml` dependencies caused a workspace inheritance error."
*   🕵️ **The Reality:** "Turns out `chimera-lang` relies heavily on workspace dependencies like `anyhow` and `ratatui` that aren't provided in the README, and `story_demo.rs` launches a blocking TUI instead of a library example."
*   💡 **The Fix:** "Add a huge banner in README saying 'REQUIRES WORKSPACE OR EXPLICIT DEPENDENCIES' and change `story_demo` to be a real headless programmatic example (or update the text to say it launches a TUI)."
