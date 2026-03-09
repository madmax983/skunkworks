# Sentry's Journal

## [Division Panic Handling]
**Learning:** Checking for division by zero (`b == 0`) is insufficient to prevent panics in Rust integer division. `i64::MIN / -1` also causes a panic due to overflow.
**Action:** Always test both `x / 0` and `MIN / -1` when implementing integer division.

## [ChimeraVM Stack Order]
**Learning:** In `ChimeraVM`, the stack is a `Vec<Value>`. `push` adds to the end (top). `pop` removes from the end (top). When OpCodes describe arguments like "stack: A, B (top)", `B` is the last element pushed and the first popped. This means code should `push(A); push(B);` to match that state.
**Action:** When writing tests that manually construct stack states via `OpCode::Push`, ensure the push order matches the documented stack expectation (bottom to top).

## [Property Testing without Deps]
**Learning:** For math-heavy libraries (`poincare-disk`), property tests (isometry, associativity) are crucial. A simple LCG (Linear Congruential Generator) allows generating deterministic "random" test data without adding heavy dependencies like `proptest` or `rand`.
**Action:** Use a helper `pseudo_random_points` LCG function for testing mathematical invariants in dependency-constrained environments.

## [Ballistics Integer Overflow]
**Learning:** Distance calculations using `i64` coordinates like `dx*dx + dy*dy` can easily overflow if inputs are large, causing a panic in debug mode or incorrect wrapping in release mode. The VM handles `Value::Int` (i64), so inputs can be `i64::MAX`.
**Action:** Always cast integer coordinates to `f64` *before* performing squaring or distance calculations if the result is intended to be a float or large magnitude. `((dx as f64).powi(2) + (dy as f64).powi(2)).sqrt()` is safe.

## [Rust Float Clamping Bug]
**Learning:** `f64::min(a, b)` returns `a` if `b` is `NaN` (and vice versa). This means `NaN.min(0.99)` returns `0.99`, effectively masking NaN errors in clamping logic.
**Action:** Always check `.is_nan()` explicitly before using `min`/`max` for clamping, or verify which value is returned.

**[Grid Wrapping Bug]
**Learning:** In 1D vectors representing 2D grids, `x >= width` checks are critical. Relying solely on `index < vec.len()` allows "scanline wrapping" where `(width, y)` writes to `(0, y+1)`.
**Action:** Always verify `x < width` and `y < height` explicitly before calculating the 1D index.

**[Topology Quirks in Testing]**
**Learning:** `ChimeraVM` defaults to `Topology::Torus`, which makes "out of bounds" testing tricky for coordinate-based logic like `exec_splash`. Tests relying on boundaries must explicitly set `vm.topology = Topology::Plane`.
**Action:** When testing grid operations, always check the default topology and override it if boundary conditions are being tested.

**[NaN Propagation in PBD]**
**Learning:** In Position Based Dynamics, a single `NaN` particle position can infect the entire system because `delta.length()` returns `NaN`, and `NaN < EPSILON` is false, bypassing zero-division checks.
**Action:** Always check `!len.is_finite()` in distance constraints to prevent NaN propagation.

**[TUI Fractional Block Logic]**
**Learning:** When mapping a continuous value (0.0-1.0) to discrete block characters (1/8, 1/4, etc.), using strictly less `<` comparisons causes off-by-one errors for exact values like 0.5 (4/8), pushing them to the next bucket (5/8).
**Action:** Use `<=` for threshold comparisons when mapping continuous ranges to discrete steps where the boundary value belongs to the lower bucket (e.g., 0.5 should be HALF, not FIVE_EIGHTHS).

## [Git Timestamp Panic]
**Learning:** Using `unwrap()` on `chrono::Utc.timestamp_opt()` with a 64-bit git commit timestamp can panic if the timestamp is out-of-bounds, causing history parsing to crash.
**Action:** Always use `.single().unwrap_or_else(...)` or a safe fallback when parsing external timestamp data.

**[Hash Trait Stack Overflow on Deeply Nested Values]**
**Learning:** Implementing `Hash` using recursion on heavily-nested recursive types like `Value::Junction` and `Value::Superposition` is dangerous. When these inputs are passed in, standard recursion easily exhausts the stack, leading to immediate program abortion (`fatal runtime error: stack overflow`).
**Action:** Always rewrite `std::hash::Hash` implementations iteratively using a `Vec` as a stack for deeply nested recursive variants to ensure stable operations and prevent stack overflow panics.

**[Loop Bounds Underflow Panic]**
**Learning:** Iterating over interior grid elements using `1..w-1` causes an underflow panic when the grid width or height is 0 or 1, since unsigned `usize` subtraction will wrap around to `usize::MAX`.
**Action:** Always use `.saturating_sub(1)` or an explicit boundary guard (e.g., `if w < 3 || h < 3`) before establishing loop bounds on `usize` variables.

**[TUI Shared Code Coverage Improvement]**
**Learning:** cargo-llvm-cov reveals significant gaps in testing for seemingly simple modules like enum mapping arrays, UI component animations, and implicit Drop trait implementations. Also, `clippy` catches tricky things like `f32::consts::PI` representations embedded in tests! When mapping arrays like fractions for a progress bar, loop-based tests are more thorough.
**Action:** Next time, always start with `cargo llvm-cov` to establish a baseline, add tests iteratively to cover all branches, and verify with `clippy --all-targets --all-features` to ensure tests themselves are idiomatically correct before considering the task complete.
**2024-03-08 - Testing Missing Git Objects in git2**
**Learning:** To simulate a missing tree object in git to test error fallback paths (e.g. failing to compute a diff for a commit), you can compute the object path manually via `.git/objects/<first_2_hex>/<rest_of_hex>` and delete it using `std::fs::remove_file`. Be aware that standard `.unwrap()` chains on `git2` functions might then panic, so ensure the system under test handles `Err` safely.
**Action:** Use manual object deletion in a temporary `git2::Repository` to test robustness and fallback logic for corrupted or sparse repositories without panicking the test suite.
**[Coverage: Nested Struct Display]
**Learning:** Display formatting logic for nested structs (e.g. tree structures like `Cord` with `subsidiaries`) often lacks complete line coverage if tests only assert on single items or empty lists. Testing with at least two items ensures the `idx < len - 1` inter-item formatting paths (like adding blank lines between them) are executed.
**Action:** Always write a test case with two or more nested elements when verifying display output for recursive or hierarchical structs to ensure separator logic is covered.
**2023-11-20 - Unreachable Code in Math Normalization**
**Learning:** Some floating-point boundary checks in vector limits (e.g. `sq_len.is_finite()` but `sq_len.sqrt() == 0.0`) are impossible to reach via typical operations due to `f32` underflow/overflow bounds (the `else { Self::zero() }` on `limit`).
**Action:** When Sentry encounters mathematically unreachable `else` branches in strict floating-point comparisons (`> 0.0`), document them in the PR instead of wasting hours trying to find an impossible `NaN` injection path.
