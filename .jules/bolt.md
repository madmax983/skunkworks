# Bolt's Critical Learnings

## Zero-Cost Allocation in Nested Loops
**Learning:** In N-body physics simulations, computing forces creates an O(N^2) inner loop. Allocating a temporary vector for accelerations (`vec![Vec2::ZERO; bodies.len()]`) in the outer `update` function adds an O(N) heap allocation per frame, slowing down the hot path.
**Action:** Move the acceleration buffer into the object struct (e.g. adding `acc: Vec2` to the `Body` struct) or use an inline accumulator variable. Then write the final calculated value to `body.acc` before the integration step. This eliminates the heap allocation completely.

## Cleaning Up Scratchpads
**Learning:** Do not leave temporary test files like `test_borrow.rs`, `test_perf.rs`, or compiled executable binaries hanging in the repository before requesting a review.
**Action:** Always delete scratchpad files (`rm temp.rs` or simply use a more transient tool, like keeping tests inside the crate properly) before running tests or preparing for merge, otherwise PRs will be blocked.
**[Eliminating Per-Frame O(N) Allocations in N-Body Sim]**
**Learning:** In gravitational simulations, allocating an intermediate `vec![Vec2::ZERO; bodies.len()]` inside the `update` loop creates an O(N) heap allocation on the hot path, causing significant performance overhead.
**Action:** Instead of dynamic allocation, add an `acc` field to the `Body` struct itself, and iteratively compute the values into a local variable before updating the struct array, thereby completely bypassing the heap allocation without fighting the borrow checker.

**[Bypassing Redundant Sqrt in Gravitational Math]**
**Learning:** A standard gravitational force direction is often written as `let dist = dist_sq.sqrt(); let dir = r / dist; ... acc += dir * (G * mass / dist_sq)`. This can be algebraically simplified to factor out the `dir` division by multiplying directly with `r`.
**Action:** Replace `(r / dist) * (G * mass / dist_sq)` with `r * (G * mass / (dist_sq * dist))`. This saves a division operation and a vector scaling operation per iteration.
