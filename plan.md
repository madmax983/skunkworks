1. **Understand current state and goal:** The goal is to act as the "Sentry" agent and increase test coverage for `crates/physics-pbd`.
2. **Current coverage:** I ran `cargo tarpaulin` and analyzed `lcov.info`. There are some uncovered lines in `physics-pbd/src/lib.rs`.
3. **Lines to cover:**
   - Lines inside `solve_distance`:
     - `DA:381,0`: return inside `if p1 >= particles.len() || p2 >= particles.len()` (wait, there was a test for out of bounds? maybe it wasn't called during step)
     - `DA:389,0`: `return` inside `if !target_len.is_finite() || !stiffness.is_finite()`
     - `DA:391,0`, `DA:393,0`, `DA:399,0`, `DA:402,0`, `DA:403,0`, `DA:408,0`, `DA:412,0`, `DA:419,0`, `DA:422,0`.
     - Line 356: `fn solve_pin(...)` is technically not getting called directly by my new tests because `Pin` constraint evaluation runs in `step`. I need to make sure iterations run and my test actually executes it. Wait, `test_solve_pin_static` calls `system.step(0.1, 1)` and it has a Pin constraint! Ah, the line numbers in `lcov.info` don't perfectly map to the `cat -n` output because I'm looking at `tail`/`head`.
4. **Let's map correctly:**
   - 356: `fn solve_pin`
   - Let's look at `tests/sentry_coverage.rs`. The new tests I wrote didn't seem to increase coverage of those lines, but they did cover some logic. Let's write more specific tests in a `#[cfg(test)] mod tests` block at the bottom of `src/lib.rs` instead, which is part of Sentry's prompt! ("Put unit tests in a `#[cfg(test)] mod tests` module at the bottom of the file.")
5. **Action Plan:**
   - Write tests directly in `crates/physics-pbd/src/lib.rs` in the `mod tests` block.
   - The uncovered lines are mostly edge cases in `solve_distance` and `solve_pin`.
   - Also, there are examples in the doc tests (`DA:17`, `DA:27`) that seem to fail coverage or maybe just the `#[test]` wrappers around doc tests aren't being tracked properly?
   - Actually, I'll add tests to the existing `mod tests` in `src/lib.rs`.
   - Edge cases to test:
     - `solve_pin` logic (calling `Self::solve_pin` directly).
     - `solve_distance` with out-of-bounds indices.
     - `solve_distance` with NaN target length/stiffness.
     - `solve_distance` with NaN particle distance.
     - `solve_distance` with zero mass particles.
     - `solve_distance` with zero distance.
     - `solve_actuator` directly.
6. **Pre-commit step:** Call `pre_commit_instructions` and follow steps.
7. **Submit:** Submit the code.
