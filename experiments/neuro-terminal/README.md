# Neuro-Terminal 🧠

A real-time neural network visualization in the terminal.

## What is it?
Neuro-Terminal is an educational experiment that visualizes the internal state of a Neural Network as it learns to classify 2D data.

It solves the "Circle Problem": Given a set of points in 2D space, classify them as "Inside" or "Outside" a circle of radius 0.6. This is a non-linear problem requiring at least one hidden layer.

## Visuals
- **Left Pane:** Decision Boundary. The background is sampled to show what the network "thinks" about the entire space.
- **Right Pane:** The Network Graph. Visualize neurons (nodes) and weights (connections). Weights change color (Red/Green) and visibility based on their values.

## Controls
- `p`: Pause/Resume training.
- `r`: Reset the network (new random weights).
- `q`: Quit.

## Implementation Details
- Written in pure Rust.
- Uses `ratatui` for TUI.
- Implements a custom Matrix math library and Backpropagation algorithm from scratch.
- Zero external ML dependencies (no Torch, no TensorFlow).

## Run
```bash
cargo run -p neuro-terminal
```
