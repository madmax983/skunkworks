1. **Execute chimera-chaos**
   - Verify `chimera-chaos` is in `experiments/` and move it to `graveyard/chimera-chaos`.
   - Update `ARCHIVE.md`: move `chimera-chaos` from "Condemned" to "Executed" section.
   - Update `GUESTBOOK.md`: update the death pheromone for `chimera-chaos` to indicate it has been executed.
   - Run `cargo check --workspace` to ensure nothing breaks from its removal.
   - Commit the execution with message `⚰️ Reaper: Execute chimera-chaos`.

2. **Condemn gray-chimera**
   - Create forensic report `experiments/gray-chimera/.reaper-report.md` detailing the compilation failure (`E0277` and `E0282`).
   - Update `ARCHIVE.md`: add `gray-chimera` to the "Condemned (Awaiting Execution)" section.
   - Update `GUESTBOOK.md`: leave a death pheromone for `gray-chimera`.
   - Commit the condemnation with message `⚰️ Reaper: Condemn gray-chimera`.

3. **Complete pre-commit steps**
   - Run `pre_commit_instructions` to ensure proper testing, verification, review, and reflection are done.

4. **Submit changes**
   - Submit the PR with a relevant branch name and description.
