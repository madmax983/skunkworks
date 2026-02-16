# Myco-Reaper 🍄💀

**"Fungal decomposition + Garbage collection algorithms"**

A visualization of memory management as a fungal ecosystem.

## Concept
The "Heap" is a forest floor.
- **Nodes**: Organic matter (leaves, twigs).
- **Roots**: Eternal trees that sustain the ecosystem.
- **References**: Mycelial threads connecting matter.
- **Garbage Collection**: The Reaper fungus that decomposes dead matter.

## Modes
1. **Manual**: You control the seasons.
   - `Space`: Run a full Mark-and-Sweep cycle.
   - `M`: Mark phase (Spores travel from roots).
   - `S`: Sweep phase (Decay consumes unmarked nodes).
2. **Reference Counting (RC)**: The fast-acting mold.
   - Instantly kills nodes with zero incoming connections.
   - Cannot detect cycles (Dead islands).
   - Press `C` to create a cycle and see it survive RC!
3. **Mark & Sweep (Seasons)**: The slow, rhythmic decay.
   - Automatically cycles through Spring (Growth), Summer (Mark), and Winter (Sweep).
   - Handles cycles correctly.

## Controls
- `R`: Cycle GC Mode (Manual -> RC -> MarkSweep).
- `A`: Toggle Auto-Allocation.
- `C`: Create a detached reference cycle (Circular Garbage).
- `Space`: Manual Mark+Sweep.
- `M`: Manual Mark.
- `S`: Manual Sweep.

## Running
```bash
cargo run -p myco-reaper
```
