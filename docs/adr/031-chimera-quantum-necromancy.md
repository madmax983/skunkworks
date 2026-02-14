# 31. Chimera Quantum & Necromancy Extensions

Date: 2024-05-22

## Status

Accepted

## Context

The Chimera language initially modeled biological systems using linear execution (Strands) and spatial interaction (Grid/Organelles). However, the simulation required deeper metaphysical capabilities to model advanced phenomena:

1.  **Non-Locality**: Biological signaling (hormones) is slow. The system needed a mechanism for instantaneous, non-local state synchronization ("Action at a Distance") to simulate quantum biological effects.
2.  **Persistence**: Death was permanent (`Apoptosis` deleted the strand). To model "Genetic Memory" and cyclic history, the system needed a way to retain and recover dead code.
3.  **Probabilistic Logic**: Standard boolean logic was too rigid for modeling quantum decision making.

## Decision

We decided to extend the `Nova` feature set with two new subsystems: **Quantum Mechanics** and **Necromancy**.

### Quantum Mechanics
*   **Entanglement**: Introduced a map `entangled_pairs: HashMap<usize, usize>` in the VM.
    *   The `Entangle` opcode links two strands.
    *   Mutative operations (`Transcribe`, `Mutate`) check this map and propagate changes to the partner strand instantly.
    *   `QuantumJump` allows instantaneous teleportation of the instruction pointer to the partner strand.
*   **Superposition**: Introduced `Value::Superposition(Vec<(Value, f64)>)`.
    *   Allows a value to exist in multiple states with associated probabilities.
    *   `Collapse` and `Observe` opcodes resolve the state to a single value based on RNG.

### Necromancy
*   **Graveyard**: Introduced a `graveyard: Vec<Strand>` stack in the VM.
    *   `Apoptosis` and `Bury` move the current strand to the Graveyard instead of deleting it.
    *   `Exhume` restores the last buried strand to the live Helix.
    *   `Seance` executes the last buried strand ephemerally (Ghost Execution) without restoring it.
    *   `Reincarnate` creates a mutated clone of a strand and buries the original.

## Consequences

### Positive
*   **Complex Interaction**: Organisms can now perform "backup and restore" operations or coordinate behaviors across separate strands without direct calls.
*   **Resilience**: The Graveyard acts as a genetic undo buffer, allowing the system to recover from catastrophic mutations.
*   **Emergent Behavior**: Superposition allows for "fuzzy" logic that can adapt to changing environments better than strict conditionals.

### Negative
*   **State Complexity**: Entanglement introduces side effects to mutation logic. Modifying Strand A implicitly modifies Strand B, which can lead to hard-to-trace bugs.
*   **Non-Determinism**: Superposition relies on RNG, making the VM execution non-deterministic and harder to replay strictly (unless the RNG seed is preserved).
*   **Memory Overhead**: The Graveyard grows indefinitely if `Exhume` or `Mourn` (which generates energy from death) are not used, potentially leading to memory bloat.
