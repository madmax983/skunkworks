# Reaction Monitor ⚛️

**Genesis: The Alchemist**

> "Your CPU is the feed rate. Your RAM is the kill rate. The pattern you see is the heartbeat of your machine."

## 🔬 Experiment Analysis

This experiment bridges the gap between **System Monitoring** and **Reaction-Diffusion Mathematics**. Instead of a boring graph or gauge, it visualizes your system's load as a living, breathing Turing Pattern.

### Concept

The Gray-Scott reaction-diffusion model simulates two chemicals, $U$ and $V$.
- $U$ is fed into the system at rate $f$.
- $V$ consumes $U$ and converts it to more $V$ (reaction), but $V$ also dies at rate $k$.

In this Moonshot:
- **Feed Rate ($f$)** is driven by **CPU Usage**.
  - High CPU = High Feed = More energy, more chaos, more "life".
- **Kill Rate ($k$)** is driven by **RAM Usage**.
  - High RAM = High Kill = More damping, constraints, "death".

### The Visual Language

- **Stable Spots/Stripes:** Balanced system load.
- **Chaotic Waves:** High activity.
- **Solid Color / Empty:** Extreme imbalance (System Idle or Overload).

### Controls

- **Left Click:** Inject Catalyst (Chemical V) at the cursor. Paint the void.
- **Escape:** Quit.

## 🛠️ Technical Details

- **Stack:** Rust, `wgpu` (Compute Shaders), `winit`, `sysinfo`.
- **Simulation:**
  - 1024x1024 Grid.
  - Ping-Pong Texture Double Buffering.
  - Compute Shader Laplacian Convolution (9-point stencil).
  - 16x16 Workgroups.
- **Mapping:**
  - $f(cpu) = 0.035 + (cpu \%) \times 0.04$
  - $k(ram) = 0.045 + (ram \%) \times 0.04$

## 🔮 Future

- **Audio Reactive:** Modulate diffusion rates ($D_u, D_v$) with audio FFT.
- **Process Spatialization:** Map specific processes to coordinates on the grid.
