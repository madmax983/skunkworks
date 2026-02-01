# Euclidean Pulse ⚛️🥁

**Genesis: The Percussionist**

A rhythm engine that generates Euclidean rhythms based on your system's CPU usage.
The harder your computer works, the more intense the groove.

## How it works

1.  **Monitor:** Polls CPU usage for each core.
2.  **Map:** Converts usage percentage (0-100%) into a number of pulses (k) over 16 steps (n) using Bjorklund's algorithm.
3.  **Synthesize:** Generates audio (Sine + Noise) for each core. Pitch is mapped to core index.
4.  **Visualize:** Displays a TUI with spinning Euclidean circles.

## Running

By default, audio is **disabled** to ensure compatibility with environments lacking ALSA/Sound drivers (like some cloud containers).

### Visual Mode (Default)
```bash
cargo run -p euclidean-pulse
```
The TUI will run, and the rhythm will "play" visually (the circles pulse red), but no sound will be produced.

### Audio Mode (Recommended)
**Requirements:** `libasound2-dev` (Linux).

```bash
cargo run -p euclidean-pulse --features audio
```
This enables the `cpal` backend. Each CPU core becomes a voice in the drum circle.

## Controls

*   `q` or `Esc`: Quit.

## The Code

*   `algo.rs`: Euclidean rhythm generation (Bjorklund's logic).
*   `monitor.rs`: System CPU polling via `sysinfo`.
*   `audio.rs`: Real-time synthesis via `cpal` (or dummy clock).
*   `tui.rs`: Visualization via `ratatui`.
