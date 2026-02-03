# Git Phasing ⚛️

**Moonshot:** Phasing patterns (Steve Reich) + File comparison (`git diff`).

A sonification experiment that treats source code as a rhythmic score. Two files (e.g., `v1.rs` and `v2.rs`) are played simultaneously as rhythmic loops.
- **Pitch:** Determined by word length (Short words = high ticks, Long words = bass).
- **Velocity:** Determined by syntax keywords (`fn`, `struct` are loud, `//` comments are quiet).
- **Phasing:** The user can introduce "drift" to one of the voices, causing the patterns to shift out of phase, creating complex polyrhythms and interference patterns similar to Steve Reich's "Piano Phase".

## Controls
- `Right Arrow`: Increase Drift (Voice 2 plays faster).
- `Left Arrow`: Decrease Drift.
- `Space`: Reset Drift to 0.
- `Q`: Quit.

## Dependencies
- `cpal` (Audio)
- `ratatui` (Visuals)

## Build
To enable audio (requires ALSA/Jack on Linux):
```bash
cargo run --features audio
```
Without audio feature, it runs in "Simulation Mode" (Visuals only).
