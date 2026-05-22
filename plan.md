1. *Evaluate Phase 1 notes*
   - Execute `replace_with_git_merge_diff` on `MUTATIONS.md` to append the required Phase 1 evaluation notes at the end of the `Phase 1 Evaluation` text.

2. *Create `experiments/git-gray` scaffolding*
   - Run `cargo new --bin experiments/git-gray`
   - Run `replace_with_git_merge_diff` on `experiments/git-gray/Cargo.toml` to add dependencies (`git-associates`, `gray-scott`, `ratatui`, `crossterm`, `tui-shared`, `anyhow`, `rand`).

3. *Implement `experiments/git-gray/src/main.rs`*
   - Use `write_file` to populate `experiments/git-gray/src/main.rs` with the hybrid TUI logic.
   - It will pull commit history using `GitModel::history_with_diffs`, setup a `GrayScott` grid, and drop chemical V based on the lines inserted/deleted per commit over time.
   - Include `--headless` bypass logic.

4. *Document `experiments/git-gray/README.md`*
   - Use `write_file` to create a `README.md` describing the Morphogenetic Repository History concept.

5. *Verify the hybrid compilation*
   - Run `cargo build -p git-gray` in `run_in_bash_session` to ensure it compiles correctly without errors.

6. *Update `MUTATIONS.md`*
   - Use `replace_with_git_merge_diff` on `MUTATIONS.md` to add `git-gray` under the `## 🌿 Attempted Crosses` section detailing its parents, concept, novel trait, predicted phenotype, and evaluation status.

7. *Update `GUESTBOOK.md`*
   - Use `replace_with_git_merge_diff` on `GUESTBOOK.md` to add a `[STABLE TRAIL]` entry for the Splice Surgeon crossing `git-associates` and `gray-scott`.

8. *Run tests*
   - Run `cargo test -p git-gray` and general `cargo test` to ensure there are no regressions.

9. *Complete pre-commit steps*
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

10. *Submit*
    - Commit with the message `🧬 Splice: Cross crates/git-associates × crates/gray-scott`.
