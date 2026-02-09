# Bolt's Journal

**[Inlining SPH Kernels]**
**Learning:** In N^2 simulation loops, function call overhead and redundant `powi` calculations can dominate. Manual inlining and hoisting constants (like `h^9`) yields ~20% speedup.
**Action:** For hot physics loops, prefer inlining small kernel functions and precomputing all coefficients.

**[Borrow Checker & Inter-struct Mutation]**
**Learning:** Returning `Cow` from a getter on a struct (e.g., `agent.speak()`) that borrows from `self` makes it impossible to mutate another part of the same collection (e.g., `agents[idx_b]`) while holding that Cow, because Rust sees the whole `Vec` as borrowed.
**Action:** When needing to transfer data between elements of the same collection, prefer cheap clones (like `Arc` or small structs) or optimize the data structure (Array vs Map) so the lookup is cheap enough that cloning isn't the bottleneck.

**[Inter-Struct Mutation & Borrow Splitting]**
**Learning:** When iterating over a field `self.buffer` to call a method `self.action()` that takes `&mut self`, the iterator holds `&self` which conflicts with `&mut self`.
**Action:** Use an index-based loop `for i in 0..self.buffer.len()` and extract the item (copying it) to drop the borrow on `self.buffer` before calling `self.action()`.

**[Vec::with_capacity in vec![] macro]**
**Learning:** `vec![Vec::with_capacity(n); m]` creates one vector with capacity and clones it `m-1` times. `Vec::clone` does not preserve capacity (it creates a fit-to-size clone).
**Action:** Use `(0..m).map(|_| Vec::with_capacity(n)).collect()` to ensure all vectors have reserved capacity.

**[Allocation-Free Diffusion]**
**Learning:** Collecting iterators into `Vec` inside hot loops (e.g., 256 cells/frame) causes massive allocator pressure.
**Action:** Inverted loop nesting (iterate neighbors once, update multiple channels) to use stack-based arrays instead of heap allocations.

**[HashMap vs Vec for Small N]**
**Learning:** For small collections (N < 100) iterated frequently (e.g., audio rate), `Vec` beats `HashMap` due to cache locality and no hashing overhead. Linear scan for lookup is also competitive for small N.
**Action:** Replace `HashMap` with `Vec` for small, frequently iterated collections like active oscillators or particles.

**[Hoisting Command Processing]**
**Learning:** Polling atomic channels inside a hot loop (e.g., per sample) introduces significant overhead.
**Action:** Process commands once per block (e.g., every 64-512 samples) instead of per sample. The latency impact (<12ms) is usually acceptable.
