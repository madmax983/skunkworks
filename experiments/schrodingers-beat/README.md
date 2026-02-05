# Schrödinger's Beat ⚛️🥁

**Polyrhythmic Mutex Sonification**

This experiment sonifies the behavior of threads contending for shared resources (Mutexes).
Instead of a drum machine programmed by a human, the rhythm emerges from the CPU scheduler and the "fight" for locks.

## Concept
- **Threads as Drummers**: Each thread has a loop interval (BPM).
- **Locks as Instruments**: To play a beat, a thread must acquire a lock.
- **Contention as Rhythm**: If a lock is busy, the thread plays a "dissonant" sound (or misses the beat), creating phase-shifting patterns.

## How to Run
```bash
# TUI only (Silent Mode - for CI/CD or no-audio envs)
cargo run -p schrodingers-beat

# Full Experience (Audio Enabled)
cargo run -p schrodingers-beat --features audio
```
**Note:** The audio feature requires system libraries (e.g., `libasound2-dev` on Linux).

## Visuals
The TUI shows the state of each thread:
- **SLEEPING**: Waiting for next cycle.
- **TRYING**: Attempting to grab the Mutex.
- **ACQUIRED**: Holding the lock (Kick/Snare).
- **CONTESTED**: Failed to lock (Collision).

## Rhythm Logic
We use `try_lock()` to maintain a strict metronome for the *attempt*.
- If Thread A holds the lock, and Thread B tries:
    - Thread B fails (`Contention`).
    - This creates a "rest" or "ghost note" in the main beat, but adds a "tension" sound.
- If we used blocking `lock()`, Thread B would wait, effectively shifting its phase relative to Thread A. (Future experiment?)

## Moonshot Status
- **Audio**: `cpal` (Optional)
- **Visuals**: `ratatui`
- **Logic**: `crossbeam`, `std::sync::Mutex`
