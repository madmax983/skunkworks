# Warden's Journal

## 2024-05-24 - Unbounded Organelle Replication (DoS)
**Threat:** The `*` (Bang) operator in `process_ribosome` spawns new organelles without checking the `MAX_ORGANELLES` limit. A malicious user (or self-replicating virus) could use this to exponentially increase the number of organelles, causing memory exhaustion (DoS).
**Defense:** Added a check `if self.organelles.len() < MAX_ORGANELLES` before spawning new organelles in `process_ribosome`.

## 2024-05-25 - Integer Overflow in Topology Twisting (DoS)
**Threat:** In `crates/locus/src/lib.rs`, the `Topology::Klein` and `Topology::Mobius` variants used standard subtraction `(s - 1) - coordinate` for coordinate twisting. If the coordinate was `i64::MIN`, this caused an integer overflow panic, leading to a Denial of Service.
**Defense:** Replaced the subtraction with `.wrapping_sub()` to handle the overflow gracefully (preserving the modular arithmetic behavior).
