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
