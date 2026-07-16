# 158. Flatten Misc Operations

Date: 2026-07-16

## Status

Proposed

## Context

The `MiscOps` module in `chimera-lang/src/vm/ops/misc.rs` suffered from deeply nested 'Pyramids of Doom' in operations like `exec_prion_op`, `exec_scavenge_op`, `exec_digest_op`, `exec_havoc_op`, `exec_transposon`, and `exec_mutagen_op`. The nested `match` and `if let` blocks created high cognitive load and made the logic difficult to follow and maintain.

## Decision

Extract the logic for specific opcodes (`Remap`, `Restore`, and `Mirror`) from `exec_prion_op` into dedicated private helper functions (`apply_remap`, `apply_restore`, `apply_mirror`). Additionally, flatten the control flow in other `misc` execution functions using idiomatic `let...else` guard clauses and early returns.

## Consequences

- **Positive:** Reduces cognitive complexity and improves code readability.
- **Positive:** Enforces idiomatic Rust error handling and control flow.
- **Negative:** Slightly increases the number of internal methods on `ChimeraVM`.
