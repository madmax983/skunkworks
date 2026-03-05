# 🗣️ Echo: Getting Started example is broken

🤦 **The Confusion:** Tried to run the "Library Usage" and "Running ChimeraScript from Rust" code examples from `experiments/chimera-lang/README.md`. The compiler threw multiple errors:
1. `unresolved import chimera_lang::compiler`
2. `use of undeclared type ChimeraVM`
3. `no function or associated item named default found for struct chimera_lang::ast::Dna`

🕵️ **The Reality:** The provided code snippets in the README do not compile out-of-the-box. The `chimera_lang::compiler` module is either not public or doesn't exist, `ChimeraVM` isn't imported by `prelude::*` properly, and `Dna::default()` doesn't exist.

💡 **The Fix:** Fix the code examples in the README to use the correct module paths and initializers that actually work on the current API (e.g., `Dna { helix: Helix { strands: vec![] }, evolution_config: None }`). Make sure `ChimeraVM` and the `compiler` are actually accessible from the crate root or prelude!
