# 55. Chimera Paradox System

Date: 2024-05-24

## Status

Proposed

## Context

The ChimeraVM operates primarily on a biological metaphor where organisms execute imperative code (DNA) to interact with their environment. While this models individual agency well, it struggles to represent global environmental laws or "magic" effects that should occur independently of any single organism's actions.

For example, implementing a rule like "Any cell containing Water that is touched by Fire turns to Steam" requires every Fire or Water particle to constantly check its neighbors, leading to massive code duplication and performance overhead if implemented via organism DNA.

We need a system to define "Physics" or "Magic" rules that:
1. Run globally or regionally every tick.
2. Are defined dynamically (runtime programmable).
3. Can modify the grid state without an associated organism body.

## Decision

We will implement a **Paradox System**, a rule-based engine that runs alongside the imperative VM.

1.  **Structure**: A `Paradox` struct will be added to `ChimeraVM`. It will hold a collection of `Rule`s.
2.  **Rules**: Each `Rule` consists of:
    *   **Trigger**: A condition (e.g., `Always`, `Signal(String)`).
    *   **Actions**: A list of effects (e.g., `Log`, `Set(x, y, val)`, `Glitch(amount)`).
3.  **Execution Phase**: The `Paradox` system will execute in its own phase during the `step()` cycle, separate from organism execution.
4.  **Borrowing Strategy**: To satisfy Rust's borrow checker (since rules need mutable access to the VM they are stored in), the `Paradox` struct will be temporarily moved out of the VM (`std::mem::take`) during its execution tick, then returned.

## Consequences

### Positive
*   **"God Mode"**: Allows for powerful, scriptable interactions that transcend standard organism limitations.
*   **Efficiency**: Global rules run once per tick per rule, rather than once per tick per organism, reducing overhead for simple environmental effects.
*   **Flexibility**: Rules can be added or removed at runtime via the `Paradox` opcode or TUI.

### Negative
*   **Complexity**: The `std::mem::take` pattern adds complexity to the `step()` function and requires careful handling to ensure the struct is always returned.
*   **Performance**: A large number of rules with complex triggers (e.g., scanning the entire grid) could degrade performance significantly.
*   **Debugging**: "Action at a distance" effects can be difficult to trace compared to localized organism actions.
