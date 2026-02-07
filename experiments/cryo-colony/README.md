# Cryo-Colony ❄️🐜

**Genesis: Cryo-Colony** is a hybrid experiment simulating agents ("Thermites") that manipulate the phase state of their environment.

## Lineage 🧬

*   **Parent A**: `phase-engine` (Genesis/Crystallographer)
    *   *Inheritance*: WGPU Instanced Rendering, Langevin Dynamics, Phase Transition Physics (Solid <-> Liquid).
*   **Parent B**: `thermo-termites` (Genesis/Mycelium)
    *   *Inheritance*: Agent-based logic, modification of local environment variables (Temperature).

## Concept

Agents inhabit a crystalline lattice.
*   **Coolers (Cyan)**: Lower local temperature, freezing liquid into solid (restoring lattice structure).
*   **Heaters (Yellow)**: Raise local temperature, melting solid into liquid (allowing flow and tunneling).

The environment reacts to temperature:
*   **Solid (Blue)**: Particles are tethered to their lattice positions by strong springs.
*   **Liquid (Red)**: Particles are loosely tethered and move chaotically due to thermal noise.

## Controls

*   **Arrow Up/Down**: Increase/Decrease global ambient temperature.
*   **R**: Reset simulation.
*   **W/A/S/D/Space/Shift**: Camera movement.

## Implementation Details

*   **Simulation**: `src/simulation.rs` - Parallel update of particles and agents using `rayon`.
*   **Rendering**: `src/renderer.rs` - WGPU instanced rendering. Instances are colored based on their state (Phase) or type (Agent).
