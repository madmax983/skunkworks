# 124. Encapsulate Chimera Lang TUI and VM Modules via Facade

Date: 2026-06-19

## Status

Proposed

## Context

In the `chimera-lang` crate, internal modules within the `tui` submodule (e.g., `tui/views/mod.rs`, `tui/mod.rs`) and the `vm` submodule (e.g., `vm/ops/mod.rs`, `vm/systems/mod.rs`, `vm/prologue/mod.rs`) were previously exposed using `pub mod`. This leaked internal implementation details to the rest of the application and external consumers, breaking the Facade pattern. The tight coupling allowed other modules to bypass established APIs and depend directly on nested structures.

## Decision

We enforced the Facade pattern by replacing the exposed `pub mod` instances with `pub(crate) mod` within these submodules. To maintain functionality, we explicitly re-exported only the intended types and functions using `pub use`. This hides the internal module hierarchy and exposes a clean, unified API for both the VM execution layer and the TUI rendering layer.

## Consequences

- **Positive:** Improved structural isolation and high cohesion within the `chimera-lang` architecture. Strict module boundaries ensure that external code cannot depend on volatile internal implementations, reducing coupling and making the codebase easier to safely refactor. Unused internal code elements are now properly identified by the compiler.
- **Negative:** Requires slightly more boilerplate for developers, who must now explicitly maintain `pub use` statements when adding new features or types to the public interface of the VM or TUI.
