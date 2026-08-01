# ADR 166: Encapsulate parsers and internal modules in various crates

## Status
Accepted

## Context
The `experiments/chimera-lang`, `experiments/system-turbulence`, `experiments/chimera-esolang`, and `graveyard/git_rhythm` crates leaked various internal modules via `pub mod`. The `chimera-lang` crate specifically leaked compiler parser modules like `helixparser_mod`, `prologueesolangparser_mod`, `scriptparser_mod`, etc., and the `nova_pachinko` module.

## Decision
Replace `pub mod` with `pub(crate) mod` for these modules to enforce strict boundaries and hide implementation details, while preserving the structural integrity and avoiding breaking integrations.

## Consequences
Implementation details are now hidden. The structural integrity is preserved and integrations remain unbroken.
