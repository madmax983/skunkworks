# Bolt's Journal

## Removing dynamic vector resizing in simulations
**Learning:** Simulations often have a central `update()` loop that iterates over agents, modifying state and tracking changes in a vector. Using `Vec::new()` requires the vector to dynamically resize (reallocate memory on the heap) multiple times as elements are pushed.
**Action:** When mapping over a collection of known size inside a hot loop, initialize the accumulator with `Vec::with_capacity(known_len)` to perform a single allocation up front.

## Avoiding `std::mem::take` on self fields for mutable borrowing
**Learning:** `std::mem::take(&mut self.field)` is tempting to sidestep borrow checker conflicts when you need to pass `&mut self` to a method while iterating over one of its fields. However, if the field represents the system's state (like `self.agents`), taking it leaves the state empty during the method call. If the method ever inspects the system state, it will incorrectly see an empty state, leading to silent logical regressions.
**Action:** Clone the field or find another way to restructure the state to avoid the borrow conflict. Micro-optimizations should never compromise correctness or the invariants of the data structure.

**[Optimizing Allocations in Hybrid Iteration]**
**Learning:** In hybrid traits/systems where components copy environmental views per agent (e.g., computing forces on a torus requiring "ghost" positions for wrapping), using `.clone()` inside the N agent update loop triggers O(N) memory allocations, destroying performance.
**Action:** Pre-allocate a single buffer with `Vec::with_capacity(size)` outside the loop and reuse it via `buffer.copy_from_slice(&original)` to reduce allocations to O(1) on the hot path. Be careful to use `enumerate` where needed to avoid clippy's `needless_range_loop` warning.

**[liquidity-bridge: Removing O(W) Allocations on the Hot Path]**
**Learning:** In 2D grid simulations (like cellular automata or particle systems), stateful buffers used solely to determine scan order (e.g. `self.scan_x.clone()`) cause unnecessary per-frame allocations. If the scan order only toggles between forward and reverse, computing the index inline with a boolean flag completely eliminates the allocation.
**Action:** Always scrutinize `.clone()` inside hot paths like `update()` loops, especially for vectors. Look for ways to compute the needed state inline using boolean flags or simple arithmetic rather than allocating intermediate state vectors.

**[quantum-boids: Removing O(N) Allocations on the Hot Path]**
**Learning:** In simulations involving physical updates (like flocking algorithms), calculating fields iteratively and creating multiple `Vec` buffers via `.collect()` (e.g. `let positions = self.boids.iter().map(|b| b.position).collect();`) and allocating a new `Vec` via `self.boids.clone()` per tick causes immense heap allocation churn (O(N) allocations per frame).
**Action:** Replace `self.boids.clone()` with a persistent `self.boids_buffer` (cleared and extended each frame, then swapped using `std::mem::swap`). Pre-allocate separate persistent buffers for computed values like `positions_buffer`, `velocities_buffer`, and `forces_buffer`, clearing and refilling them on the hot path. This drops allocations per frame to zero.

**[celestial-rhythms: Removing O(N) Allocations on the Physics Hot Path]**
**Learning:** In N-body physics simulations, initializing accumulator vectors (like forces or accelerations) inside the inner `update` loop (e.g., `let mut acc = vec![Vec2::ZERO; n];`) causes excessive heap allocation churn, especially when sub-stepping is used (e.g., 4 steps per frame).
**Action:** Move the accumulator into the struct as a persistent buffer (`pub acc_buffer: Vec<Vec2>`). During the update, use `.clear()` and `.resize(n, Vec2::ZERO)` to reuse the existing capacity, dropping the per-tick allocations to zero.

**[Lensing Poetry Allocation Elimination]**
**Learning:** `vec![Vec2::ZERO; n]` allocations on a per-frame basis inside physics integrators introduce significant O(N) heap allocation overhead. Moving the intermediate acceleration buffer into the object struct itself (e.g., `acc: Vec2` on `Body`) allows the allocator to be bypassed entirely. Also, prefer `[Vec2::ZERO; n]` stack arrays in tests when size is fixed, as clippy will flag small static vecs as `useless_vec`.
**Action:** When inspecting physics hot-paths, look for temporary vectors used for acceleration accumulation and try to move them into the entity struct.
