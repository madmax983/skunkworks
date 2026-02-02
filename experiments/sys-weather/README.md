# System Weather

**Genesis Experiment: The Meteorologist**

`sys-weather` is a chaotic system visualizer that uses your computer's system load (CPU and Memory) to drive the parameters of a Lorenz Attractor.

## Concept
The Lorenz Attractor is a system of three differential equations originally developed to model atmospheric convection. It is sensitive to initial conditions (The Butterfly Effect).

In this experiment:
- **CPU Usage** drives the **Prandtl number ($\sigma$)**, representing the fluid viscosity/thermal conductivity ratio. Higher CPU = Higher $\sigma$ (more turbulence/dissipation).
- **Memory Usage** drives the **Rayleigh number ($\rho$)**, representing the temperature difference driving the convection. Higher Memory = Higher $\rho$ (driving force).
- **Time** drives the evolution of the state vector $(x, y, z)$.

## Controls
- **Space**: Toggle "Live Mode" (using system stats) vs "Static Mode" (standard chaotic parameters).
- **R**: Reset the simulation state.
- **Q**: Quit.

## Tech Stack
- `ratatui`: TUI rendering.
- `sysinfo`: System metrics collection.
- `tui-shared`: Terminal setup wrapper.

## Running
```bash
cargo run -p sys-weather
```
