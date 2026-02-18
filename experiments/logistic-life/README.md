# Logistic Life: The War for Stability ⚛️🦠

> "Life thrives on the edge of chaos."

**Logistic Life** simulates a population of agents fighting for survival on a chaotic substrate known as a **Coupled Map Lattice (CML)**.

## The World

The world is a 2D grid where each cell evolves according to the **Logistic Map** equation:
$$ x_{n+1} = (1 - \epsilon) f(x_n) + \frac{\epsilon}{4} \sum_{neighbors} f(x_{neighbor}) $$
$$ f(x) = r x (1 - x) $$

- **$x$**: The state of the cell (0.0 to 1.0).
- **$r$**: The local growth rate. This determines the behavior:
    - $r < 3.0$: Stable fixed point.
    - $3.0 < r < 3.57$: Period doubling (Oscillations).
    - $r > 3.57$: **Chaos**.
- **$\epsilon$**: The coupling strength (diffusion).

## The Factions

Two species of agents inhabit this world, each with a different preference for stability:

1.  **🔴 Red Agents (Chaos Seekers)**:
    - Thrive in high-$r$ regions ($r > 3.5$).
    - **Terraform**: They slightly **increase** the local $r$, pushing the world towards chaos.
    - Feed on entropy.

2.  **🔵 Blue Agents (Order Seekers)**:
    - Thrive in low-$r$ regions ($r < 3.0$).
    - **Terraform**: They slightly **decrease** the local $r$, stabilizing the world.
    - Feed on order.

As agents move and reproduce, they modify their environment, creating a feedback loop between the population and the physical laws of their universe.

## Controls

| Key | Action |
| :--- | :--- |
| **Left Click** | Paint **Chaos** (Increase $r$) |
| **Right Click** | Paint **Order** (Decrease $r$) |
| **Arrow Up** | Increase Coupling ($\epsilon$) |
| **Arrow Down** | Decrease Coupling ($\epsilon$) |
| **Space** | Pause / Resume |
| **G** | Toggle **Heatmap Mode** (Visualize $r$ instead of $x$) |
| **R** | **Reset** Simulation |
| **Esc** | Quit |

## Observations

- **The Edge of Chaos**: Agents often congregate at the boundaries between stable and chaotic regions ($r \approx 3.57$).
- **Pattern Formation**: High coupling ($\epsilon$) causes the grid to synchronize, forming large waves and striations.
- **Ecological Balance**: If one faction dominates, they may make the world uninhabitable for themselves (e.g., Red agents making the world *too* chaotic even for them).

## Running

```bash
cargo run -p logistic-life
```
