# Warden's Journal

## 2024-05-24 - Unbounded Organelle Replication (DoS)
**Threat:** The `*` (Bang) operator in `process_ribosome` spawns new organelles without checking the `MAX_ORGANELLES` limit. A malicious user (or self-replicating virus) could use this to exponentially increase the number of organelles, causing memory exhaustion (DoS).
**Defense:** Added a check `if self.organelles.len() < MAX_ORGANELLES` before spawning new organelles in `process_ribosome`.

## 2024-05-25 - Integer Overflow in Topology Twisting (DoS)
**Threat:** In `crates/locus/src/lib.rs`, the `Topology::Klein` and `Topology::Mobius` variants used standard subtraction `(s - 1) - coordinate` for coordinate twisting. If the coordinate was `i64::MIN`, this caused an integer overflow panic, leading to a Denial of Service.
**Defense:** Replaced the subtraction with `.wrapping_sub()` to handle the overflow gracefully (preserving the modular arithmetic behavior).

## 2024-05-26 - Akashic Record Data Loss
**Threat:** The `AkashicWrite` OpCode would overwrite the entire database file if `load_records` failed (e.g. due to corruption or size limit), leading to catastrophic data loss.
**Defense:** Implemented atomic writes (write-to-temp + rename) and strict size checks in `save_records`. Refactored `load_records` to report errors instead of returning an empty map.

## 2024-05-27 - ReDoS in Nova Pattern Matching (Stack Overflow)
**Threat:** The `glob_match` function in `experiments/chimera-lang/src/vm/nova.rs` used unchecked recursion to handle `*` wildcards. A malicious pattern like `********************` (many stars) combined with a target string would cause exponential branching and/or a stack overflow, leading to a crash (DoS).
**Defense:** Replaced the recursive algorithm with an iterative `O(N*M)` implementation that splits the pattern by `*` and verifies that the target string contains the segments in order.
