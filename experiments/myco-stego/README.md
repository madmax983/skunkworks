# Myco-Stego: Biological Steganalysis 🍄🔐

**Parents**: `experiments/stego-cartridge` + `experiments/myco-transit`

**Concept**: A visualization where Slime Mold agents (Physarum Polycephalum) navigate the Least Significant Bit (LSB) landscape of an image. If the image contains hidden steganographic data (LSB=1), the agents congregate on the data, revealing its structure and density.

## How it Works

1.  **Steganography**: The image is analyzed bit by bit. Pixels with LSB=1 are treated as "Food Sources" or high-pheromone areas.
2.  **Simulation**: 5000 agents spawn and navigate the image using sensory logic. They are attracted to the hidden data.
3.  **Visualization**: The trails left by the agents are rendered in the terminal, showing the "ghost" of the hidden message.

## Usage

Run with a default generated cover image containing a hidden payload:

```bash
cargo run -p myco-stego
```

Or provide your own image (PNG recommended):

```bash
cargo run -p myco-stego -- --image path/to/image.png
```

## Lineage

-   **Stego-Cartridge**: Provided the LSB embedding/extraction logic (`stego.rs`).
-   **Myco-Transit**: Provided the agent-based simulation and TUI rendering logic (`simulation.rs`).
-   **Novel Trait**: Visualizing the invisible data layer as a biological terrain.
