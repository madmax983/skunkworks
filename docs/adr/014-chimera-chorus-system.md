# 14. Chimera Chorus System (Acoustic Magic)

Date: 2024-10-26

## Status

Accepted

## Context

The `chimera-lang` experiment models a biological system where "Genes" (OpCodes) drive local metabolic actions (Move, Eat, Divide). However, we identified a need for a higher-order mechanism to trigger global state changes—akin to "spells" or "incantations" in fantasy settings.

Standard biological evolution is slow and local. We wanted a way for an organism to discover specific "sequences of power" that could drastically alter its fate (e.g., instant energy, spontaneous generation, or mass destruction) without complex metabolic infrastructure. This introduces a layer of "Acoustic Magic" to the simulation.

## Decision

We implemented the **Chorus System** within the `nova` feature set. This system introduces an auditory buffer to the VM that listens for resonance patterns.

### 1. The Chorus Buffer
*   The VM maintains a `chorus_buffer: VecDeque<String>` with a fixed capacity (currently 8).
*   This buffer acts as a sliding window of the most recent "notes" sung by the organism.

### 2. New OpCodes
*   **`OpCode::Sing`**: Pops a string from the stack (the "Note") and pushes it into the `chorus_buffer`. Immediately triggers a check for valid Chords. Consumes Energy.
*   **`OpCode::Listen`**: Reads the current state of the `chorus_buffer` and pushes it onto the stack as a `Value::Junction`, allowing the organism to inspect its own resonance.

### 3. The Chord Protocols
After every `Sing` operation, the VM checks the tail of the buffer for specific sequences. If a match is found, a global effect triggers, and the buffer is cleared.

| Chord Name | Sequence | Effect |
| :--- | :--- | :--- |
| **Vitality** | `["Mi", "Re", "Do"]` | **Energy Restore**: Adds +50 Energy to the VM. |
| **Genesis** | `["Do", "Mi", "Sol"]` | **Spontaneous Generation**: Spawns a new `Worker` organelle at a random location. |
| **Apocalypse** | `["La", "Sol", "Fa", "Mi", "Re", "Do"]` | **Culling**: Kills one random organelle. |
| **Transmute** | `["Lead", "Gold"]` | **Alchemy**: Converts all grid cells containing "Lead" into "Gold". |

## Consequences

**Positive:**
*   **Emergent Gameplay**: DNA strands can now contain hidden "spells" that trigger powerful effects if executed in the right order.
*   **Global Agency**: Organisms can affect the entire system state (e.g., `Genesis`) without needing physical access to specific grid locations.
*   **Resonance Mechanic**: Creates a dependency on *history* (the sequence of operations) rather than just immediate inputs.

**Negative:**
*   **Implicit State**: The `chorus_buffer` is a hidden state variable. Debugging why a spell failed requires tracking the buffer history.
*   **Concurrency Risks**: If multiple organelles `Sing` in the same tick (in a future parallel implementation), they might interrupt each other's chords, creating "dissonance."
*   **Performance**: String comparisons in `check_chorus_chords` happen on every `Sing` operation, adding a slight overhead.
