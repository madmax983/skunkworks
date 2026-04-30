# 090. Extract Nova Dispatch

Date: 2026-04-29

## Status
Accepted

## Context
The `experiments/chimera-lang/src/vm/mod.rs` file was heavily bloated, largely due to `exec_nova_dispatch` being a monolithic 400+ line `match` block. This block tightly coupled Nova expansion logic into the main Virtual Machine definition, creating an architectural bottleneck. Any modification to Nova-specific execution logic required touching the core VM definition file, increasing the risk of regressions and making the `mod.rs` file difficult to navigate and maintain.

## Decision
Extracted `exec_nova_dispatch` from `mod.rs` into a new `experiments/chimera-lang/src/vm/ops/nova_dispatch.rs` module under the existing `impl crate::vm::ChimeraVM` block. The new module is then conditionally imported in `experiments/chimera-lang/src/vm/ops/mod.rs` via `#[cfg(feature = "nova")]`.

## Consequences
- **Positive:** Reduced coupling between the core VM execution loop and Nova expansion domain logic.
- **Positive:** Significantly improved modularity and reduced the line count of the core `mod.rs` file.
- **Negative:** Increased indirection slightly, as Nova-specific operations now require navigating to the `nova_dispatch.rs` module rather than finding them inline within `mod.rs`.
