1. **Phase 1: Evaluate Previous Hybrids**
   - Use `run_in_bash_session` to compile and evaluate `git-locus` via `cargo check -p git-locus`.
   - Update `MUTATIONS.md` using python scripts to move `git-locus` from `Proposed Crosses` to `Attempted Crosses` and document the evaluation results.

2. **Phase 2: Create One New Hybrid**
   - Read `MUTATIONS.md` proposed crosses (it is empty, so we must invent a new one autonomously, or find one from proposed section).
   - *Since `Proposed Crosses` only contains `git-locus` which is being evaluated, we will invent a new hybrid*: `gray-miller` (`crates/gray-scott` + `crates/miller-lattice`).
   - Create the directory `experiments/gray-miller`.
   - Implement the hybrid by having the discrete hierarchical crystal structure from `miller-lattice` act as a static barrier or feed rate modifier on the continuous Turing pattern of `gray-scott`.
   - Update `Cargo.toml` in `experiments/gray-miller` with the correct dependencies (`gray-scott`, `miller-lattice`, `macroquad`, etc.).
   - Make sure to add `gray-miller` to the workspace `Cargo.toml` members.
   - Ensure the hybrid compiles and passes `cargo check -p gray-miller`. Include a `--headless` bypass.

3. **Update Tracking Files**
   - Update `MUTATIONS.md` to add `gray-miller` to `Attempted Crosses`. Include the parents, concept, novel trait, predicted phenotype, status, and evaluation.
   - Leave a recombination pheromone for `gray-miller` in `GUESTBOOK.md`.

4. **Run Pre-Commit Checks**
   - Call `pre_commit_instructions` to ensure proper testing, verifications, reviews and reflections are done.

5. **Submit**
   - Commit and push changes via `submit`.
