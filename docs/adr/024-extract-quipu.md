# 024. Extract Quipu Data Structures

## Status
Accepted

## Context
Multiple experiments (`quipu-symphony`, `quipu-cradle`, `quipu-serializer`) utilized the Inca `Quipu` data structure (consisting of `Knot`s and `Cord`s) for storing and visualizing numerical data. Each experiment implemented its own version of these structures, leading to code duplication and inconsistencies in representation and behavior (e.g., how knots are visualized or how values are calculated).

## Decision
We will extract the core `Quipu`, `Cord`, and `Knot` data structures and their associated logic into a shared workspace library: `crates/quipu`.

### Key Components:
1.  **Knot Enum:** Represents the three types of knots used in base-10 Quipu encoding: `Simple` (10s+), `Long` (Units 2-9), and `FigureEight` (Unit 1).
2.  **Cord Struct:** Represents a vertical string containing clusters of knots, mapped to powers of 10.
3.  **Quipu Struct:** A collection of Cords.
4.  **Display Logic:** Shared `fmt::Display` implementation for consistent text-based visualization.

## Consequences
### Positive
*   **Reuse:** New experiments can leverage the Quipu structure without re-implementing it.
*   **Consistency:** All Quipu-based visualizations will now look the same and behave identically.
*   **Maintenance:** Bug fixes or enhancements (e.g., supporting subtraction or new knot types) only need to be applied in one place.

### Negative
*   **Coupling:** Experiments are now dependent on a shared crate, meaning changes to `crates/quipu` could potentially break multiple downstream consumers if not handled carefully.
