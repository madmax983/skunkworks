# 141. Flatten Empty Namespaces and NarrativeBuilder

Date: 2026-07-01

## Status

Accepted

## Context

Several experiments used empty structs merely as namespaces for single methods, adding unnecessary object-oriented boilerplate where functional approaches are more idiomatic and simpler. Specifically, `TextGlitcher` in `mnem-*` experiments, `Assembler` in `hidden-brush`, and `RecoveryEngine`/`EntropyEngine` in `digital-sediment`. Furthermore, `NarrativeBuilder` in `chimera-lang` provided a verbose Builder pattern for simple Grid mutations, which was overly complex. These violated the KISS and YAGNI principles.

## Decision

*   Flattened `TextGlitcher`, `Assembler`, `RecoveryEngine`, and `EntropyEngine` empty structs into free functions.
*   Removed `NarrativeBuilder` in `chimera-lang`, refactoring tests to manipulate the VM grid and DNA helix directly.

## Consequences

*   **Positive:** Reduces cognitive load by eliminating speculative abstraction naming and namespaces. Aligns with essentialist engineering practices (KISS). Code is more declarative and idiomatic.
*   **Negative:** None.
