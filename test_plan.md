1. **Fix Root README.md jargon**
   - Add a plain-English summary to `README.md` right below the title explaining that "Chimera is a visual programming language and simulation environment".
2. **Fix `experiments/chimera-lang/README.md` enzymes type documentation**
   - Explicitly document the expected type for each argument in `splice` and other enzymes.
3. **Fix `experiments/neuro-physics/README.md` quick start command**
   - Update `cargo run -p neuro-physics --headless` to `cargo run -p neuro-physics -- --headless`.
4. **Fix `crates/ferrous-core/README.md` quick start example**
   - Wrap the rust code in `fn main() { ... }` block.
5. **Fix `graveyard/git_rhythm/README.md` private module**
   - Update the import in the example from `use git_rhythm::nova::NarrativeGenerator;` to `use git_rhythm::NarrativeGenerator;`.
6. **Fix `experiments/chimera-lang/examples/story_demo.rs` headless mode**
   - Add a check for `--headless` in `story_demo.rs` using `std::env::args()` and avoid launching the TUI if the flag is present, or just print a message and exit early like we do in macroquad tests.
7. **Fix `crates/arthropod/README.md` missing macroquad dependency**
   - Add `macroquad = "0.4"` to the `[dependencies]` in the `Installation` section of the README.
8. **Fix `experiments/quipu-market/README.md` running instructions**
   - Provide a visual example and explicitly write out `cargo run -p quipu-market` (it's already there but verify).
9. **Pre-commit and Submit**
   - Complete pre-commit checks and submit.
