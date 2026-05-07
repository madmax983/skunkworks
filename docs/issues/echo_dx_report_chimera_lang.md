# 🗣️ Echo: Getting Started with chimera-lang is a friction nightmare

**Scenario:** I am a new user trying to use `chimera-lang` based on the public docs/examples.

## 🚧 STUMBLE - The Friction Points:

1. **The Error Check (Missing File Error):**
   *   🤦 **The Confusion:** When I accidentally ran the command with a file that doesn't exist (`cargo run -p chimera-lang --headless --input non_existent_file.prl`), I got `Error: No such file or directory (os error 2)`. It didn't even tell me *which* file was missing. If I was running a complex script, I'd have no idea what failed.
   *   🕵️ **The Reality:** Turns out I misspelled the filename, but the CLI didn't tell me what it was trying to open, just gave a generic OS error.
   *   💡 **The Fix:** The CLI should provide a contextual error message (e.g., "Failed to read file: non_existent_file.prl") instead of a bare OS error.

2. **The Import Scan (Library Setup):**
   *   🤦 **The Confusion:** The README says that to use `chimera-lang` as a library, I *must* explicitly include 5 other local workspace crates (`tui-shared`, `locus`, `resonance-audio`, `hyper-system`, `poincare-disk`) in my own `Cargo.toml`. Why does a programming language VM need a Poincare Disk or Audio dependency by default?
   *   🕵️ **The Reality:** Turns out the core language is heavily coupled with its visualizations and sound effects via workspace path dependencies.
   *   💡 **The Fix:** Decouple the core language execution from the TUI/Audio/Math visualization crates. A user who just wants to run code headless shouldn't need a massive dependency tree.

3. **The Slang Check (Jargon Overload):**
   *   🤦 **The Confusion:** The documentation is overloaded with dense biological/alchemical lore. Terms like "Petri Dish", "strand", "membrane", "osmosis", and "incubate" are used to describe basic programming concepts like grids, arrays, loops, and file reading. It's confusing to map these concepts when just trying to understand the API.
   *   🕵️ **The Reality:** Turns out "Petri Dish" is just a 16x16 grid, and "strand" is just a list of instructions.
   *   💡 **The Fix:** Keep the fun lore, but provide plain-English equivalents next to them (e.g., "Petri Dish (2D Memory Grid)").

4. **The README Run (story_demo):**
   *   🤦 **The Confusion:** The README explicitly says to run `cargo run -p chimera-lang --features nova --example story_demo`. It launches a blank interactive TUI and says "Incubating narrative...". If I was expecting a terminal output, I'd be completely lost.
   *   🕵️ **The Reality:** Turns out it's an interactive UI application, but the text before the code block doesn't warn me about this.
   *   💡 **The Fix:** Mention that this is an interactive TUI demo in the text right above the code block.
