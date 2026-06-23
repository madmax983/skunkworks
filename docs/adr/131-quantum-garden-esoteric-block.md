# 131. Quantum Garden Esoteric Block

Date: 2026-06-23

## Status

Proposed

## Context

The Prologue persona has recently integrated the `quantum-garden` experiment into `chimera-lang`. Similar to previous esoteric blocks (like `chaos-hologram` or `cymatic-ocean`), `quantum-garden` needed to be natively accessible via the `chimera-lang` compiler and Virtual Machine.

## Decision

We expanded the `chimera-lang` grammar (Pest) to support a new esoteric block: `quantum_garden_block`.

The `PrologueCompiler` was updated to parse this syntax block and emit a corresponding new instruction: `OpCode::QuantumGarden`.

Finally, the `nova_dispatch.rs` execution pipeline was updated to map this instruction to its respective simulation system within `ChimeraVM`.

## Consequences

- **Positive:** Enables esolang scripts to directly invoke and control the quantum garden simulation paradigms, allowing for quantum jump, tunneling, and superposition dynamics natively within `chimera-lang` code.
- **Negative:** Continues to tighten the coupling between the `chimera-lang` compiler/VM and isolated experimental features, growing the overall scope and responsibilities of the dispatch logic in `nova_dispatch.rs` and the language's `OpCode` definitions.
