# QuipuNet 🧬

**Lineage:**
- **Parent A:** `experiments/quipu-cradle` (Quipu Data Structure & Serialization)
- **Parent B:** `experiments/hive-mind-dependencies` (Dependency Graph Topology)

## Concept

QuipuNet implements a Peer-to-Peer network simulation where data packets are physical Incan Quipus.
Nodes exchange strings encoded as knots on cords.

## Novel Trait: Physical Serialization

Bandwidth is limited by complexity. The speed of a packet traveling through the network is inversely proportional to the number of knots it carries (`1.0 / sqrt(knots)`).
Heavier data moves slower.

## Controls

- `Space`: Spawn a random packet with a random "message" (encoded as a number).
- `q`: Quit.

## Visualization

- **Left Panel:** The Network Graph. Nodes are white dots. Packets are colored dots (Green = Fast/Light, Red = Slow/Heavy) moving along edges.
- **Right Panel:** The Packet Inspector. Shows the Quipu structure of the latest packet in ASCII art.
