# Bolt's Critical Learnings

## Zero-Cost Allocation in Nested Loops
**Learning:** In N-body physics simulations, computing forces creates an O(N^2) inner loop. Allocating a temporary vector for accelerations (`vec![Vec2::ZERO; bodies.len()]`) in the outer `update` function adds an O(N) heap allocation per frame, slowing down the hot path.
**Action:** Move the acceleration buffer into the object struct (e.g. adding `acc: Vec2` to the `Body` struct) or use an inline accumulator variable. Then write the final calculated value to `body.acc` before the integration step. This eliminates the heap allocation completely.

## Cleaning Up Scratchpads
**Learning:** Do not leave temporary test files like `test_borrow.rs`, `test_perf.rs`, or compiled executable binaries hanging in the repository before requesting a review.
**Action:** Always delete scratchpad files (`rm temp.rs` or simply use a more transient tool, like keeping tests inside the crate properly) before running tests or preparing for merge, otherwise PRs will be blocked.

## Distance Checks in Hot Loops
**Learning:** Checking distances between entities in an O(N^2) loop (e.g. `pos_i.distance(pos_j) < RADIUS`) introduces a costly square root operation for every comparison. This overhead dominates CPU time in physics or proximity interactions.
**Action:** Always use squared distances (`distance_squared`) and compare against a precalculated squared radius (`RADIUS * RADIUS`) when determining if entities are within a threshold.
**[TUI Render Loops and Intermediate Allocations]
**Learning:** Using `.collect::<Vec<_>>().join("\n")` inside TUI render loops causes significant unnecessary intermediate heap allocations per frame, which violates zero-cost abstraction principles.
**Action:** Replace `collect` and `join` with a pre-allocated `String` and `std::fmt::Write` loops to append directly, preventing intermediate vector allocations.
**TUI Rendering Allocation Optimization**
**Learning:** Using `.collect::<Vec<_>>()` to format and join strings inside TUI render loops causes significant unnecessary intermediate heap allocations per frame.
**Action:** Use `iter().map(...)` with `std::fmt::Write` to construct strings iteratively in a single buffer, and directly pass iterators to TUI widgets (like `List::new`) that accept them.
**[TUI Rendering String Joining Optimization]**
**Learning:** Using `.map(|v| format!("{}", v)).collect::<Vec<_>>().join(" + ")` causes an intermediate vector allocation and multiple string allocations inside TUI render loops.
**Action:** Replace `.collect::<Vec<_>>().join(" + ")` with `std::fmt::Write` loops to append directly into a mutable string buffer (`write!(&mut s, ...)`) avoiding the intermediate vector allocation completely.
