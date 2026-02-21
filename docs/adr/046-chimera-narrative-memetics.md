# 046. Chimera Narrative & Memetics System

## Status
Proposed

## Context
As the Chimera Prologue system evolved to include basic logic, arithmetic, and quantum operations (ADR 042), a limitation became apparent: the system lacked higher-level semantic processing capabilities. While it could simulate digital circuits and simple biological behaviors (ADR 029), it struggled to model complex information propagation, viral ideas ("Memes"), or coherent storytelling structures ("Narratives").

Experiments in `biomorphic-lexicon` and `memetic-market` demonstrated the potential for "literary automata"—systems where text strings are not just data, but active agents capable of mutation and evolution. However, the existing Prologue grid treated strings primarily as opaque values or simple instructions, missing the opportunity for rich, string-manipulation-based emergent behavior.

Furthermore, the lack of persistent, named storage (beyond local registers) limited the ability of agents to "remember" complex concepts or share a common cultural knowledge base.

## Decision
We have decided to integrate two new subsystems into the Prologue environment: **Narrative Physics** and **Memetics**.

### 1. Narrative Physics (`narrative.rs`)
This module introduces "Story Runes" that treat string values as narrative elements. It implements a form of "computational narratology" on the grid.

*   **Runes:**
    *   `α` (Alpha) - **Incipit**: Generates a seed theme (e.g., "Hero", "Shadow") based on input.
    *   `ω` (Omega) - **Terminus**: Resolves a story string into an outcome (e.g., "Victory", "Tragedy").
    *   `✍` (Hand) - **Revision**: Edits stories based on modifier inputs (Reverse, Uppercase, Cut).
    *   `📖` (Book) - **Library**: A persistent, global key-value store shared across the entire grid. This allows agents to "publish" stories and "read" classics, enabling cultural transmission.

### 2. Memetics (`memetics.rs`)
This module introduces "Viral Runes" that treat string values as evolving organisms ("Memes"). It simulates the spread and mutation of information.

*   **Runes:**
    *   `ι` (Iota) - **Source**: Extracts a meme (string) from the grid into the signal network.
    *   `ε` (Epsilon) - **Evolve**: Applies random mutations to a string (Substitution, Insertion, Deletion), simulating semantic drift.
    *   `φ` (Phi) - **Censor**: Acts as a filter, blocking memes that contain specific substrings (censorship/immune system).
    *   `σ` (Sigma) - **Spread**: Broadcasting a meme to all neighboring grid cells (viral infection).
    *   `κ` (Kappa) - **Imitate**: Copies the most complex (longest) meme from neighbors, simulating trend-following behavior.

## Consequences

### Positive
*   ** emergent Complexity:** The interaction between Narrative (structure) and Memetics (mutation) allows for the emergence of "evolving stories" and "cultural transmission" within the simulation.
*   **Global Persistence:** The `Library` provides a mechanism for long-term storage and communication between distant parts of the grid, solving the "local-only" memory constraint.
*   **Rich String Manipulation:** The new runes provide a robust set of tools for procedural text generation and manipulation directly on the grid.

### Negative
*   **State Bloat:** The `Library` is a global `HashMap` that could potentially grow unbounded if not managed (though `MAX_STRING_LEN` limits individual entry size).
*   **Nondeterminism:** Memetic mutation (`ε`) introduces randomness, making exact reproduction of simulation states harder without strict RNG seeding.
*   **Complexity:** The distinction between "Signal" (transient) and "Grid" (persistent) string manipulation becomes more critical and potentially confusing for users.

## Compliance
This decision aligns with the "Biomorphic" design philosophy of Chimera, extending biological metaphors (Evolution, Viral Spread) to the domain of information theory.
