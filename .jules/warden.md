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

## 2025-01-27 - Locus Division by Zero Panic (DoS)
**Threat:** `Topology::normalize` panicked due to division by zero (via `rem_euclid`) when `width` or `height` were 0. This is a DoS vector if dimensions are user-controlled (e.g. terminal resize).
**Defense:** Added explicit checks for `width == 0 || height == 0` at the start of `normalize`, returning `None`.

## 2025-02-18 - Bio-Transit Grid Undefined Behavior
**Threat:** `TrailMap::diffuse_and_decay` used `unsafe { *self.grid.get_unchecked(...) }` inside a parallel loop. Since `TrailMap` fields are public, a user could truncate `grid` independently of `width` and `height`, causing the unchecked access to read out of bounds (UB).
**Defense:** Replaced the `unsafe` block with standard safe indexing. This turns the potential UB into a safe panic if invariants are violated.

## 2025-05-27 - Unbounded IPC File Read (DoS)
**Threat:** `vm::ipc::receive` used `fs::read_to_string` on user-controlled files in the Ether directory without a size limit. An attacker could create a massive file (e.g., 10GB) causing OOM/DoS when the VM attempts to read it.
**Defense:** Replaced `fs::read_to_string` with `File::open(...).take(1024 * 1024).read_to_string(...)`. This caps the read at 1MB. Files exceeding this limit (or truncated JSON) are deleted and an error is logged.
