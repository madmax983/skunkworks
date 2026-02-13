# Gaze Attractor 👁️

> "The act of observing a chaotic system changes it."

**Gaze Attractor** is a hybrid experiment combining `chaos-pendulum` (Double Pendulum Physics) and `foveated-code` (Eye/Gaze Simulation).

It simulates a "fabric" of connected nodes (a dependency graph or coupled pendulums) that exhibits chaotic behavior. A simulated "Eye" tracks the most energetic parts of the system (smooth pursuit).

## The Observer Effect

The user interacts with the system through the "Gaze".
- **Stabilize Mode (Cyan)**: The gaze acts as a damper, calming the chaos in the foveated region.
- **Excite Mode (Magenta)**: The gaze adds energy (kicks) to the nodes it looks at.

## Lineage

- **Parent A**: `experiments/chaos-pendulum` (Physics Engine, Verlet Integration)
- **Parent B**: `experiments/foveated-code` (Eye Tracking, Foveal Logic)
- **Novel Trait**: **The Observer Effect**. Physics dependent on attention.

## Controls

- `q`: Quit
- `m`: Toggle Mode (Stabilize <-> Excite)

## Phenotype

The system visualizes the tension between "Entropy" (Natural Chaos) and "Attention" (The Observer). When you look at the chaos, does it calm down or explode?
