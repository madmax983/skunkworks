# 075. Consolidate Synaptic Physics into Neuro Sim

## Status
Accepted

## Context
Previously, the `synaptic-physics` crate was extracted (ADR 027) to encapsulate the low-level Izhikevich neuron model. Later, the `neuro-sim` crate was introduced (ADR 043) to provide a higher-level network simulation layer (managing synapses, spike propagation, and network topology). However, keeping `synaptic-physics` as a separate crate introduced unnecessary dependency overhead and fragmentation, as almost all consumers of the Izhikevich model also required the network simulation capabilities provided by `neuro-sim`.

## Decision
We decided to consolidate the `synaptic-physics` logic directly into the `neuro-sim` crate as a submodule (`crates/neuro-sim/src/physics.rs`). The standalone `crates/synaptic-physics` crate has been removed.

## Consequences

### Positive
*   **Reduced Fragmentation:** Simplifies the workspace dependency graph by removing a narrowly-focused crate.
*   **Higher Cohesion:** The core mathematical model (`Izhikevich`) now lives alongside the network topology logic (`Network`, `Synapse`) that orchestrates it.
*   **Simplified Imports:** Consumers only need to depend on `neuro-sim` to access both the low-level neuron model and the high-level network structures.

### Negative
*   **Loss of Granularity:** Projects that only wanted to calculate the Izhikevich equations without using the `Network` structure must now pull in the entire `neuro-sim` crate. However, in practice, this use case is rare.
