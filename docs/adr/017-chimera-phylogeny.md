# 17. Chimera Phylogeny & Host Interaction

Date: 2024-10-25

## Status

Accepted

## Context

The `chimera-lang` experiment is designed to simulate a "living" codebase. A key aspect of this simulation is the ability for the organism (the VM executing the DNA) to interact with its environment (the host filesystem and shell). This interaction is crucial for experiments involving:

*   **Repository Gardening**: The VM autonomously maintaining, refactoring, or evolving the source code of the repository itself.
*   **Genetic Algorithms on Files**: Using the filesystem as a gene pool, where files are read, processed, and rewritten based on fitness functions.
*   **Self-Replication**: The ability for a Chimera program to propagate itself to other files or directories.

Without direct host interaction, the VM remains isolated in a sandbox, limiting its evolutionary potential to internal memory states.

## Decision

We implemented the `phylogeny` feature, which exposes a set of Opcodes for direct filesystem and shell interaction. These Opcodes are gated behind the `feature = "phylogeny"` flag to prevent accidental execution in secure environments.

The new Opcodes are:

1.  **`Crawl`**: Scans a directory and returns a `Junction` (list) of filenames.
2.  **`Sequencing`**: Reads the content of a file into a String on the stack.
3.  **`Synthesize`**: Writes a String from the stack to a file, overwriting existing content.
4.  **`Infect`**: Appends a String from the stack to a file.
5.  **`Shell`**: Executes a shell command and pushes the `stdout` to the stack.

## Consequences

**Positive:**
*   **True Evolution**: The VM can now modify its own source code (if it can locate it) or the source code of other projects, enabling real-world "code organism" behavior.
*   **Automation**: Chimera scripts can be used for build automation, testing, and other dev-ops tasks within the repository.
*   **Data Persistence**: The VM can save its state or "offspring" to disk, surviving beyond a single execution cycle.

**Negative:**
*   **Security Risk**: Granting `Shell` and `Synthesize` capabilities allows arbitrary code execution and file destruction. A malicious or buggy Chimera script could delete the entire repository or execute harmful commands.
*   **Platform Dependence**: Shell commands are inherently platform-specific (e.g., `ls` vs `dir`), reducing the portability of DNA strands that use them.
*   **No Sandboxing**: The current implementation relies entirely on the OS user's permissions. There is no internal "jail" or path validation within the VM itself. The `phylogeny` feature is an "all-or-nothing" trust model.
