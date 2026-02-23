# Ferrous Quipu 🧲🧶

**Magnetic Data Textiles.**

> "The history of the empire is written in knots, and the knots pull on each other with the force of memory." - The Splice Surgeon

## 🧬 Lineage
- **Parent A**: `experiments/quipu-symphony` (The Archaeologist)
  - *Inheritance*: `Cord` data structure, Knots as significant events, vertical hanging structure.
- **Parent B**: `experiments/ferrous-graph` (The Physicist)
  - *Inheritance*: Physics engine (`Body`, `Universe`), Magnetic Platter logic, force-directed interactions.
- **Novel Trait**: **Magnetic History**. The data structure is not static. Knots act as magnetic dipoles, causing cords to sway, tangle, and repel each other based on their content.

## 🕹️ Controls
- **Q**: Quit.
- **R**: Regenerate random Quipu cords.
- **Up / Down**: Adjust playback speed (Playhead scan rate).

## 🧲 Physics
- **Gravity**: Pulls all knots downwards.
- **Magnetism**: Knots deposit magnetism onto a background "Platter". Gradients in this field push knots around.
- **Repulsion**: Knots repel each other (preventing total collapse).
- **Springs**: Cords are held together by edges between knots.

## 🎵 Sonification
- A "Playhead" scans vertically.
- When a physical knot crosses the playhead, it triggers an event (visual flash).
- Because the cords are swaying physically, the rhythm is organic and non-deterministic.

## 🏗️ Build
```bash
cargo run -p ferrous-quipu
```
