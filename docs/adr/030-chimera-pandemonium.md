# 30. Chimera Pandemonium Reactor

Date: 2024-05-22

## Status

Accepted

## Context

The Chimera VM executes genetic code sequentially, treating the genome as a linear array of genes. However, many interesting behaviors in the VM—such as recursion, loops, and self-modification—exhibit cyclical patterns that are difficult to visualize or manipulate in a traditional linear view.

Furthermore, the "Mad Scientist" persona adopted by users of the system demands a more tactile, chaotic method for interacting with the genome. Editing individual opcodes via text is precise but lacks the visceral feeling of "playing god" or inducing mutation through raw energy. Users need a way to apply broad, stochastic changes to the genome to escape local optima or simply to observe the resulting chaos.

## Decision

We will implement a **Pandemonium Reactor**, a dedicated interactive subsystem within the Chimera environment.

1.  **Spiral Visualization**: The linear genome will be mapped to a polar coordinate system ($r = \theta$), creating a "Spiral of Life" visualization. This allows long genomes to be viewed compactly and reveals periodic structures as alignment along radial vectors.

2.  **Atomic Mutation Backend**: A new module, `vm::pandemonium`, will be created to handle atomic genomic operations that are distinct from standard execution. These operations include:
    *   **Mutate**: Randomly alter a single gene's opcode or argument.
    *   **Scramble**: Shuffle genes within a specified radius.
    *   **Purge**: Delete genes within a radius (creating a Void).
    *   **Duplicate**: Clone a gene sequence.
    *   **Storm**: Apply a combination of effects over a wide area.

3.  **TUI Integration**: The Reactor will be accessible via a new `ViewMode::Pandemonium` in the TUI, featuring a targeting reticle controlled by the user.

## Consequences

**Positive:**
*   **Intuitive Manipulation**: Users can visually identify and target repeating patterns (loops) for modification.
*   **Rapid Prototyping**: Allows for quick, messy experiments with genetic structure without worrying about syntax errors (as the VM is resilient).
*   **Gamification**: Transforms the debugging/editing process into an interactive experience.

**Negative:**
*   **Destructive Capability**: The Reactor makes it trivial to destroy valid, functioning code. It is a tool of chaos, not precision.
*   **Complexity**: Mapping Cartesian cursor coordinates back to the linear genome index requires distance minimization logic ($O(N)$), which may impact TUI performance on very large genomes.
