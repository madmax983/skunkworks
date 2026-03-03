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

## 🗣️ Echo: Getting Started example is broken

*   🤦 **The Confusion:** "Tried to run the `examples/evolution.pro` as shown in the README Quick Start (`cargo run -- --input examples/evolution.pro`). The file was missing. When I tried to create the file based on the README syntax, I got an error `Warning: Unknown enzyme 'beta'. Did you mean 'ret'?`."
*   🕵️ **The Reality:** "Turns out the Quick Start command needed `-p chimera-lang` from the root workspace, the `examples/evolution.pro` file did not exist in the root (it only had `chaos_lab.chs` in the workspace's `examples` folder), and the README syntax for `definitions` incorrectly showed a nested `strand` block inside curly braces, which the `prologue_compiler` choked on since it automatically wraps those contents with `strand rune_X { ... }`."
*   💡 **The Fix:** "Add the missing `examples/evolution.pro` file, update the `cargo run` command in `README.md` to properly reference `-p chimera-lang`, and fix the `definitions` example in the README to remove the nested `strand` syntax."
