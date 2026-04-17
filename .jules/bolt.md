**Double Buffering Fluid Simulations**
**Learning:** In continuous simulations like fluid dynamics, updating grids often causes excessive memory allocation per frame (e.g. `chem_a.clone()`).
**Action:** Add parallel buffers (`chem_a_buf`) to the struct. Use `.copy_from_slice()` initially and `std::mem::swap(&mut buf_1, &mut buf_2)` at the end of the update step to perform zero-allocation state swaps. This saves 4 heap allocations per frame in `FluidSim`.

**[Optimized hyperbolic-dungeon path cloning]**
**Learning:** `Vec` cloning in hot loops like game update and render tree traversal leads to numerous heap allocations and bottlenecks. Methods traversing recursive spatial structures (like Poincaré disk rendering) are especially prone to this.
**Action:** Replaced owned `Vec<usize>` arguments with slice references `&[usize]`. Changed API to build new paths only when extending (e.g. `get_canonical_step` allocating only when returning a new modified path) instead of blindly returning owned Vecs on each recursive step or lookup. Uses `Vec::with_capacity` when extending the path to minimize reallocation.

**[Semantic Telemetry Overheads]**
**Learning:** Using `HashMap` with the default `SipHasher` inside frequently serialized semantic payloads (like `Entity` and `Snapshot`) incurs heavy instantiation overhead and hashing latency for small collections (e.g. `props` or `metrics` holding 1-5 keys). Additionally, dynamic `Vec::new()` calls when constructing `Snapshot`s causes multiple heap reallocations.
**Action:** Default to `BTreeMap` over `HashMap` for small mappings to bypass SipHasher overhead and guarantee deterministic serialization, and always use `Vec::with_capacity` when struct construction sizes can be reasonably anticipated.

**[Optimized DirEntry sorting allocations]**
**Learning:** When sorting a collection of `std::fs::DirEntry` elements using `.file_name()`, `sort_by_key` calls the key extraction function O(N log N) times. Since `DirEntry::file_name()` allocates an `OsString`, this results in O(N log N) hidden heap allocations.
**Action:** Use `sort_by_cached_key` instead of `sort_by_key` whenever the key extraction method is expensive or allocating. This caches the keys and evaluates the extraction exactly O(N) times.

**[neuro-sim] Hoisting zero-delay spikes to avoid Vec::push heap allocations**
**Learning:** During Spiking Neural Network (SNN) simulations in `neuro-sim`, the majority of network connections may be immediate (delay=0). Pushing these immediate spikes into the `spikes_in_transit` vector and extracting them immediately via `retain_mut` causes expensive O(n) heap allocations (growing the `Vec`) every step. By checking for `delay == 0` when generating spikes and routing them directly to the `weight_to_add` accumulator, we avoid heap allocations entirely for the most common synapse type, dramatically reducing the per-frame allocation load without any change to network behaviour.
**Action:** Always check if a queue/transit data structure can be bypassed for immediate values (delay/timer = 0) to save the allocation overhead of pushing to `Vec`.

**[Eliminate Per-Frame Rayon Vector Allocations]**
**Learning:** Hot loops using Rayon `par_iter().map().collect::<Vec<_>>()` will constantly trigger heap allocations that kill framerate in simulations. Rayon provides `collect_into_vec(&mut vec)` specifically for reusing an existing capacity without dropping it.
**Action:** Always pre-allocate memory outside hot game/simulation loops with `Vec::with_capacity` and re-use the vector by clearing it and collecting into it. `collect_into_vec` replaces the whole vector so `deposits.clear()` works great with it.

**[Optimize Vec capacity in git-mycelium]**
**Learning:** `Vec::new()` requires numerous re-allocations when adding elements, especially for inner loops.
**Action:** Switch to `Vec::with_capacity` when creating temporary vecs, mapping collections, or filtering based on collections where the upper bound of the length is known.
**[Chunk-based Grid Iteration]**
**Learning:** Nested `for y ... for x` loops with explicit array index access (`self.cells[idx]`) in Rust result in per-element bounds checks when iterating over a flat 1D vector representing a 2D grid. Iterating via `chunks_exact_mut(width)` is roughly 2x faster in hot loops as it eliminates these bounds checks and allows clean per-row logic (e.g. tracking `y` without division).
**Action:** Use `chunks_exact_mut` or `chunks_mut` for 2D grid iterations over flat 1D vectors instead of nested `for` loops.

**[ExactSizeIterator `.collect()`]**
**Learning:** Calling `.collect()` on an `ExactSizeIterator` automatically pre-allocates the optimal vector capacity. Replacing it with a manual `Vec::with_capacity(len)` followed by a `for` loop provides zero performance benefit and only adds verbosity.
**Action:** Trust `.collect()` for iterators with a known length (e.g. `commit.parents()`).
**Bolt Optimization: Loop Unrolling and Branchless Bounds Checking in compute_laplacian**
**Learning:** `rem_euclid` (modulo arithmetic) inside the hot 3x3 convolution loop of the Gray-Scott simulation caused significant performance overhead, requiring 18 modulo operations per cell per tick.
**Action:** Unroll the 3x3 convolution loop into explicit neighbor calculations with pre-computed branchless boundary checks (`left`, `right`, `up`, `down`). This provides a ~2x performance speedup on the simulation's hottest path and avoids redundant math operations.
