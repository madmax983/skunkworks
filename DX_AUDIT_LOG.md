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

## 🔍 Experience - The Walkthrough

**Scenario:** "I am a new user trying to add `Nova`'s story feature."
**Action:** Try to use the API based *only* on the public docs/examples.

## 🚧 Stumble - The Friction Points

1. **Missing Feature:** Tried to run the `story_demo`. Compiler said `NarrativeGenerator` not found.

## 📢 Report - The Complaint

**Title:** 🗣️ Echo: Getting Started example is broken

**Description:**
*   🤦 **The Confusion:** "Tried to run the `story_demo`. Compiler said `NarrativeGenerator` not found."
*   🕵️ **The Reality:** "Turns out I needed to enable feature `nova`."
*   💡 **The Fix:** "Add a huge banner in README saying 'REQUIRES FEATURE NOVA'."

## 🔍 Experience - The Walkthrough

**Scenario:** "I am a new user trying to run the Evolution example from the root README."
**Action:** Try to use the quick start command `cargo run -- --input examples/evolution.pro` based *only* on the public docs in `README.md`.

## 🚧 Stumble - The Friction Points

1.  **Multiple Binaries:** The command `cargo run` fails because it "could not determine which binary to run". The repository is a workspace with dozens of binaries.
2.  **Parser Error:** When I guess that I should run the `chimera-lang` binary instead (`cargo run -p chimera-lang -- --input examples/evolution.pro`), it fails with a syntax error on line 1: `expected strand`. It seems the file format `.pro` is not what the default parser expects.

## 📢 Report - The Complaint

**Title:** 🗣️ Echo: Root README Quick Start is broken

**Description:**
*   🤦 **The Confusion:** "Tried to run the `evolution.pro` example from the root `README.md`. Cargo gave me an error about multiple binaries, and when I specified `-p chimera-lang`, it crashed with a parsing error about 'expected strand'."
*   🕵️ **The Reality:** "Turns out the repository is a massive workspace so a bare `cargo run` doesn't work. Furthermore, the `chimera-lang` binary doesn't seem to know how to parse `.pro` files natively without extra configuration or flags that are completely missing from the README."
*   💡 **The Fix:** "Update the root README's Quick Start command to specify the exact binary required and any necessary flags or features needed to parse `.pro` files."

## 🔍 Experience - The Walkthrough

**Scenario:** "I am a new user trying to run the Hello World Prologue example from the root README."
**Action:** Run `cargo run -p chimera-lang --features nova -- --input experiments/chimera-lang/examples/mad_scientist.prl`.

## 🚧 Stumble - The Friction Points

1.  **Errors Galore:** The README tells me "Run the Hello World Prologue example to see the Prologue engine in action", but when I run it, the console is filled with `Error: Unknown OpCode` and `Stack underflow` errors.
2.  **Path Confusion:** The README has instructions to clone the repo and `cd chimera-lang`, but there is no `chimera-lang` folder at the root. It's inside `experiments/chimera-lang`.
3.  **Insane Dependency Requirements:** The library usage instructions in `experiments/chimera-lang/README.md` require me to manually add `tui-shared`, `locus`, `resonance-audio`, `hyper-system`, `poincare-disk` using relative local paths just to get it to compile! It tells me to `use chimera_lang::ast::Dna`, but I have to clone 10 random sub-crates just to try out the library.

## 📢 Report - The Complaint

**Title:** 🗣️ Echo: Getting Started example is broken and DX is terrible

**Description:**
*   🤦 **The Confusion:** "Tried to run the Hello World Prologue example (`mad_scientist.prl`) and the console exploded with 'Unknown OpCode' and 'Stack underflow' errors. The README says `cd chimera-lang` but the folder doesn't exist. Finally, to use the library I have to import half of the workspace manually."
*   🕵️ **The Reality:** "Turns out the Mad Scientist mode injects chaos runes that the VM tries to execute as OpCodes, causing error spam. The repo structure doesn't match the clone instructions, and the library is deeply coupled with random workspace crates instead of keeping them optional or private."
*   💡 **The Fix:** "Fix `mad_scientist.prl` so it doesn't crash visually for new users, correct the folder path in the root README, and decouple the `chimera-lang` crate from requiring users to manually import 5 different internal UI/physics crates just to run a script."

---

# Echo's DX Audit Log 🗣️

**Target:** `experiments/chimera-lang/README.md`
**Date:** 2025-05-24

## 🔍 Experience - The Walkthrough

I am a new user trying to run the Genesis example as described in the `chimera-lang` README.md. I literally copy-pasted the example commands from the "Running" section:

```bash
# Running complex examples like Genesis
cargo run -p chimera-lang --release -- --input experiments/chimera-lang/examples/genesis.chs
```

## 🚧 Stumble - The Friction Points

1. **File Not Found Confusion:**
   The command fails immediately with:
   ```
   Error: No such file or directory (os error 2)
   ```
   When I look inside `experiments/chimera-lang/examples/`, there is no file named `genesis.chs`. The only files there are `mad_scientist.prl` and `story_demo.rs`.

## 📢 Report - The Complaint

**Title:** 🗣️ Echo: Getting Started example is broken (Missing genesis.chs)

* 🤦 **The Confusion:** Tried to run the complex example `genesis.chs` as explicitly documented in the README. The system just spat an `os error 2` at me.
* 🕵️ **The Reality:** The file `experiments/chimera-lang/examples/genesis.chs` does not exist in the codebase at all.
* 💡 **The Fix:** Either remove the Genesis example from the README, or actually provide the `genesis.chs` file in the examples directory. Simple!

