# 🗣️ Echo DX Audit Report: Chimera Prologue

## 🔍 EXPERIENCE - The Walkthrough

**Scenario:** "I am a new user trying out Chimera Prologue by copy-pasting the `Hello World Prologue` example from `README.md`."

**Action:**
1. Ran `cargo run -p chimera-lang --features nova -- --input experiments/chimera-lang/examples/mad_scientist.prl`.
2. Also tested missing file errors with `cargo run -p chimera-lang -- --input none_existing.pro`.

## 🚧 STUMBLE - The Friction Points

1. **The README Run hangs:** The command exactly as provided in the README `cargo run -p chimera-lang --features nova -- --input experiments/chimera-lang/examples/mad_scientist.prl` hangs indefinitely when executed in non-interactive shell sessions or scripts. It turns out I needed to append the `--headless` flag to bypass the TUI in such environments.
2. **Cryptic OS Errors:** When I specified a non-existent input file, the CLI crashed with a raw OS error: `Error: No such file or directory (os error 2)`. It does not tell me *which* file it failed to find or the path it was trying to access, leaving me guessing if I had a typo in the path or if something else was missing.

## 📢 REPORT - The Complaint

### Issue 1: Getting Started example hangs without `--headless`
* 🤦 **The Confusion:** "Tried to run the `Hello World Prologue` example. The terminal just froze and timed out. I thought the program was broken."
* 🕵️ **The Reality:** "Turns out the TUI waits indefinitely in non-interactive sessions, and I needed to append the `--headless` flag."
* 💡 **The Fix:** "Add a note in the README about the `--headless` flag for background runs, or better yet, make the TUI detect non-interactive sessions automatically and fallback to headless."

### Issue 2: Unhelpful Missing File Error Message
* 🤦 **The Confusion:** "I ran the command and got `Error: No such file or directory (os error 2)`. What file? My input file? A config file? The compiler?"
* 🕵️ **The Reality:** "It was my input file path that was wrong."
* 💡 **The Fix:** "Wrap raw file IO errors with context. The message should say something like `Error: Could not open file at 'none_existing.pro' - No such file or directory`."
