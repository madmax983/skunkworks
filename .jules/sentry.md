**2023-10-27 - [Fix Resonance Audio Pluck Underflow]**
**Learning:** `PhysicsGrid::new` allowed creating 0x0 grids. `PhysicsGrid::pluck` then checked bounds with `x < self.width - 1`, which panics on `0 - 1` for `usize`.
**Action:** Always verify dimensions are non-zero before subtracting `1` for bounds checking in grid structures.
