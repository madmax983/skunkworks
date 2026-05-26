# 102. Chimera Physics Blocks

Date: 2026-05-26

## Status
Proposed

## Context
Various physics and TUI experiments (`gray_scott`, `locus`, `neuro`, `platter`) required direct integration into the `chimera-lang` esolang to allow scripts to natively invoke and configure their specialized simulations without external workarounds.

## Decision
The `prolouge` compiler and `pest` grammars were updated to support four new block types: `gray_scott_block`, `locus_block`, `neuro_block`, and `platter_block`. These blocks compile to corresponding `OpCode` variants, which are subsequently dispatched to the respective subsystems during VM execution in `nova_dispatch.rs`.

## Consequences
* **Positive:** Esolang scripts can directly express advanced physics and visualization configurations, centralizing execution and configuration.
* **Negative:** Increases the complexity of the compiler and VM, tightly coupling the esolang with external physics libraries.
