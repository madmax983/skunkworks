# 132. Encapsulate Arthropod and Turbulent Rhythms via Facade

Date: 2026-06-24

## Status
Proposed

## Context
The Atlas persona recently observed that internal modules within `crates/arthropod/src/lib.rs` and `experiments/turbulent-rhythms/tests/havoc_contention.rs` were leaking via `pub mod`. This breaks the intended Facade pattern and exposes implementation details directly to consumers, resulting in tight coupling and violating architectural boundaries.

## Decision
We enforced the Facade pattern by replacing the exposed `pub mod` instances with `pub(crate) mod` within `crates/arthropod` and `experiments/turbulent-rhythms`. To maintain functionality and API continuity, we explicitly re-exported only the intended types and functions using `pub use`.

## Consequences
*   **Positive:** Strengthens encapsulation, preventing external consumers from depending on internal directory and module structures. Reduces coupling and improves the maintainability of the affected crates.
*   **Negative:** Adds a small amount of boilerplate by requiring explicit `pub use` statements to form the boundary APIs.
