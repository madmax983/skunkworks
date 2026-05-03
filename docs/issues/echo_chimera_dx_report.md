# 🗣️ Echo: Getting Started example is broken and errors are vague

🤦 **The Confusion:**
1. I tried to use `chimera-lang` as a library for a new project. I copy-pasted the `Cargo.toml` exactly as the README said ("You MUST explicitly include these dependencies..."). But Cargo still failed to compile, complaining about `error inheriting anyhow from workspace root manifest's workspace.dependencies.anyhow` inside `chimera-lang` itself! Adding things to *my* `Cargo.toml` didn't fix *your* crate. I am leaving.
2. I tried running the CLI with a typo in my filename (`--input bad.dna`). It just printed `Error: No such file or directory (os error 2)`. I spent 10 minutes thinking the `chimera-lang` executable was missing before realizing it meant my DNA file.

🕵️ **The Reality:**
1. `chimera-lang`'s own `Cargo.toml` uses `anyhow.workspace = true`. Path dependencies are not allowed to use workspace inheritance if they are imported by a project outside that workspace. The README's instructions to add `anyhow` to the user's project do not override the workspace resolution inside the path dependency.
2. The CLI uses `?` on the file open operation, which throws the bare OS error without saying *which* file it failed to open.

💡 **The Fix:**
1. Either stop using `workspace = true` in `chimera-lang`'s `Cargo.toml`, or update the README to explicitly tell users they MUST create a Cargo Workspace and include `chimera-lang` in it.
2. Wrap the file open error with context! e.g., "Failed to open input file 'bad.dna': No such file or directory".
