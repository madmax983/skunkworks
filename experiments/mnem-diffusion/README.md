# Mnemonic Diffusion

**Experiment Type:** Biological Codebase Visualization / Hybrid
**Status:** [NEW HYBRID]
**Stack:** Rust, Macroquad, Gray-Scott Reaction-Diffusion

## Concept

A visual experiment resulting from crossing `mnem-rot` with `gray-scott`.

- **From mnem-rot:** The dependency graph parsing, force-directed layout, and the concept of "code rot" (entropy) measured by file health.
- **From gray-scott:** The reaction-diffusion chemical substrate `GrayScott` generating Turing patterns.
- **Novel Trait (Phenotype):** "Mnemonic Diffusion". The files of the codebase act as active chemical catalysts on the 2D substrate. The older, more rotten a file is (high entropy), the more of the `V` chemical ("fungus/decay") it injects into the environment. The result is a healthy `U` substrate being slowly consumed by biological Turing patterns growing around the legacy sections of the codebase graph.

## Controls

- **Right Click + Drag:** Pan the camera
- **Scroll Wheel:** Zoom in/out

## Running

```bash
cargo run -p mnem-diffusion
```
