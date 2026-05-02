# 🗣️ Echo: Getting Started example is broken

🤦 **The Confusion:**
Tried to run the `my-chimera-project` from the "Library Usage" section in the README. Cargo instantly failed with `error inheriting anyhow from workspace root manifest's workspace.dependencies.anyhow` because my new project wasn't part of the workspace. Then, when I tried to run `chimera-lang` with `--no-default-features`, the entire crate failed to compile due to missing `OpCode::HyperMul` and other `nova` feature guards bleeding into the main compiler.

🕵️ **The Reality:**
The `chimera-lang` crate heavily relies on `workspace.dependencies` (like `anyhow`, `ratatui`, `tui-shared`) in its `Cargo.toml`. This makes it impossible to literally copy-paste the README example into a standalone project and use it via `path` without setting up an entire empty workspace wrapper. Furthermore, disabling the default `nova` feature breaks the build entirely because the conditional compilation (`#[cfg(feature = "nova")]`) is incomplete and missing in several files like `prolouge_compiler.rs` and `tui/state.rs`.

💡 **The Fix:**
Add a huge banner to the README saying "THIS CRATE CANNOT BE USED AS A PATH DEPENDENCY OUTSIDE A WORKSPACE". Better yet, remove `workspace = true` dependencies in `chimera-lang/Cargo.toml` so the example actually works! Also, fix the broken `--no-default-features` compilation by wrapping the `HyperMul`, `HyperDiv`, and `ZipWith` opcodes properly.