---

# Echo's DX Audit Log 🗣️

**Target:** `crates/locus/README.md`
**Date:** 2025-05-24

## 🔍 Experience - The Walkthrough

**Scenario:** "I am a new user trying to run the quickstart example for the `locus` crate."
**Action:** Try to follow the README using a fresh crate. Copied the exact example code to `src/main.rs`.

## 🚧 Stumble - The Friction Points

1.  **Missing Dependency:** The example code uses `Topology` and `Vec2` from `locus`. However, when trying to use it in an external project, `locus` is an internal workspace crate and the `README.md` lacks installation instructions.
    - *Impact:* Compilation error or confusion on how to add `locus` to `Cargo.toml`.
    - *Fix:* Provide clear `Cargo.toml` dependency instructions (e.g., `locus = { path = "../crates/locus" }`).

## 📢 Report - The Complaint

**Title:** 🗣️ Echo: Missing installation instructions

**Description:**
*   🤦 **The Confusion:** "Tried to run the `locus` example. There are no instructions on how to install it or add it to my `Cargo.toml`."
*   🕵️ **The Reality:** "Turns out I need to figure out the path to the internal crate manually because it's not on crates.io and the README doesn't tell me."
*   💡 **The Fix:** "Add a clear `Installation` section with the `Cargo.toml` snippet."

---

# Echo's DX Audit Log 🗣️

**Target:** `crates/poincare-disk/README.md`
**Date:** 2025-05-24

## 🔍 Experience - The Walkthrough

**Scenario:** "I am a new user trying to run the quickstart example for the `poincare-disk` crate."
**Action:** Try to follow the README using a fresh crate. Copied the exact example code to `src/main.rs`.

## 🚧 Stumble - The Friction Points

1.  **Missing Dependency:** The example code uses `poincare_disk`. However, when trying to use it, the `README.md` lacks installation instructions.
    - *Impact:* Compilation error or confusion on how to add `poincare-disk` to `Cargo.toml`.
    - *Fix:* Provide clear `Cargo.toml` dependency instructions (e.g., `poincare-disk = { path = "../crates/poincare-disk" }`).

## 📢 Report - The Complaint

**Title:** 🗣️ Echo: Missing installation instructions

**Description:**
*   🤦 **The Confusion:** "Tried to run the `poincare-disk` example. There are no instructions on how to install it or add it to my `Cargo.toml`."
*   🕵️ **The Reality:** "Turns out I need to figure out the path to the internal crate manually."
*   💡 **The Fix:** "Add a clear `Installation` section with the `Cargo.toml` snippet."

---

# Echo's DX Audit Log 🗣️

**Target:** `chimera-lang` Compilation
**Date:** 2025-05-24

## 🔍 Experience - The Walkthrough

**Scenario:** "I am a new user trying to run the project without default features to reduce bloat."
**Action:** `cargo run -p chimera-lang --example story_demo --no-default-features`

## 🚧 Stumble - The Friction Points

1.  **Massive Compilation Failure:** The codebase fails to compile with 30+ errors due to unconditionally referencing enums, variants (e.g. `ViewMode::Tesseract`), methods (`exec_core_op`), and fields (`prologue_state`) that are hidden behind the `nova` feature flag.
    - *Impact:* Total failure to build. Users are forced to use the bloated default features.
    - *Fix:* Ensure that internal modules correctly apply feature gates (`#[cfg(feature = "nova")]`) around usages of feature-gated items, or provide fallback implementations.

## 📢 Report - The Complaint

**Title:** 🗣️ Echo: Compilation fails entirely with --no-default-features

**Description:**
*   🤦 **The Confusion:** "Tried to compile without default features to make the binary smaller. The compiler exploded with 30+ errors about missing variants and unknown fields."
*   🕵️ **The Reality:** "Turns out the codebase is full of hardcoded references to `nova` features that aren't properly `#cfg` gated. The feature flags are broken."
*   💡 **The Fix:** "Fix the feature gates throughout `chimera-lang` (e.g., `tui::state::ViewMode`, `vm::mod::exec_core_op`, etc.) so the project compiles cleanly with `--no-default-features`."

---

# Echo's DX Audit Log 🗣️

**Target:** `crates/hyper-system/README.md`
**Date:** 2025-05-24

## 🔍 Experience - The Walkthrough

**Scenario:** "I am a new user trying to use the 4D math utilities."
**Action:** Copy and pasted the 4D Rotation & Projection example from the README.md into a new binary project.

## 🚧 Stumble - The Friction Points

1.  **Private Module Access:** The example fails to compile with `error[E0603]: module 'math' is private`.
    - *Impact:* Total failure to run the example.
    - *Cause:* The `math` module has been made `pub(crate)` but the `README.md` still instructs users to `use hyper_system::math::Vec4;`.

## 📢 Report - The Complaint

**Title:** 🗣️ Echo: Getting Started example is broken (hyper-system private module)

**Description:**
*   🤦 **The Confusion:** "Tried to run the basic 4D rotation example from `hyper-system`'s README. The compiler immediately slapped me with 'module `math` is private'."
*   🕵️ **The Reality:** "Turns out the architectural changes locked the `math` module inside the crate, making the documentation completely incorrect and the example unrunnable."
*   💡 **The Fix:** "Either update the README example to import `Vec4` correctly from the public API facade (e.g., `use hyper_system::Vec4;` if re-exported), or make the module public again if users are supposed to access it directly."
