# Chimera Canopy 🌳🧬

**"OS Ecology: Parasitic Code Simulation"**

This experiment visualizes your operating system's process table as a living forest, inhabited by **ChimeraVM** agents that feed on CPU cycles.

## The Metaphor

- **Trees**: Processes.
    - **Height/Structure**: Determined by the Process ID (DNA).
    - **Trunk Thickness**: Determined by Memory Usage.
- **The Sun**: The CPU Scheduler.
    - Moves across the sky, casting "light" (CPU cycles) onto the trees.
    - Active processes glow.
- **Parasites**: ChimeraVM Agents.
    - **Blue Dots**: Living code organisms.
    - **Behavior**: They attach to process trees to feed on CPU cycles.
    - **Movement**: They jump between trees to find better resources or escape dying processes.
    - **Life Cycle**: They gain energy when the Sun shines on their host. They die if they run out of energy or fall off the map.

## Technical Details

- **Parents**: `process-canopy` (Visualization) + `chimera-lang` (VM Logic).
- **Stack**: Rust, `macroquad` (Graphics), `sysinfo` (System Monitoring), `chimera-lang`.
- **Hybrid Vigor**: Combines real-time system monitoring with an artificial life simulation. The environment is not procedural noise, but the *actual state of your computer*.

## Running

```bash
cargo run --release -p chimera-canopy
```

## Controls

- **Space**: Toggle Scheduler Mode (Round Robin / Priority).
