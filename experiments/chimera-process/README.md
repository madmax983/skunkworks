# Chimera Process 🧬🖥️

**"The Operating System is Alive."**

A hybrid experiment combining system monitoring with artificial life.

- **Parent A**: `experiments/process-canopy` (System monitoring via `sysinfo`)
- **Parent B**: `experiments/chimera-lang` (Evolving Virtual Machine)

## Concept

Every process on your computer is now the host for a digital organism (a ChimeraVM instance).
The organism's DNA is deterministically generated from the process name.
Its metabolism is driven by the process's CPU usage.

- **High CPU**: The organism is hyper-active, executing its genome rapidly.
- **Memory**: The organism has access to memory "nutrients".
- **Grid**: The organism lives on a 16x16 grid where it can store data (or pheromones).

## The Visualization

A TUI (Terminal User Interface) displays the active processes and their symbiotic agents.

- **Green**: Healthy, active agents.
- **Red**: Hyper-active, high-energy agents (High CPU).
- **Gray**: Dormant or slow agents.

## Running

```bash
cargo run -p chimera-process
```

## Controls

- `q`: Quit

## Lineage

This experiment explores the idea of "System as Ecosystem". Instead of just viewing numbers, we view the "life" of the system.
It inherits the robust TUI capabilities of `ratatui` and the biological simulation engine of `chimera-lang`.
