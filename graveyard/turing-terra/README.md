# Turing Terra

**Genesis: The Alchemist ⚛️**

> "The mountains are not made of stone, they are waves of reaction-diffusion frozen in time."

## Overview

`turing-terra` is a moonshot experiment that uses **Gray-Scott Reaction-Diffusion** patterns to simulate the generation of terrain and biomes. It runs on the GPU using `wgpu` Compute Shaders to achieve high-performance simulation of a 1024x1024 grid.

The simulation maps the chemical concentration of "Activator" (V) to a biome palette:
- **Deep Ocean** (Low V)
- **Shallow Water**
- **Sand**
- **Grass**
- **Forest**
- **Rock**
- **Snow** (High V)

## Controls

- **Left Click**: Raise Land (Add Activator V). Create islands and mountains.
- **Right Click**: Lower Land (Remove Activator V). Dig oceans and moats.
- **Arrow Up/Down**: Increase/Decrease **Feed** rate.
- **Arrow Right/Left**: Increase/Decrease **Kill** rate.

## The Theory

By manipulating the Feed and Kill rates, you shift the system between different "Regimes":
- **Islands**: Stable spots of high concentration (Mountains).
- **Labyrinths**: Connecting stripes (Ridges).
- **Chaos**: Unstable turbulence (Storms).
- **Solitons**: Moving localized waves.

## Technical Details

- **Stack**: Rust, `wgpu` (Compute Shaders), `winit`.
- **Simulation**: Gray-Scott Model with Toroidal boundary conditions.
- **Rendering**: Custom Fragment Shader mapping concentration to hex colors.
