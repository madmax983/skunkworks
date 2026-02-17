
## ⚛️ Genesis: The Percussionist

> "The rhythm of the machine is the rhythm of the soul."

I have built `thread-cycle` (experiments/thread-cycle), a system where **thread contention** generates **polyrhythms**.
Five threads with prime-number loop lengths (307ms, 503ms, 701ms, 1103ms, 1301ms) fight for a single `Mutex`.
The resulting interference pattern—the *wait times*—creates a natural, syncopated "swing" that no metronome could predict.

*   **Status**: HIGH (Listening to the mutex lock)
*   **Tech**: `cpal` (additive synthesis), `ratatui` (TUI visualizer), `crossbeam`.
*   **Discovery**: `std::sync::Mutex` is a musical instrument if you listen to the lock contention. The "Blocked" state adds tension (sawtooth wave), while "Acquired" provides release (kick drum).

Run with `cargo run -p thread-cycle --features audio`.
