**2023-10-27 - [Fix Resonance Audio Pluck Underflow]**
**Learning:** `PhysicsGrid::new` allowed creating 0x0 grids. `PhysicsGrid::pluck` then checked bounds with `x < self.width - 1`, which panics on `0 - 1` for `usize`.
**Action:** Always verify dimensions are non-zero before subtracting `1` for bounds checking in grid structures.

**2023-10-28 - [Ensure safe allocation caps for arbitrary limits]**
**Learning:** `Vec::with_capacity` using `usize::MAX` causes capacity overflow panics and OOM vulnerabilities when processing user inputs like history limits.
**Action:** When pre-allocating memory, cap the requested length against an absolute internal bound using `.min(MAX_SAFE_LIMIT)` before calling `Vec::with_capacity`.

**2023-10-29 - [Eliminate unwrap]**
**Learning:** `unwrap()` is a ticking time bomb and can be refactored into a `if let Some(x) = ...` or similar constructs to guarantee panic safety.
**Action:** Refactor `unwrap()` into safer structures like `if let` blocks or use `unwrap_or`/`unwrap_or_else` defaults to avoid panics entirely, ensuring robust code without needing `#[should_panic]` test bypasses.
