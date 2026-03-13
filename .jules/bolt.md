**Memory Optimizations using slice operations**
**Learning:** Reusing existing pre-allocated data structures and using `.clone_from_slice()` or using `fill(None)` instead of allocating a fresh grid performs drastically better for hot simulation loops like game grids. Replacing `vec![vec![None; GRID_SIZE]; GRID_SIZE]` with a cached memory block that is taken, refilled and restored avoids thousands of inner-loop heap allocations.
**Action:** When working on grid systems passing state back and forth, ensure vectors are taken, filled, and reused, rather than instantiating new vectors per frame.
