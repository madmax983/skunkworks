# Chimera Choreography 💃🧬

**Lineage:** `chimera-lang` × `laban-rover`

This experiment explores **Choreographic Programming**. ChimeraVM agents ("Dancers") executing genetic code where opcodes and memory state trigger movement changes based on **Laban Effort** parameters.

## Concept

Each dancer is a ChimeraVM instance. The VM's grid memory is mapped to the four Laban Effort factors:
*   **Space (Direct/Indirect):** Determines pathfinding linearity. Indirect agents wander; Direct agents seek goals.
*   **Weight (Strong/Light):** Determines physical force/acceleration magnitude.
*   **Time (Sudden/Sustained):** Determines velocity capping and impulse frequency.
*   **Flow (Bound/Free):** Determines friction and inertia.

## Controls

*   `q` / `Esc`: Quit
*   `+` / `-`: Zoom

## Lineage

*   **Parent A (`chimera-lang`):** Provides the genetic execution engine (VM) and grid memory.
*   **Parent B (`laban-rover`):** Provides the Laban Movement Analysis logic and TUI visualization concepts.
*   **Novelty:** The code *dances*. The execution state is visualized not as a debug log, but as physical movement quality.
