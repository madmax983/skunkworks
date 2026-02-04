# 7. Ghost Input Replay System

Date: 2024-10-24

## Status

Accepted

## Context

Testing Terminal User Interface (TUI) applications is notoriously difficult because user input is ephemeral, timing-dependent, and hard to reproduce programmatically. Standard unit tests cannot easily cover complex interaction sequences (e.g., "press 'j' three times, wait 500ms, then press 'enter'").

Furthermore, `crossterm` events do not implement `serde::Serialize` or `serde::Deserialize`, making it impossible to save input sessions to disk for later analysis or replay without a custom wrapper.

We needed a way to:
1.  Record user sessions (demos, bug reports).
2.  Replay sessions deterministically for regression testing.
3.  Simulate input for "attract mode" or automated tutorials.

## Decision

We implemented a **Ghost Input System** in the `tui-shared` crate, guarded by a `ghost` feature flag.

The core components are:

1.  **`GhostEvent`**: A serializable enum that mirrors `crossterm::event::Event`. It implements `From<Event>` and `Into<Event>`.
2.  **`GhostRecorder`**: A struct that captures events with timestamps relative to a start time and saves them to a JSON format.
3.  **`GhostReplayer`**: A struct that loads a JSON recording and plays back events, respecting the original timing intervals.

The system relies on `serde` and `serde_json` for persistence.

## Consequences

### Positive
*   **Deterministic Testing:** We can now write integration tests that drive the UI through a recorded sequence of inputs and assert the final state.
*   **Debuggability:** Users encountering crashes can potentially share a recording file to reproduce the issue exactly.
*   **Documentation:** We can generate GIF-like terminal demos by replaying recorded sessions.

### Negative
*   **Maintenance Overhead:** Every time `crossterm` adds a new event type or key code, we must update `GhostEvent` to match.
*   **Performance:** There is a slight overhead in converting between `Event` and `GhostEvent` during recording/replay, though this is negligible for human-speed input.
*   **Serialization Size:** Raw JSON recordings can be verbose.
