# Lock-Step ⚛️🥁

**Genesis: The Percussionist**

> "Rhythm is mathematics made audible. Polyrhythms create hypnotic phase relationships. Swing is a subtle timing function."

## Overview

`lock-step` is a rhythmic experiment that uses **thread contention** as a creative constraint.
Multiple "Musician" threads run on different polyrhythmic loops (5/4, 7/4, 3/4) and compete for a single shared resource (the "Stage" Mutex).

- **Success (`try_lock()` ok):** The thread plays its primary sound (Kick, Snare, Hat).
- **Failure (`try_lock()` err):** The thread plays a "ghost note" (click/noise) or is silenced, creating syncopation.

The result is a self-organizing drum circle where the groove is determined by the Operating System's scheduler and the mathematical phase relationships of the loops.

## Usage

Run with audio enabled (requires `alsa-sys` on Linux):

```bash
cargo run -p lock-step --features audio
```

Run in "Silent/Text-Only" mode (CI-safe):

```bash
cargo run -p lock-step
```

## Technical Details

- **Language:** Rust
- **Audio:** `cpal` + custom synthesis (Sine/Square/Noise + ADSR Envelopes)
- **Concurrency:** `std::thread`, `std::sync::Mutex`, `crossbeam::channel`
- **Visuals:** Console output only (The Percussionist forbids graphics).

## The Rhythm

- **Kick:** 800ms period (50Hz Sine)
- **Snare:** 1200ms period (200Hz Square + Noise)
- **Hat:** 300ms period (Noise)
- **Chaos:** Random interval (Drag/Swing)
