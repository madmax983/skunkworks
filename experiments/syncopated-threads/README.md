# Syncopated Threads ⚛️🥁

> "Rhythm is mathematics made audible. The Euclidean algorithm generates African bell patterns. Polyrhythms create hypnotic phase relationships. Swing is a subtle timing function." - Genesis

**Syncopated Threads** is a Moonshot experiment that generates rhythm not through a sequencer, but through the **lock contention of Operating System threads**.

## The Concept

This experiment treats `std::sync::Mutex` as a percussion instrument.
- **Kick Drum** = Mutex A
- **Snare Drum** = Mutex B
- **Hi-Hat** = Mutex C

Multiple threads ("Limbs") run in parallel, attempting to acquire these locks in polyrhythmic loops (e.g., every 500ms vs 333ms).
- When a thread acquires a lock, it "hits" the drum (triggers a sound).
- It holds the lock for a `sustain` duration.
- It releases the lock and sleeps for a `rest` duration.

**The Swing:**
When two threads vie for the same mutex (e.g., the "Jazz Player" thread trying to hog the Snare), one must wait. This blocking behavior introduces **micro-timing deviations (jitter)** naturally derived from the OS scheduler and thread contention. The rhythm "breathes" based on system load.

## Visuals (TUI)

The Terminal User Interface (TUI) visualizes the state of each thread in real-time:
- **Red (Waiting):** The thread is blocked, trying to hit the drum. This is "tension".
- **Green (Playing):** The thread holds the lock and is sounding. This is the "beat".
- **Blue (Sleeping):** The thread is resting between beats.

## Audio

Due to environmental constraints (lack of ALSA headers), this experiment creates a **WAV recording** of the session instead of live playback.
- Run the experiment.
- Watch the rhythm unfold in the terminal.
- Press `q` to quit.
- Listen to `syncopated_rhythm.wav` generated in the project root.

## Usage

```bash
cargo run --release
```

## Dependencies
- `ratatui` (TUI)
- `hound` (WAV generation)
- `crossbeam-channel` (Thread communication)
