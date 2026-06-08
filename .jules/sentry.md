**2023-10-27 - [Fix Resonance Audio Pluck Underflow]**
**Learning:** `PhysicsGrid::new` allowed creating 0x0 grids. `PhysicsGrid::pluck` then checked bounds with `x < self.width - 1`, which panics on `0 - 1` for `usize`.
**Action:** Always verify dimensions are non-zero before subtracting `1` for bounds checking in grid structures.

**2023-10-28 - [Ensure safe allocation caps for arbitrary limits]**
**Learning:** `Vec::with_capacity` using `usize::MAX` causes capacity overflow panics and OOM vulnerabilities when processing user inputs like history limits.
**Action:** When pre-allocating memory, cap the requested length against an absolute internal bound using `.min(MAX_SAFE_LIMIT)` before calling `Vec::with_capacity`.
