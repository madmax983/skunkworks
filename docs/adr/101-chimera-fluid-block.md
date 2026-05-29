# 101. Chimera Fluid Block

Date: 2026-05-25

## Status
Accepted

## Context
Simulating continuous environments within the discrete execution model of `chimera-lang` was inefficient and syntactically verbose. We needed a native way to execute fluid dynamics operations directly from scripts.

## Decision
The `prolouge` compiler was updated to support a `fluid_block`. This block is compiled into the new `OpCode::Fluid` instruction, allowing the ChimeraVM to delegate physical simulations to specialized physics subsystems.

## Consequences
* **Positive:** Enables powerful, native fluid simulation capabilities directly controllable by esolang logic.
* **Negative:** Further couples the `chimera-lang` VM with continuous physics engine subsystems, potentially reducing its portability.
