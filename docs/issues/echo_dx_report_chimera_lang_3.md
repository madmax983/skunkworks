# 🗣️ Echo: `chimera-lang` CLI and Library DX Friction Points

**Scenario:** I am a new user trying to use `chimera-lang` based on the public docs/examples.

## 🚧 STUMBLE - The Friction Points:

1. **The "README Run" Failure (Library Boilerplate):**
   *   🤦 **The Confusion:** The README says that to use `chimera-lang` as a library, I *must* explicitly include several dependencies. I created a new project and copied the `Cargo.toml` example. However, Cargo failed immediately with an error about inheriting `anyhow` from the workspace root manifest's `workspace.dependencies.anyhow`.
   *   🕵️ **The Reality:** Turns out `chimera-lang/Cargo.toml` relies on `.workspace = true` configurations (like `anyhow.workspace = true`). These paths break when the crate is used as an external path dependency outside the original workspace.
   *   💡 **The Fix:** Remove `workspace = true` declarations in `chimera-lang`'s `Cargo.toml` and replace them with specific versions (e.g., `anyhow = "1.0"`).

2. **The "Error Check" Confusion (Missing File Error):**
   *   🤦 **The Confusion:** When I ran the command with a file that doesn't exist, I received the error `Error: No such file or directory (os error 2)`. It didn't tell me *which* file was missing. If I had a complex setup or ran an internal script, I wouldn't know what failed to open.
   *   🕵️ **The Reality:** The CLI passes through a bare OS error from `fs::File::open` without providing contextual metadata.
   *   💡 **The Fix:** Wrap the file opening logic with contextual error messages (e.g., "Failed to open input file: <filename>") using a tool like `anyhow::Context`.

3. **The Feature Guard Failure (`--no-default-features` Compilation):**
   *   🤦 **The Confusion:** I tried compiling the CLI without the default `nova` feature using `cargo check -p chimera-lang --no-default-features`. The entire compilation failed with errors like `no variant or associated item named 'HyperMul' found for enum 'OpCode'` and similar errors for `HyperDiv`.
   *   🕵️ **The Reality:** The `prolouge_compiler.rs` file matches on these `OpCode` variants even when the `nova` feature (which defines them) is disabled, causing features to bleed.
   *   💡 **The Fix:** Add `#[cfg(feature = "nova")]` guards around the code paths (like the match arms in `prolouge_compiler.rs`) that depend on these variants.
