# 2. Git Rhythm Audio-Visual Pipeline

Date: 2024-05-21

## Status

Accepted

## Context

The `git_rhythm` experiment explores "synesthetic" code analysis by converting git history into audio and visual signals. The goal is to allow a user to "hear" and "see" the rhythm of development (e.g., commit frequency, size/churn as intensity).

We need an architecture that:
1. **Abstracts Git Complexity:** Reading git history involves complex FFI operations (via `git2`) that should not pollute the creative coding logic.
2. **Synchronizes Audio and Visuals:** The sound and the visual representation must respond to the same underlying data events (commits).
3. **Supports TDD:** The logic for synthesis and visualization should be testable without requiring a live git repository or an audio device.

## Decision

We adopt a **Pipeline Architecture** composed of three distinct stages:

1.  **Harvester (`harvester.rs`):** Responsible for interfacing with `git2`. It traverses the git history and maps raw git commits into a simplified domain model: `MusicalCommit` (containing hash, author, timestamp, churn).
2.  **Synthesizer (`synth.rs`):** A stateful audio generator. It accepts `MusicalCommit` data to modulate parameters (Frequency Modulation synthesis) and produces raw audio samples (`f32`). It is agnostic to how the audio is played.
3.  **Visualization (`vis.rs`):** A view state manager. It accepts `MusicalCommit` data for metadata display and audio samples for waveform rendering.

The `main` application loop acts as the coordinator, pulling data from the Harvester and pushing it to the Synthesizer and Visualizer.

## Consequences

**Positive:**
- **Testability:** `Synthesizer` and `VisualState` can be tested with mock `MusicalCommit` structs, as seen in the unit tests.
- **Separation of Concerns:** Audio logic (FM synthesis math) is completely separate from Git logic.
- **Portability:** The `Synthesizer` generates raw samples. This allows us to use different backends (e.g., writing to a WAV file via `hound` if real-time audio is unavailable) without changing the core logic.

**Negative:**
- **Memory Overhead:** The `Harvester` loads the relevant commit history into a `Vec<MusicalCommit>` in memory. For extremely large repositories, this might need to be converted to a streaming iterator.
- **Coupling to Main:** The coordination logic in `main` becomes critical. If the main loop blocks (e.g., TUI rendering takes too long), audio generation might stutter if it were real-time (though currently it's likely offline or buffered).
