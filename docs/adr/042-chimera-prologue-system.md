# 042. Modular Prologue Rune System

## Status
Accepted

## Context
The "Prologue" system within Chimera Lang allows for visual, grid-based logic execution, enabling the creation of digital circuits and autonomous agents alongside the main stack-based DNA execution. As the system expanded to include more rune categories (Alchemy, Quantum, Teleport, Chronos, etc.), the central dispatch logic in `mod.rs` became a bottleneck, both in terms of code maintainability and readability. A monolithic `match` statement handling dozens of rune types was no longer scalable.

## Decision
We decided to adopt a modular architecture for the Prologue system:

1.  **Domain-Specific Modules:** Rune logic is split into separate modules based on functionality (e.g., `alchemy`, `quantum`, `topology`, `virology`).
2.  **Standardized Interface:** Each module exposes an `apply_*_runes` function that takes the current grid state and returns whether a change occurred.
3.  **Central Orchestration:** The `mod.rs` module retains the core `PrologueState` struct and the `exec_prologue_tick` function, which sequentially calls the `apply` functions of the submodules.
4.  **State Management:** The `PrologueState` holds global prologue data (signals, delayed signals, registers, teleport channels), which is passed mutably to submodules as needed.

## Consequences

### Positive
*   **Maintainability:** New rune categories can be added as new modules without modifying existing logic code.
*   **Readability:** The main execution loop is a clear list of system phases rather than a massive conditional block.
*   **Extensibility:** Easier to experiment with new logic systems (e.g., "Void" or "Time Travel" runes) in isolation.

### Negative
*   **Performance:** There is a slight overhead due to iterating through multiple module dispatchers, even if no runes of that type are present (though most modules check for relevant runes first).
*   **Coupling:** Submodules still require access to the shared `ChimeraVM` or `PrologueState`, leading to long argument lists in the `apply` functions.
