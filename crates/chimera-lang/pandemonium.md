# The Pandemonium Reactor ☢️

> "Order is but a transient island in the ocean of Chaos. We are the storm." - Prologue

The Pandemonium Reactor is a new interactive interface for the Chimera Virtual Machine, designed for the Mad Scientist who prefers hands-on genomic manipulation over careful programming.

## Concept

The Reactor visualizes the entire genome as a **Spiral of Life**. Each point on the spiral represents a gene. The color of the point indicates the strand index, creating a vibrant, swirling pattern of genetic potential.

## Controls

*   **View Mode**: Press `P` or cycle with `Tab` to access the Pandemonium Reactor.
*   **Navigation**: Use `Arrow Keys` to move the Targeting Reticle (Red Circle).
*   **Radius Control**: Use `[` and `]` to decrease/increase the effect radius of your tools.
*   **Tool Selection**:
    1.  **Mutate (1)**: Precision strike. Mutates a single gene at the cursor's location.
    2.  **Scramble (2)**: Area of Effect. Randomizes the order of genes within the effect radius.
    3.  **Purge (3)**: Void Beam. Deletes all genes within the effect radius.
    4.  **Duplicate (4)**: Cloning Ray. Duplicates the gene at the cursor's location.
*   **Activate**: Press `Space` to fire the selected tool.

## Philosophy

The Pandemonium Reactor embodies the principle of **Interactive Evolution**. Instead of waiting for random mutations to accumulate, the user becomes the agent of selection (and destruction). It allows for rapid prototyping of genetic structures through chaotic intervention.

## Technical Details

*   **Visualization**: Maps linear genomic coordinates to a polar spiral `r = theta`.
*   **Input**: Cartesian cursor mapped back to genomic space via distance minimization.
*   **Backend**: Leverages the `vm::pandemonium` module for atomic genomic operations.

## Warning

Use with caution. Excessive exposure to Pandemonium radiation may cause:
*   Infinite loops
*   Stack overflows
*   Sentient bugs
*   Loss of sanity

*Evolution is not a straight line. It's a spiral.*
