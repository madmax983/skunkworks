# Stego Attack

**Hybrid**: `stego-cartridge` x `locust-ddos`

A visualization where an image "cartridge" dissolves into a swarm of agents that attack a target defined in the image's steganographic data.

## Concept
The image is not just a static asset; it contains:
1.  **Visual Data**: The starting position and color of the agents (pixels).
2.  **Hidden Data**: The attack configuration (Target coordinates, Speed, Dissolve rate) embedded in the LSBs.

When loaded, the image dissolves as the pixels become agents and fly towards the target.

## Usage

### Run Demo
```bash
cargo run -p stego-attack
```

### Generate Custom Attack
```bash
# Generate a noise image (or use your own)
cargo run -p stego-attack --bin gen_noise -- --output my_input.png

# Embed attack config
cargo run -p stego-attack --bin generator -- \
    --input my_input.png \
    --output my_attack.png \
    --target-x 0.8 \
    --target-y 0.2 \
    --speed 5.0 \
    --dissolve 0.05

# Run attack
cargo run -p stego-attack -- my_attack.png
```

## Lineage
-   **Parent A**: `stego-cartridge` (Steganography, Image as Cartridge)
-   **Parent B**: `locust-ddos` (Swarm Intelligence, Attack Simulation)
-   **Novel Trait**: Steganographic Command & Control. The medium destroys itself to execute the command.
