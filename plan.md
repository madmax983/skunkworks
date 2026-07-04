1. **Remove `self.scan_x.clone()` in `liquidity-bridge/src/market.rs`**
   - The `update` function in `experiments/liquidity-bridge/src/market.rs` clones the entire `self.scan_x` vector every frame.
   - We can replace the clone with a boolean flag representing the direction, or just loop over the existing `self.scan_x` without cloning since it doesn't need to be cloned (the order of iterations can just read from `self.scan_x` without mutable borrow conflicts, wait, `self.cells` is being mutated. Wait, we can't borrow `self.scan_x` immutably while mutating `self`? No, we can, as long as we only borrow `self.scan_x` or do we have an issue?).
   - Let's check if we can borrow `self.scan_x` while mutating `self.cells`. If not, we can just replace the entire logic with a boolean flag `reverse_scan` because `scan_x` is only either `0..width` or `(0..width).rev()`.

2. **Implement boolean flag approach**
   - Determine if the scan should be reversed.
   - Update `Pass 1: Bids (Up)` to loop `for i in 0..self.width`, where `x = if reverse_scan { self.width - 1 - i } else { i };`.
   - Do the same for `Pass 2: Asks (Down)`.
   - Remove `self.scan_x` entirely from the `Grid` struct, as we don't need to store it anymore! This saves even more memory.

3. **Verify tests pass**
   - Run `cargo clippy --all-targets --all-features -- -D warnings` and `cargo test`.
   - Run `cargo fmt --all`.

4. **Complete pre commit steps**
   - Complete pre commit steps to make sure proper testing, verifications, reviews and reflections are done.

5. **Submit the change**
   - Branch: `bolt-liquidity-bridge-opt`
   - Title: `⚡ Bolt: Optimize market scan allocations in liquidity-bridge`
   - Description: "Replaced `self.scan_x.clone()` with a simple boolean flag, removing O(W) vector allocations per frame and reducing memory footprint by removing `scan_x` from `Grid`."
