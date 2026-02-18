# Ripple Scheduler

**Lineage:** `canopy-scheduler` × `ripple-tank` (via `resonance-audio`)

## Concept
A visualization of Operating System scheduling algorithms where process execution creates physical disturbances in a fluid medium.

- **Processes** are sources of waves.
- **CPU Time** is the energy source.
- **Interference Patterns** reveal the temporal structure of the scheduling algorithm.

## Controls
- **Space**: Pause/Resume
- **A**: Add random process
- **S**: Switch Scheduling Algorithm (Round Robin, FCFS, Priority, SJF)
- **K**: Kill current process
- **R**: Reset simulation

## Algorithms
- **Round Robin**: Creates rhythmic, regular pulses as the quantum rotates.
- **FCFS**: Long, sustained disturbances from a single source.
- **Priority**: Higher priority processes are visualized (and sonified) with different positioning/intensity.
- **SJF**: Shortest jobs create quick bursts.

## Dependencies
- `macroquad`: Rendering
- `resonance-audio`: Wave simulation physics
