# 58. Chimera Procedural Botany

Date: 2025-05-15

## Status

Accepted

## Context

The ChimeraVM operates on a grid, but creating complex, self-similar, and organic structures (like plants, fractals, or "alien" growths) using raw, imperative OpCodes (e.g., `GWrite`, `Jump`) is tedious and verbose.

We wanted to enable the organism to generate sophisticated geometry from compact DNA instructions, similar to how biological DNA encodes the blueprints for complex organisms. Specifically, we needed a way to describe branching structures and recursive patterns.

## Decision

We implemented a **Lindenmayer System (L-System)** interpreter as a specialized `Seed` Organelle, integrated into the Nova feature set.

### 1. The Seed Organelle
The `Plant` OpCode spawns a new `Seed` Organelle. This organelle is unique: instead of executing standard DNA, it interprets a specific **Axiom** (initial string) and a set of **Production Rules** (string mapping).

### 2. Recursive Expansion
The `Seed` Organelle iteratively expands its axiom. For example, a rule `F -> F[+F]F[-F]F` replaces every `F` with the branching pattern. This allows a small initial string to grow into a massive, complex structure over several generations.

### 3. Turtle Graphics Interpretation
The expanded string is interpreted as a set of turtle graphics commands:
*   `F`, `G`: Move forward and draw a segment (e.g., stem).
*   `+`, `-`: Rotate the turtle (branching angle).
*   `[`, `]`: Push/Pop the turtle state (position + angle), enabling branching.
*   `L`: Draw a leaf/flower.
*   `~`: "Wiggle" (random deviation), simulating organic growth.
*   `♪`: "Sing" (emit a musical note), enabling sonic interactions.

### 4. Grid Interaction
The turtle draws directly onto the VM's grid. If a cell is occupied, the growth stops or diverts, allowing the plant to navigate obstacles or interact with other organisms.

## Consequences

### Positive
*   **Compact Representation:** Complex geometry (fractals, trees) can be encoded in a few bytes of DNA string data.
*   **Organic Aesthetic:** L-Systems naturally produce life-like branching structures, enhancing the biological metaphor.
*   **Emergent Gameplay:** "Singing Plants" can trigger other game mechanics (e.g., opening doors via `Chorus` chords) simply by growing.

### Negative
*   **Memory Usage:** The string expansion can grow exponentially. We enforce a recursion depth limit and a maximum string length to prevent memory exhaustion.
*   **State Complexity:** The `Seed` Organelle must manage a sophisticated turtle stack and expansion buffer, making it heavier than a standard `Worker` Organelle.
