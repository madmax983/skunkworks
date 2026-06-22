# 128. Chimera Esoteric Blocks

Date: 2026-06-22

## Status

Proposed

## Context

The Prologue persona has successfully integrated several new experimental environments into `chimera-lang`. Specifically, the `chaos-hologram`, `cymatic-ocean`, and `verge-computer` experiments have been adapted as esoteric logic blocks. Previously, these were distinct experiments. To enable scripts to utilize their logic and paradigms seamlessly, they needed to be natively accessible via the `chimera-lang` compiler and Virtual Machine.

## Decision

We have expanded the `chimera-lang` grammar (Pest) to support new esoteric blocks: `chaos_hologram_block`, `cymatic_ocean_block`, and `verge_block`.

The `PrologueCompiler` was updated to parse these syntax blocks and emit corresponding new instructions: `OpCode::ChaosHologram`, `OpCode::CymaticOcean`, and `OpCode::Verge`.

Finally, the `nova_dispatch.rs` execution pipeline was updated to map these instructions to their respective simulation systems within `ChimeraVM`.

## Consequences

- **Positive:** Enables esolang scripts to directly invoke complex external paradigms, including holographic chaos memory, cymatic fluid dynamics, and recursive verge computation, expanding the expressiveness of the language.
- **Negative:** Tightens the coupling between the `chimera-lang` compiler/VM and these specific experimental simulation crates. The VM must now explicitly know about and dispatch to these experimental features, increasing the complexity of `nova_dispatch.rs` and the `OpCode` enum.