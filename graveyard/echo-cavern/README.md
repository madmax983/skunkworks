# Echo Cavern ⚛️🔊

**"The Room You Can Hear The Shape Of"**

A Moonshot experiment combining **Wave Equation Physics** and **Codebase Sonification**.
This tool generates a physical acoustic model of your codebase, where directories are caverns and files are pillars.
It then simulates sound waves propagating through this structure in real-time using a Finite Difference Time Domain (FDTD) solver.

## Controls

*   **WASD**: Move the "Ear" (Listener).
*   **Arrows**: Move the "Source".
*   **Mouse**: Move Source (Cursor).
*   **Left Click / Space**: Ping (Pluck the air).
*   **Enter**: Toggle Continuous Drone (Oscillator).

## How it works

1.  **Layout**: The file system is scanned and mapped to a 2D grid using a recursive subdivision algorithm.
2.  **Physics**: `resonance-audio` simulates the 2D Wave Equation.
    *   `.rs` files are hard walls (reflective).
    *   `.md` / `.txt` files are soft (absorbent/fast).
    *   Binaries are void (absorbent).
3.  **Audio**: The pressure at the listener's position is streamed to your speakers via `cpal`.
4.  **Visuals**: `macroquad` renders the pressure field as a heatmap overlaying the cavern map.

## Obsession
"A piano doesn't play recordings—it vibrates. Every room has a resonant frequency. Sound bends around corners, reflects off walls, creates standing waves."
This experiment lets you *hear* the architecture of your code.
