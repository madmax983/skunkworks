# Warden's Journal

## 2024-05-24 - Unbounded Organelle Replication (DoS)
**Threat:** The `*` (Bang) operator in `process_ribosome` spawns new organelles without checking the `MAX_ORGANELLES` limit. A malicious user (or self-replicating virus) could use this to exponentially increase the number of organelles, causing memory exhaustion (DoS).
**Defense:** Added a check `if self.organelles.len() < MAX_ORGANELLES` before spawning new organelles in `process_ribosome`.

## 2024-05-25 - Integer Overflow in Topology Twisting (DoS)
**Threat:** In `crates/locus/src/lib.rs`, the `Topology::Klein` and `Topology::Mobius` variants used standard subtraction `(s - 1) - coordinate` for coordinate twisting. If the coordinate was `i64::MIN`, this caused an integer overflow panic, leading to a Denial of Service.
**Defense:** Replaced the subtraction with `.wrapping_sub()` to handle the overflow gracefully (preserving the modular arithmetic behavior).

## 2024-05-26 - Lag Switch Exploitation in Physics Simulation
**Threat:** The  neuron model in  decayed injected current by a fixed amount per update call, regardless of time step . This meant that by lowering the update rate (lagging), a user could extend the duration of an impulse significantly, potentially cheating in simulations. Additionally, negative or NaN  could cause divergence or panic.
**Defense:** Decoupled  from frame rate by using time-dependent exponential decay . Added validation for  and NaN inputs to ensure robust behavior.

## 2024-05-26 - Lag Switch Exploitation in Physics Simulation
**Threat:** The `Izhikevich` neuron model in `crates/synaptic-physics` decayed injected current by a fixed amount per update call, regardless of time step `dt`. This meant that by lowering the update rate (lagging), a user could extend the duration of an impulse significantly, potentially cheating in simulations. Additionally, negative or NaN `dt` could cause divergence or panic.
**Defense:** Decoupled `current_decay` from frame rate by using time-dependent exponential decay `0.95^dt`. Added validation for `dt <= 0.0` and NaN inputs to ensure robust behavior.
