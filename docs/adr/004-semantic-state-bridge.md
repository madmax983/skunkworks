# 004. Semantic State Bridge for TUI Applications

## Status
Accepted

## Context
Terminal User Interfaces (TUIs) render visual information as a grid of characters and escape codes. While efficient for humans, this representation is opaque and brittle for automated agents, particularly Large Language Models (LLMs).

To interact with a TUI application, an external agent typically has to "scrape" the screen buffer, which loses semantic meaning (e.g., is this string of text a button, a label, or part of a drawing?). This makes building robust AI-driven testing, debugging, or autonomous agent integration difficult.

We need a mechanism to expose the *internal* state of the application—entities, properties, layout, and available actions—in a structured, machine-readable format, independent of the visual rendering.

## Decision
We decided to introduce a new workspace crate, `crates/tui-semantic`, which provides a standardized "Semantic Bridge".

Key components:
*   **Pure Data Structures:** The crate defines serializable (JSON) structs representing the application's semantic state, with no runtime dependencies on `ratatui` or `crossterm`.
*   **`Snapshot` Struct:** Represents a single frame of the application's semantic state. It includes:
    *   **Entities:** Typed objects with positions, velocities, and arbitrary properties.
    *   **Regions:** Named areas of the screen (e.g., "Inventory", "Log").
    *   **Metrics:** Global counters (Score, FPS, Turn Count).
    *   **Actions:** Available interactions (e.g., "move_north", "quit").
*   **Inherent Implementation:** Instead of enforcing a trait, applications are encouraged to implement an inherent `snapshot()` method that returns a `tui_semantic::Snapshot`. This allows for greater flexibility and avoids strict interface coupling.

## Consequences

### Positive
*   **AI Interpretability:** LLMs can reason about the game state ("I am at (10,5) and health is low") rather than pixel patterns.
*   **Decoupling:** The semantic representation is independent of the TUI library (Ratatui) or backend (Crossterm).
*   **Testability:** Allows for integration tests that assert against the logical state (e.g., "Player should have 5 items") rather than screen content.

### Negative
*   **Implementation Overhead:** Developers must explicitly implement the mapping from their internal structures to the `Snapshot` format.
*   **Data Duplication:** There is a slight memory and CPU cost to generating the snapshot, as it duplicates the live state into a serializable form.
