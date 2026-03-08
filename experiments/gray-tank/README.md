# Gray Tank (Acoustic Morphogenesis)

**Experiment Type:** Reaction-Diffusion / Acoustic Wave Physics
**Status:** [FRESH]
**Stack:** Rust, Macroquad, Gray-Scott, Resonance Audio

## 🧬 Lineage

- **Parent A:** `experiments/ripple-tank` (2D physical acoustic wave tank simulation, wave propagation, audio synthesis)
- **Parent B:** `crates/gray-scott` (Reaction-Diffusion simulation, continuous chemical gradients, morphogenesis)

## Concept

"Acoustic Morphogenesis". This hybrid crosses the physical, energy-conserving wave propagation of a ripple tank with the biological pattern formation of the Gray-Scott model. In this setup, the 2D physical acoustic wave tank simulation creates physical displacement (pressure) that directly affects the diffusion and concentration of chemicals (U and V).

## Novel Trait: Wave-driven Turing patterns

The wave displacement advects the chemicals, meaning the standing waves or ripples create flow that physically pulls the patterns along the wavefronts, altering the morphological patterns and disrupting symmetrical diffusion. Where pressure is high, the "V" catalyst is injected; where it's low or oscillating, the pattern is physically deformed. The result is a bidirectional visual where you can pluck the wave tank to physically mold the growing Turing patterns.

## Controls

- **Left Click:** Pluck the water surface manually.
- **Middle Click:** Move the listening ear (green dot).
- **Right Click:** Add a wall to the acoustic grid.
- **Space:** Clear all waves.
- **C:** Clear all walls.
