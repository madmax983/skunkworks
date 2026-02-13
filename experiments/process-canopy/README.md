# Process Canopy 🌳

**"Forest Competition + Resource Scheduling Simulation"**

This experiment visualizes your operating system's process table as a living forest.
Each tree represents a running process.

## The Metaphor

- **Trees**: Processes.
    - **Height/Structure**: Determined by the Process ID (DNA).
    - **Trunk Thickness (Scale)**: Determined by **Memory Usage**. (More RAM = Thicker Tree).
    - **Foliage**: Represents activity.
- **The Sun**: The CPU Scheduler.
    - The Sun moves across the sky, casting "light" (CPU cycles) onto the trees.
    - When a tree is lit, it is "Scheduled" (Active).
    - Active trees glow and have vibrant leaves.
- **The Soil**: System RAM.
- **Roots**: Represent the memory footprint anchored in the soil.

## Controls

- **Space**: Toggle Scheduler Mode.
    - **Round Robin**: The Sun moves steadily from left to right, giving equal time to all trees.
    - **Priority**: The Sun snaps to the process with the highest CPU usage, simulating a priority scheduler.

## Technical Details

- **Stack**: Rust, `macroquad` (Graphics), `sysinfo` (System Monitoring), `rand` (Procedural Generation).
- **L-Systems**: Each tree's geometry is generated using a Lindenmayer System derived from its PID hash.
- **Simulation**: The forest is rebuilt every 2 seconds to reflect the current state of the OS.

## Running

```bash
cargo run --release
```
