# Sentry's Journal

**Floating Point Auctions**
**Learning:** Using `unwrap_or(Ordering::Equal)` when sorting `f64` can hide `NaN` logic bugs. Explicitly handling `NaN` (e.g., treating as -Infinity) ensures deterministic market clearing.
**Action:** Always use a custom comparator or `total_cmp` (if valid) for sorting floats in financial/market logic.

**Agent Spam**
**Learning:** Agents in simulations may accidentally or maliciously submit duplicate bids. Without deduplication, one agent can monopolize resources.
**Action:** Deduplicate bids by Agent ID before processing auctions.
