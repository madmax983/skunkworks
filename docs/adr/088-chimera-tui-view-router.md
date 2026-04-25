# 088. Chimera TUI View Router

Date: 2026-04-25

## Status
Accepted

## Context
The view routing logic inside the `terminal.draw` block of the monolithic `run_app` function in `experiments/chimera-lang/src/tui/app/mod.rs` was massive and bloated. The `run_app` function handled both the high-level event loop and the detailed resolution of which UI view to render, violating the Single Responsibility Principle and leading to a "God Function" structure.

## Decision
Extracted the view resolution logic into a new `router.rs` module featuring a `route_view` function. The `run_app` function now simply delegates the rendering decision to this dedicated router.

## Consequences

### Positive
*   **Separation of Concerns:** `mod.rs` remains lean and strictly dedicated to high-level event loop execution.
*   **Readability:** The view routing logic is centralized and isolated, making it easier to add new views or modify existing routing rules.
*   **Maintainability:** Reduces the cognitive load when modifying the main application loop.

### Negative
*   **Indirection:** Introduces a new abstraction layer (`router.rs`) that developers must navigate when tracing the rendering pipeline.
