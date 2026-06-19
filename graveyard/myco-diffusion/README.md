# Myco-Diffusion 🍄

**Status:** FRESH
**Lineage:** `bio-transit` × `rhythm-diffusion`
**Concept:** Audio-reactive Slime Mold navigating a Reaction-Diffusion landscape.

## Description

This experiment simulates a population of "Commuters" (Slime Mold agents) that traverse a grid between "Home" and "Work" cities. However, the terrain they traverse is a living chemical reaction (Gray-Scott model).

- **Agents:** Forage for the active chemical 'V' and deposit it as they move.
- **Landscape:** The chemical 'V' diffuses and reacts with the substrate 'U', creating Turing patterns.
- **Rhythm:** The reaction parameters (Feed and Kill rates) are modulated by a synthetic audio rhythm, causing the landscape to "breathe" and shift, altering the agents' paths.

## Controls

- **Q**: Quit
- **A**: Toggle Agent Visibility (faint white dots)

## Technical Details

- **Engine:** Macroquad (Rendering) + Rayon (Parallel Simulation)
- **Simulation:**
  - **Agents:** 5,000 parallel agents sensing local gradients.
  - **Grid:** 300x300 Gray-Scott Reaction-Diffusion system.
  - **Audio:** Procedural synthesis driving simulation parameters (no audio output, just internal rhythm).

## Lineage

- **Parent A (bio-transit):** Provided the Agent logic (Physarum sensor model + Commuter behavior).
- **Parent B (rhythm-diffusion):** Provided the Reaction-Diffusion dynamics and the concept of audio-driven parameters.
- **Novelty:** The feedback loop where agents create the very landscape that sustains/directs them, which in turn is destabilized by the music.

## Credits

Splice Surgeon 🧬
