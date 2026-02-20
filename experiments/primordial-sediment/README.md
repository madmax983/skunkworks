# Primordial Sediment

*The code is rotting, and something is eating it.*

This experiment visualizes a codebase as a decaying "Sediment" layer, upon which a "Primordial Soup" of biological entities live.

## Concept

-   **Sediment**: The text of the codebase (loaded from git history or local file) acts as the substrate. Over time, it decays due to entropy (random bit rot).
-   **Soup**: Biological particles (Algae, Grazers, Predators) inhabit the text grid.
-   **Interaction**:
    -   **Algae (Detritivores)**: Feed on non-whitespace characters (code). When they eat, the character disappears (becomes whitespace). They convert "Complexity" into "Life".
    -   **Grazers**: Feed on Algae.
    -   **Predators**: Feed on Grazers.

## Lineage

-   **Parent A**: `experiments/digital-sediment` (Git history visualization, Entropy/Bit Rot).
-   **Parent B**: `experiments/primordial-soup` (Fluid physics, biological rules).

## Novel Trait

**Bio-Entropic Feedback Loop**: The ecosystem is sustained by the destruction of the codebase. A healthy ecosystem means a rapidly decaying file.

## Controls

-   `q`: Quit
