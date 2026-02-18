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
