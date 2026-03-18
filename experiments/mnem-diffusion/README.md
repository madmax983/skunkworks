# Entropy Morphogenesis 🍄

**Genesis Experiment**: `mnem-diffusion`

A biological simulation visualizing codebase entropy and decay using a reaction-diffusion model.

## Concept
This experiment merges the concept of code rot (`mnem-rot`) with chemical reaction-diffusion (`gray-scott`). It visualizes "Entropy Morphogenesis"—where healthy code diffuses a nutrient chemical ('U') while rotting code diffuses a kill chemical ('V'). This creates a visual representation of how codebase decay might spread and "infect" healthy areas if left unchecked.

## The Simulation
*   **The Substrate**: A 2D reaction-diffusion grid (`Gray-Scott`).
*   **The Code Nodes**: Simulated parts of a codebase that slowly rot over time (entropy increase).
*   **The Reaction**:
    *   **Healthy Code (Cyan)** injects 'U' (Feed chemical, visualized as blue/black).
    *   **Rotting Code (Magenta)** injects 'V' (Kill chemical, visualized as red/purple).
*   **The Result**: Code decay physically bleeds and diffuses across the space, forming complex, organic Turing patterns.

## Interactivity
*   **Left Click**: "Heal" nearby code nodes, resetting their entropy to zero and stopping the spread of decay.

## Tech Stack
*   **Rust**
*   **Macroquad** for visualization (WASM-ready).
*   **Gray-Scott** reaction-diffusion from `crates/gray-scott`.

## Running
```bash
cargo run -p mnem-diffusion
```
