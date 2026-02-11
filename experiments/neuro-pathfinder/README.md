# Neuro Pathfinder 🦀🗺️

**Lineage:** `neuro-crab` × `symphonic-terrain`

A hybrid experiment combining Spiking Neural Networks (SNN) with procedural terrain generation.

## Concept

An autonomous agent (a hexapod "crab") controlled by a biologically-inspired neural network traverses a 3D landscape generated from text.

- **The Brain:** The crab uses a Central Pattern Generator (CPG) network based on Izhikevich neurons (from `neuro-crab`).
- **The World:** The terrain is generated from noise and text using Perlin noise and font rasterization (from `symphonic-terrain`).
- **The Interaction:** The crab's legs sense the terrain height. The "drive" current to the neurons pushes the crab forward, but it must negotiate the slopes.

## Controls

- **Arrow Up/Down:** Increase/Decrease Neural Drive Current (Speed).
- **Arrow Left/Right:** Rotate Camera.
- **W/S:** Zoom Camera.

## Hybrid Traits

- **Cognitive Cartography:** The agent physically experiences the "meaning" of the text as terrain. High complexity text creates mountains that are harder to climb.
- **Sonification:** The audio engine generates sound based on the crab's height and velocity, and the neural activity of its legs.

## Lineage Details

- **Parent A (`neuro-crab`):** Provided the `network.rs`, `neuron.rs` logic, and the original 2D crab structure.
- **Parent B (`symphonic-terrain`):** Provided `heightmap.rs`, `audio.rs`, and the 3D rendering/camera setup.
- **Mutation:** The `Crab` was evolved into `Crab3D`, adding 3D kinematics and ground-clamping logic to walk on `HeightMap`.
