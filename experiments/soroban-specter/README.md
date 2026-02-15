# Soroban Specter 🧮🌊

**Parents**: `experiments/soroban-market` × `experiments/fluid-specter`

A "Fluid Abacus" where the beads are not solid objects, but regions of high fluid density constrained by potential fields (the rods). The act of calculation becomes a hydrodynamic process.

## Concept

In a standard Soroban, beads slide along rods. In **Soroban Specter**, the beads are density sources in a real-time fluid simulation.
*   **Active Beads** (those pushed against the central beam) emit high-density fluid.
*   **Inactive Beads** (those pushed away) are faint.
*   **Calculation**: As numbers are added/subtracted, beads move, shifting the density sources and creating turbulence in the fluid.

## Lineage

*   **Allele A (Structure)**: `crates/soroban` (from `soroban-market`) provides the logic for the Japanese Abacus state and arithmetic.
*   **Allele B (Physics)**: `fluid.rs` (from `fluid-specter`) provides the Jos Stam Real-Time Fluid Dynamics solver.
*   **Emergent Trait**: "Hydro-computation". The visual state of the calculation is a continuous, flowing medium rather than discrete solid states.

## Controls

*   **Space**: Toggle Auto-Mode (Random additions/subtractions).
*   **Click**: Inject fluid manually (Splash).
*   **C**: Clear Soroban.
