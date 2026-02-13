# Bard's Journal 🎻

## Philosophy
- If it isn't documented, it doesn't exist.
- A good example is worth 1,000 lines of explanation.
- Error messages are the first line of documentation—make them helpful.
- Your target audience is a tired developer at 3 AM. Be kind to them.

## Critical Learnings
(Entries will be added here as new "magic" behaviors or critical misunderstandings are discovered.)

## 2024-05-22 - Binary Crate Doc Tests
**Confusion:** Doc tests in `src/main.rs` (or modules of a binary crate) fail to compile because they cannot link against the binary itself.
**Clarification:** Use ````rust,ignore` for examples in binary crates, or structure the app as a library + thin binary wrapper if runnable examples are critical.

## 2024-05-23 - Hyperbolic to Euclidean Conversion
**Confusion:** It's easy to mistake hyperbolic radius $R$ for Euclidean radius $r$ when visualizing.
**Clarification:** In the Poincaré disk model, the conversion is $r = \tanh(R/2)$, not $\tanh(R)$. Using the wrong formula distorts the tiling near the boundary.

## 2026-02-08 - Izhikevich Random Generation
**Confusion:** The `Izhikevich::random` method seemed to return arbitrary parameters, leading to unpredictable simulation outcomes.
**Clarification:** It uses hardcoded probabilities (60% Regular Spiking, 20% Fast Spiking, 20% Chattering) to mimic cortical distribution. Documenting these probabilities is critical for reproducibility.

## 2026-05-22 - Non-Commutative Addition
**Confusion:** Users (and Bard) were confused why `mobius_add(ant, step)` didn't result in a point `|step|` away from `ant`.
**Clarification:** `mobius_add(z, a)` implements $a \oplus z$ (left translation by $a$), which is an isometry. To move "relative to `ant`" by `step`, one must calculate $ant \oplus step$, which corresponds to `mobius_add(step, ant)`. Order matters in non-Euclidean space!

## 2026-05-27 - Market Particle Physics
**Confusion:** The movement of Bids and Asks seemed inverted. Why do Bids move to index 0 (Up) if 0 is the "top"?
**Clarification:** In the `market-sim` grid, Y=0 represents the **Highest Price**.
- Bids (Buyers) start at Y=Height-1 (Lowest Price) and bubble up (index decreases) to find sellers.
- Asks (Sellers) start at Y=0 (Highest Price) and fall down (index increases) to find buyers.
Collisions happen when they meet in the middle. The documentation must make this coordinate system explicit.

## 2026-06-03 - Vec2 vs Topology Coordinates
**Confusion:** Users (and Bard) mixed up `Vec2(x, y)` (Cartesian: horizontal, vertical) and `Topology::normalize(y, x)` (Matrix: row, col). This led to off-by-one errors and incorrect wrapping behavior.
**Clarification:** `Vec2` is designed for continuous physics (X-right, Y-down/up), while `Topology` operates on discrete grid indices (Row-major: Y-down, X-right). Documentation must explicitly warn about swapping coordinates when bridging these systems.

## 2026-06-05 - Git Associates Usage
**Confusion:** Multiple experiments reimplemented git logic using fragile `std::process::Command` calls, unaware of the robust `git-associates` crate.
**Clarification:** `git-associates` provides a safe, high-level wrapper around `git2` for analyzing history and diffs. It should be the first choice for any git-related functionality to ensure consistency and performance.
