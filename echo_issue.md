# 🗣️ Echo: Getting Started example is broken

🤦 **The Confusion:** Tried to run the "Library Usage" and "Running ChimeraScript from Rust" code examples from `experiments/chimera-lang/README.md`. I literally copy-pasted the code into a fresh `main.rs` and added `chimera-lang` to `Cargo.toml`. The compiler threw multiple errors:
1. `unresolved import chimera_lang::compiler`
2. `use of undeclared type ChimeraVM`
3. `no function or associated item named default found for struct chimera_lang::ast::Dna`
4. `error: failed to load manifest for dependency chimera-lang` (due to missing `workspace.dependencies` like `anyhow` and `pest` when compiling outside the workspace).

🕵️ **The Reality:** The provided code snippets in the README do not compile out-of-the-box. The `chimera_lang::compiler` module and `ChimeraVM` might not be cleanly exported for external users not using `prelude::*`, `Dna::default()` (or similar initializers) doesn't exist as shown in previous docs (it requires `evolution_config: None` and a complex nested struct, though `from_genes` works now if imported), and most importantly, copying the crate as a dependency into a fresh project fails because `chimera-lang` relies on workspace-level dependencies that aren't resolved when included via a simple path dependency without a workspace root. Also, advanced features silently fail to compile without the `nova` feature flag.

💡 **The Fix:** Fix the code examples in the README to be completely foolproof.
- Provide a full, copy-pasteable `Cargo.toml` example that actually works (or explain how to resolve the workspace dependency hell).
- Use explicit, absolute module paths (`chimera_lang::vm::ChimeraVM`, `chimera_lang::compiler::compile`) instead of assuming prelude usage, or explicitly show the imports.
- Add a huge banner in README saying 'REQUIRES FEATURE NOVA' for the ChimeraScript compiler examples.
## 🔍 Experience - The Walkthrough

**Scenario:** "I am a new user trying to run the Evolution example from the root README."
**Action:** Try to use the quick start command `cargo run -p chimera-lang --features nova -- --input experiments/chimera-lang/examples/evolution.pro` based *only* on the public docs in `README.md`.

## 🚧 Stumble - The Friction Points

1.  **Missing File & Unhelpful Error:** The command fails with an incredibly unhelpful message: `Error: No such file or directory (os error 2)`. It turns out `evolution.pro` does not exist in the `examples` directory (only `evolution.chs` exists). The error message provides zero context about *which* file it failed to find.

## 📢 Report - The Complaint

**Title:** 🗣️ Echo: Unhelpful file error & missing example file

**Description:**
*   🤦 **The Confusion:** "Tried to run the `evolution.pro` example from the root `README.md`. Cargo gave me an error saying 'No such file or directory (os error 2)' but didn't tell me which file was missing."
*   🕵️ **The Reality:** "Turns out `evolution.pro` doesn't exist, and the error message from `chimera-lang` just spits out a generic OS error instead of telling the user what path failed to open."
*   💡 **The Fix:** "Improve the error message in `chimera-lang`'s `main.rs` to include the filename (e.g., `Failed to open file '...': ...`), and update documentation if necessary."
