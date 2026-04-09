**Double Buffering Fluid Simulations**
**Learning:** In continuous simulations like fluid dynamics, updating grids often causes excessive memory allocation per frame (e.g. `chem_a.clone()`).
**Action:** Add parallel buffers (`chem_a_buf`) to the struct. Use `.copy_from_slice()` initially and `std::mem::swap(&mut buf_1, &mut buf_2)` at the end of the update step to perform zero-allocation state swaps. This saves 4 heap allocations per frame in `FluidSim`.
**[Optimized hyperbolic-dungeon path cloning]**
**Learning:** `Vec` cloning in hot loops like game update and render tree traversal leads to numerous heap allocations and bottlenecks. Methods traversing recursive spatial structures (like Poincaré disk rendering) are especially prone to this.
**Action:** Replaced owned `Vec<usize>` arguments with slice references `&[usize]`. Changed API to build new paths only when extending (e.g. `get_canonical_step` allocating only when returning a new modified path) instead of blindly returning owned Vecs on each recursive step or lookup. Uses `Vec::with_capacity` when extending the path to minimize reallocation.

**[Semantic Telemetry Overheads]**
**Learning:** Using `HashMap` with the default `SipHasher` inside frequently serialized semantic payloads (like `Entity` and `Snapshot`) incurs heavy instantiation overhead and hashing latency for small collections (e.g. `props` or `metrics` holding 1-5 keys). Additionally, dynamic `Vec::new()` calls when constructing `Snapshot`s causes multiple heap reallocations.
**Action:** Default to `BTreeMap` over `HashMap` for small mappings to bypass SipHasher overhead and guarantee deterministic serialization, and always use `Vec::with_capacity` when struct construction sizes can be reasonably anticipated.
