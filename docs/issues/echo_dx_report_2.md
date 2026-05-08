# 🗣️ Echo: Getting Started and Library Usage Friction Points

## 🤦 The Confusion 1: Library Import Boilerplate
**Scenario:** "I am a new user trying to use `chimera-lang` as a library in a new project."
**Action:** Copied the exact `Cargo.toml` dependencies from the `README.md` and `test_story.rs` code block.
🕵️ **The Reality:** The `README.md` requires copying 17 dependencies including multiple local paths like `tui-shared = { path = "path/to/skunkworks/crates/tui-shared" }`. When I just wanted to use `chimera-lang`, it complained about `unused manifest key` and then failed if I didn't get all the local relative paths right. I shouldn't have to manually manage internal workspace crates just to use the language!
💡 **The Fix:** Export `chimera-lang` cleanly so users only need `chimera-lang = "0.1.0"`.

## 🤦 The Confusion 2: "os error 2" on Missing File
**Scenario:** "I ran the command with a typo in the filename."
**Action:** `cargo run -p chimera-lang -- --input non_existent.pro`
🕵️ **The Reality:** "Error: No such file or directory (os error 2)"
💡 **The Fix:** The error message should tell me *what* file was missing. "Error: Could not open file 'non_existent.pro': No such file or directory".

## 🤦 The Confusion 3: TUI Story Demo Freezes
**Scenario:** "I ran the story demo exactly as stated in the README."
**Action:** `cargo run -p chimera-lang --features nova --example story_demo`
🕵️ **The Reality:** The command timed out and froze my terminal unless I quit it manually. The `README.md` says to run it, but a demo that blocks forever is annoying when I just want to see it finish. It might be better to have an automated end or instructions on what to expect.
