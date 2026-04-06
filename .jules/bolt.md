**Double Buffering Fluid Simulations**
**Learning:** In continuous simulations like fluid dynamics, updating grids often causes excessive memory allocation per frame (e.g. `chem_a.clone()`).
**Action:** Add parallel buffers (`chem_a_buf`) to the struct. Use `.copy_from_slice()` initially and `std::mem::swap(&mut buf_1, &mut buf_2)` at the end of the update step to perform zero-allocation state swaps. This saves 4 heap allocations per frame in `FluidSim`.
