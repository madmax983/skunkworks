# 21. Chimera Oracle (Prolog Logic Engine)

Date: 2025-05-15 (Simulated)

## Status

Accepted

## Context

The Chimera VM, originally an imperative stack-based machine, lacked a declarative way for organisms to query their environment or reason about abstract relationships. Imperative checks for complex conditions (e.g., "Is there a neighbor with energy > 50 and a specific gene?") required verbose and fragile bytecode sequences.

Furthermore, the "Logos" feature (Logic Chemistry) required a way to define chemical reactions as rules (`reaction(Agent, Reagent, Product)`) that could be queried dynamically, rather than hardcoded in Rust.

We needed a system that could:
1. Store facts and rules (Knowledge Base).
2. Perform unification and resolution (Prolog-style).
3. Introspect VM state (Energy, Grid, Genes) as if they were logical facts.

## Decision

We integrated a **Prolog-subset inference engine** directly into the `ChimeraVM` via the `oracle` module (`src/vm/oracle.rs`).

### Key Components

1.  **Knowledge Base (KB)**: A `Vec<Value>` storing facts and rules. Rules are represented as `Junction(Any, ["rule", Head, Body...])`.
2.  **Unification**: A `unify(t1, t2, subst)` function that matches terms and binds variables (strings starting with `?`).
3.  **Resolution**: A `solve(goals, ...)` function that performs depth-limited backward chaining.
4.  **Dynamic Predicates**: Special predicates that do not exist in the KB but are resolved against the VM's runtime state:
    *   `cell(X, Y, Val)`: Queries the grid.
    *   `gene(Strand, Idx, Op)`: Queries the genome.
    *   `neighbor(X, Y, Dir, NX, NY)`: Queries topology.
    *   `energy(E)`: Queries VM energy.

### New OpCodes

*   `Assert`: Adds a fact to the KB.
*   `Retract`: Removes a fact.
*   `Rule`: Defines a rule (Head :- Body).
*   `Query`: Solves a goal and returns success/failure.
*   `Seek`: Finds a strand index satisfying a predicate (e.g., `has_feature(?Target, "OpCode")`).
*   `Manifest`: Materializes the result of a query into the world (e.g., `cell(X, Y, NewVal)`).
*   `Augury`: Registers a standing query ("Omen") that triggers an effect when fulfilled.

## Consequences

### Positive
*   **Introspection**: Organisms can now "see" their own code and state using logic queries.
*   **Declarative Logic**: Complex behaviors (like chemical reactions or social rules) can be defined as data (rules) rather than code.
*   **Logos Integration**: The `process_logos` system relies entirely on `reaction/3` rules in the KB, allowing "programmable chemistry".

### Negative
*   **Performance**: Prolog resolution is computationally expensive compared to direct memory access. A recursion depth limit (50) is enforced to prevent stack overflows.
*   **Complexity**: Debugging logic failures (unification errors) inside the VM is difficult.
*   **State Drift**: If the KB grows too large with `Assert`, performance degrades linearly (no indexing yet).

## Compliance

This decision adheres to the "Chimera Sovereignty" principle by giving the VM introspective power over its own execution context.
