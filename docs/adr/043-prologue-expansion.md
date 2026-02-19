# 043. Prologue Ecosystem Expansion

## Status
Proposed

## Context
The Prologue system, originally conceived as a simple logic gate simulator (ADR 042), has rapidly evolved to support complex simulations including Artificial Life, Environmental Physics, and Chaos Theory. This expansion introduced numerous undocumented modules (`critter`, `biolum`, `chaos`, `necromancy`, `construct`, `void`, `optics`, `list`, `evolution`) and required significant changes to the core `ChimeraVM` structure, specifically the addition of `light_grid` and `light_color_grid` for bioluminescence and `registers` in `PrologueState` for agent persistence.

These changes were made organically to support the "Chimera" vision of a multi-paradigm simulation environment, but they lack formal architectural definition and documentation, leading to potential confusion about the system's capabilities and boundaries.

## Decision
We formally recognize and document the expansion of the Prologue system into the following domains:

1.  **Artificial Life (`critter`, `evolution`):**
    *   **Critters (`C`):** Autonomous agents with genetic code, energy, and state persistence (via `PrologueState.registers`).
    *   **Evolution (`∞`):** Darwinian selectors that trigger mutations based on goal/actual comparisons.
2.  **Physics & Environment (`biolum`, `optics`):**
    *   **Bioluminescence (`Φ`, `Λ`, `Ω`):** A secondary grid layer (`light_grid`, `light_color_grid`) simulating photon intensity and color diffusion.
    *   **Optics (`\`, `/`, `-`):** Runes that manipulate light paths.
3.  **Chaos & Entropy (`chaos`, `void`):**
    *   **Chaos (`k`, `z`, `h`):** Runes introducing non-deterministic behavior and random walks.
    *   **Void (`µ`, `Ø`, `§`):** Runes for memory management and garbage collection within the grid.
4.  **Metaprogramming (`construct`, `list`):**
    *   **Construct (`B`, `Π`):** Runes for blueprinting and prototyping grid structures.
    *   **List Processing (`[`, `]`, `U`, `V`, `F`, `T`):** Higher-order functions operating on data streams.
5.  **Esoterica (`necromancy`):**
    *   **Necromancy (`†`, `‡`, `Ψ`):** Interactions with the "Graveyard" of dead code (DNA strands).

The `ChimeraVM` struct is officially extended to include `light_grid: Vec<Vec<i64>>` and `light_color_grid: Vec<Vec<(u8, u8, u8)>>` to support these features.

## Consequences

### Positive
*   **Richness:** The Prologue system now supports a vast array of emergent behaviors, from simple logic circuits to complex ecosystems.
*   **Modularity:** The new modules follow the pattern established in ADR 042, keeping the codebase organized despite the increased complexity.
*   **Transparency:** Formalizing these modules clarifies the system's capabilities for future developers and users.

### Negative
*   **Complexity:** The Prologue execution cycle (`exec_prologue_tick`) is now significantly more complex, iterating through many more potential systems.
*   **Memory Footprint:** The addition of `light_grid` and `light_color_grid` increases the memory usage of every `ChimeraVM` instance, even if bioluminescence is not used.
*   **State Management:** Persistence of agent state in `PrologueState.registers` requires careful management to prevent memory leaks (e.g., dead agents leaving state behind).
