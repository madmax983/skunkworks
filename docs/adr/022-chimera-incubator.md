# 22. Chimera Incubator (Genetics & CRISPR)

Date: 2025-05-15 (Simulated)

## Status

Accepted

## Context

The "Nova" extension aims to simulate biological processes within the execution environment of the Chimera VM. A core requirement was the ability for programs (organisms) to evolve, reproduce, and modify their own source code based on environmental factors.

Standard Genetic Algorithms (GA) usually operate outside the individual (in a "God" loop). However, consistent with the "Chimera Sovereignty" principle (ADR 018), we wanted the *organisms themselves* to control their reproduction and evolution via OpCodes.

Furthermore, we needed a way for the environment (the Grid) to influence the genome, allowing "abiogenesis" or "infection" from data.

## Decision

We implemented the **Chimera Incubator** system (`src/vm/nova_genetics.rs`), providing a set of high-level biological operators as OpCodes.

### Key Components

1.  **Incubator (`Incubate`)**:
    *   Reads a sequence of values from the Grid (the environment) starting at `(X, Y)` with length `L`.
    *   Translates these values into OpCodes (Integers -> `Push(N)`, Strings -> `OpCode`).
    *   Compiles this sequence into a new `Strand` (DNA) and injects it into the organism's genome.
    *   **Significance**: This allows "Code from Data," enabling the environment to act as a mutagen or a source of new genetic material.

2.  **Reproduction (`Mitosis` / `Splice`)**:
    *   `Mitosis(StrandIdx)`: Clones a specific strand, preserving its epigenetic state.
    *   `Splice(Method, StrandA, StrandB)`: Combines two strands using crossover algorithms (Interleave, Uniform, Midpoint) to create a hybrid child.

3.  **Genome Editing (CRISPR/Cas9)**:
    *   `CrisprScan(Guide, Target)`: Scans a target strand for a pattern matching a guide strand.
    *   `Cas9Cut(Index, Strand)`: Cuts a strand at a specific index, splitting it into two.
    *   `Ligase(Donor, Recipient)`: Joins two strands end-to-end.
    *   `Integrase` / `Excision`: Inserts or removes specific genes.

4.  **Epigenetics (`Methylate` / `Demethylate`)**:
    *   Allows tagging specific genes with markers.
    *   Markers are inherited during `Mitosis` but do *not* change the underlying code (OpCode).
    *   Used by regulatory logic (e.g., "Skip methylated genes") to control expression without mutation.

## Consequences

### Positive
*   **Self-Modifying Code**: Organisms can rewrite themselves, repair damage, or optimize their own code.
*   **Lamarckian Evolution**: Epigenetic markers allow acquired states to be passed to offspring.
*   **Emergent Complexity**: The combination of `Incubate` (Environment -> Code) and `Manifest` (Code -> Environment, via ADR 021) creates a closed feedback loop for evolution.

### Negative
*   ** fragility**: The mapping from Grid Values to OpCodes in `Incubate` is strict. Random noise rarely produces valid, executable code (most becomes `Nop` or `Unknown`).
*   **Cancer Risk**: Without strict energy costs (which we implemented), organisms tend to evolve into tight `Mitosis` loops, consuming all resources.
*   **Lineage Tracking**: The `Cladistics` module must track complex parentage (Splice has 2 parents, Mitosis 1, Incubate 0/Environment), complicating the family tree visualization.

## Compliance

This system complements the "Nova" feature set, working alongside the "Oracle" (ADR 021) to provide the physical/genetic layer to the Oracle's logical/intellectual layer.
