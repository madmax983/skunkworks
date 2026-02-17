# Thread Cycle ⚛️🥁

> "Rhythm is mathematics made audible."

`thread-cycle` is a generative rhythm engine powered by **thread contention**.
Multiple threads run in polymetric loops (prime number intervals), attempting to acquire a shared `Mutex`.
The resulting interference patterns create a syncopated, evolving rhythm.

## Concept

Each thread is a "percussionist":
1.  **Rest**: Sleep for the remainder of its cycle.
2.  **Attempt Lock**: Try to acquire the shared resource. (Sound: *Click*)
3.  **Blocked**: If the resource is taken, wait. (Sound: *Buzz/Sawtooth*)
4.  **Acquire**: Lock acquired. (Sound: *Kick*)
5.  **Work**: Hold the lock for a fixed duration. (Sound: *Pluck/Drone*)
6.  **Release**: Release the lock. (Sound: *Snare*)

The "Groove" emerges from the **latency** introduced when threads block each other. A perfect polymeter becomes a "swung" beat due to resource starvation.

## Usage

### Run with Audio (Recommended)
Requires `cpal` and system audio libraries (ALSA on Linux).

```bash
cargo run -p thread-cycle --features audio
```

### Run Visualization Only (Headless/CI)
```bash
cargo run -p thread-cycle
```

## Controls

*   `q` or `Esc`: Quit

## Tech Stack

*   **Audio**: `cpal` for real-time synthesis (additive synthesis + noise).
*   **Visuals**: `ratatui` for TUI thread state visualization.
*   **Concurrency**: `std::thread`, `crossbeam-channel` for event bus.
