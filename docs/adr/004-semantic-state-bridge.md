# 004. Semantic State Bridge for TUI Applications

## Status
Proposed

## Context
Terminal User Interfaces (TUIs) render visual information as a grid of characters and escape codes. While efficient for humans, this representation is opaque and brittle for automated agents, particularly Large Language Models (LLMs).

To interact with a TUI application, an external agent typically has to "scrape" the screen buffer, which loses semantic meaning (e.g., is this string of text a button, a label, or part of a drawing?). This makes building robust AI-driven testing, debugging, or autonomous agent integration difficult.

We need a mechanism to expose the *internal* state of the application—entities, properties, layout, and available actions—in a structured, machine-readable format, independent of the visual rendering.

## Decision
We decided to introduce a new workspace crate, `crates/tui-semantic`, which provides a standardized "Semantic Bridge".

Key components:
*   **`SemanticState` Trait:** An interface that TUI applications implement to export their state.
*   **`Snapshot` Struct:** A serializable (JSON) data structure representing a single frame of the application's semantic state. It includes:
    *   **Entities:** Typed objects with positions, velocities, and arbitrary properties.
    *   **Regions:** Named areas of the screen (e.g., "Inventory", "Log").
    *   **Metrics:** Global counters (Score, FPS, Turn Count).
    *   **Actions:** Available interactions (e.g., "move_north", "quit").
*   **`Command` Enum:** A standard set of instructions that the agent can send back to the application (e.g., `GetSnapshot`, `SendKey`, `InvokeAction`).

## Consequences

### Positive
*   **AI Interpretability:** LLMs can reason about the game state ("I am at (10,5) and health is low") rather than pixel patterns.
*   **Decoupling:** The semantic representation is independent of the TUI library (Ratatui) or backend (Crossterm).
*   **Testability:** Allows for integration tests that assert against the logical state (e.g., "Player should have 5 items") rather than screen content.

### Negative
*   **Implementation Overhead:** Developers must explicitly implement the `SemanticState` trait and map their internal structures to the `Snapshot` format.
*   **Data Duplication:** There is a slight memory and CPU cost to generating the snapshot, as it duplicates the live state into a serializable form.
