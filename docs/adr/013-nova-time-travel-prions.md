# 13. Nova Time Travel & Prion Protocols

Date: 2024-10-25

## Status

Accepted

## Context

The `chimera-lang` experiment aims to simulate advanced biological processes within an esoteric programming language. Standard execution flow is linear and deterministic. However, to model complex evolutionary strategies like "backtracking" (Time Travel) and "adaptive mutation" (Prions), we needed mechanisms to manipulate the VM's state and instruction set at runtime.

*   **Time Travel**: The ability to save a checkpoint, explore a path, and revert if it leads to an unfavorable outcome (e.g., death).
*   **Prions**: The ability to dynamically redefine the meaning of instructions (e.g., swapping `Add` with `Sub`) to simulate infectious coding or adaptive reflexes.

## Decision

We implemented two core protocols in the "Nova" feature set:

1.  **The Spore Protocol (Time Travel)**:
    *   We introduced a `Spore` struct that captures a deep clone of the `ChimeraVM` state (Stack, Grid, DNA, Epigenome, etc.).
    *   `OpCode::Sporulate`: Serializes the current state into a `Spore` and stores it in the VM's `spores` vector, pushing its ID to the stack.
    *   `OpCode::Germinate`: Accepts a spore ID and replaces the current VM state with the stored `Spore` state, effectively rewinding time.

2.  **The Prion Protocol (Instruction Remapping)**:
    *   We added a `remap_table: HashMap<OpCode, OpCode>` to `ChimeraVM`.
    *   During execution, the VM checks this table before executing any gene. If an entry exists, the mapped OpCode is executed instead.
    *   `OpCode::Remap` and `OpCode::Restore` allow the program to modify this table at runtime.
    *   `OpCode::Mirror` reverses the execution direction on the DNA strand.

## Consequences

**Positive:**
*   **Evolutionary Search**: Programs can perform depth-first searches of the solution space using Sporulate/Germinate (Try -> Fail -> Revert).
*   **Metaprogramming**: Prions allow self-modifying behavior without changing the immutable DNA source, simulating epigenetic shifts.
*   **Resilience**: The `Spore` mechanism serves as a fault-tolerance system for risky operations (like `Cas9Cut`).

**Negative:**
*   **Memory Overhead**: `Spore` snapshots are heavy (Deep Copy). Creating too many spores can exhaust memory.
*   **Debuggability**: Dynamic opcode remapping makes static analysis of the DNA impossible. "What you see is NOT what you execute."
*   **Complexity**: The VM step function must now handle directionality and lookups, slightly reducing raw execution speed.
