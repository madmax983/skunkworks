# Chaotic Defense ⚛️🦋

**"Harvest the Chaos"**

A Tower Defense game where enemy behavior and spawn rates are driven by the [Logistic Map](https://en.wikipedia.org/wiki/Logistic_map):

$$ x_{n+1} = r \cdot x_n \cdot (1 - x_n) $$

## The Concept

The world is governed by a chaotic parameter $r$.
- **Low $r$ ($< 1$)**: Population dies out.
- **Medium $r$ ($1 < r < 3$)**: Population stabilizes.
- **High $r$ ($> 3.57$)**: Chaos ensues. The population fluctuates wildly and unpredictably.

Your goal is to survive. You need enemies to harvest resources (they drop resources when killed), but if the population grows too large or chaotic, you will be overwhelmed.

### Mechanics
- **Nests**: Spawn enemies based on their local chaotic population $x$.
- **Global R**: You can influence the global growth rate $r$, which slowly pulls all nests towards it.
- **Towers**: Consume resources to build. They increase "Industrialization", which slightly raises global $r$.
- **Enemies**: Their movement is jittered by their internal chaotic state.

## Controls

- `Arrow Left/Right`: Decrease/Increase Global $r$.
- `WASD`: Move Cursor.
- `Space`: Build Tower (Cost: 20 Resources).
- `q`: Quit.

## Visualization

- **Top View**: The game grid.
  - `M` (Magenta): Nests.
  - `C` (Cyan): Towers.
  - `R` (Red): Enemies.
  - `X` (Yellow): Cursor.
- **Middle View**: Bifurcation Monitor. traces the history of the population $x$ of the first nest. Watch how it changes from a flat line (stable) to oscillation (period doubling) to noise (chaos) as $r$ increases.
- **Bottom View**: Status Bar.

## Running

```bash
cargo run --bin chaotic-defense
```

## The Math

The system uses the Logistic Map to generate deterministic chaos. No random number generators are used for the core mechanics. The "random" movement of enemies is derived from their chaotic internal state.
