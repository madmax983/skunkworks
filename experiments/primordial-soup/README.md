# Primordial Soup 🍲🧬

> "In the warm little pond, with all sorts of ammonia and phosphoric salts, light, heat, electricity, etc. present..." — Charles Darwin

A TUI-based fluid simulation where the particles are living organisms.

## Concept

This experiment combines **Smoothed Particle Hydrodynamics (SPH)** with **Biological Evolution**.
The "soup" is a fluid simulation where particles flow, clump, and disperse based on physics.
However, each particle is also an organism (Algae, Grazer, or Predator) with a metabolism.

- **Algae (Green)**: Photosynthesize near the surface.
- **Grazers (Cyan)**: Move and eat Algae.
- **Predators (Red)**: Hunt Grazers.

The fluid dynamics drive the organisms together, creating "feeding frenzies" or dispersing populations, while biological rules determine survival.

## Lineage 🧬

This is a hybrid of:

- **[fluid-rain](../fluid-rain)**: Provided the SPH fluid solver and TUI physics loop.
- **[chimera-lang](../chimera-lang)**: Provided the biological concepts (Metabolism, Organelle specialization).

### Novel Trait
**Hydrodynamic Ecosystems**: Evolution driven by fluid mechanics. The environment (fluid flow) physically shapes the population clusters, rather than just being a static grid.

## Usage

```bash
cargo run -p primordial-soup
```

- **q**: Quit
