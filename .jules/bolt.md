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
