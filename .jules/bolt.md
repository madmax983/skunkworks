**[ExactSizeIterator `.collect()`]**
**Learning:** Calling `.collect()` on an `ExactSizeIterator` automatically pre-allocates the optimal vector capacity. Replacing it with a manual `Vec::with_capacity(len)` followed by a `for` loop provides zero performance benefit and only adds verbosity.
**Action:** Trust `.collect()` for iterators with a known length (e.g. `commit.parents()`).

**Bolt Optimization: Loop Unrolling and Branchless Bounds Checking in compute_laplacian**
**Learning:** `rem_euclid` (modulo arithmetic) inside the hot 3x3 convolution loop of the Gray-Scott simulation caused significant performance overhead, requiring 18 modulo operations per cell per tick.
**Action:** Unroll the 3x3 convolution loop into explicit neighbor calculations with pre-computed branchless boundary checks (`left`, `right`, `up`, `down`). This provides a ~2x performance speedup on the simulation's hottest path and avoids redundant math operations.

**Bolt Optimization: Rayon chunk-based iteration in Gray-Scott**
**Learning:** Using `par_iter_mut()` and `enumerate()` combined with division and modulo arithmetic (`i % w`, `i / w`) per pixel inside the hottest loop of a simulation significantly degrades parallel performance by introducing branching and math instructions into the critical path.
**Action:** Use `par_chunks_exact_mut` or `par_chunks_mut` for 2D parallel grid iterations. This allows iterating row by row, naturally providing the `y` coordinate and avoiding expensive modulo/division operations while preserving the bounds-check elision benefits of chunking.

**[Chunk-based Grid Iteration for Multi-Array Stencils]**
**Learning:** In FDTD or 2D grid simulations (like `resonance-audio` wave equation solvers), iterating over multiple arrays (`u`, `u_prev`, `u_next`, etc.) with a flat 1D index (`let idx = y * w + x`) forces the compiler to perform explicit bounds checks on every read and write inside the hot loop. Furthermore, the `y * w` multiplication happens repeatedly.
**Action:** Replace nested `for y ... for x` loops over explicit indices with iterator zipping using `chunks_exact(w)` or `chunks_exact_mut(w)`. By zipping all needed grid rows together into a single tuple and iterating `1..w-1` over the yielded row slices, bounds checks are elided and math overhead is reduced while preserving safe access to adjacent neighbor elements within the same or neighboring rows.

**Bolt Optimization: Removing intermediate iterator allocations**
**Learning:** `collect::<Vec<_>>()` forces an intermediate heap allocation when constructing enums or structs that internally just collect again or require the final vector representation immediately.
**Action:** Where `Vec`s are mapped over directly to create struct variants (e.g., `GrammarRule::Sequence(iter.collect())`), inline the `collect` into the constructor to avoid an intermediate bound variable map pass.
