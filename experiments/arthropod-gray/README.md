# Arthropod-Gray Hybrid 🧬

A hybrid experiment combining `arthropod` and `gray-scott`.

## Lineage

This experiment is a cross between the immediate mode UI elements of `arthropod` and the continuous reaction-diffusion Turing patterns of `gray-scott`.
It enables manual intervention in a continuous biological morphogenetic growth simulation.

- **From arthropod (Parent A):** The immediate mode GUI widgets (`Button`), handling user interaction and real-time state toggling, and X11 headless bypass geometry safety.
- **From gray-scott (Parent B):** The continuous FDTD scalar fields `u` and `v`, computing complex spatial thermodynamic Turing patterns.
- **Novel Trait:** Interactive Chemical Morphogenesis. The discrete GUI allows real-time shifts in the underlying `feed` and `kill` chemical equilibrium rules, drastically altering the continuous morphological phenotype (switching from mitotic cell division to static coral growth dynamically).

## Execution

To run visually (requires X11/Wayland):
```bash
cargo run -p arthropod-gray
```

To run in headless CI environments:
```bash
cargo run -p arthropod-gray -- --headless
```
