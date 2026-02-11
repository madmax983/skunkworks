# Broken Mirror 🪞🔥

> "Symmetry is the silence before the storm."

A real-time GPU simulation of the **2D XY Model** and **Kosterlitz-Thouless Phase Transition**.

## Concept
The XY Model consists of a grid of 2D spins (angles).
- At High Temperature: Spins are random (Paramagnetic, Symmetry Restored).
- At Low Temperature: Spins align (Ferromagnetic, Symmetry Broken).
- Near Critical Temperature: Topological defects (Vortices and Anti-Vortices) form and unbind.

This experiment visualizes the "Symmetry Breaking" process where a chaotic system settles into ordered domains, and the "Phase Transition" where the nature of the order changes.

## Controls
- **Scroll**: Adjust Temperature (Heat/Cool).
- **Left Click**: Apply Magnetic Field (Align spins to mouse).
- **Right Click**: Inject Noise (Local heating).
- **Space**: Pause/Resume.

## Tech Stack
- **wgpu**: Compute Shaders for simulation (Double Buffered Storage Textures), Render Shaders for visualization.
- **winit**: Windowing.
