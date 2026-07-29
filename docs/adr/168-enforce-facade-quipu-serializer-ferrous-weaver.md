# 168. Enforce Facade Pattern in quipu-serializer and ferrous-weaver

## Status
Proposed

## Context
The `experiments/quipu-serializer` leaked its internal `ser` module through its API. This led dependent crates like `ferrous-weaver` to import directly from the internal module (`quipu_serializer::ser::to_quipu`), bypassing the intended Facade pattern and creating high coupling to the internal directory structure.

## Decision
We updated dependent usages in `ferrous-weaver` and related documentation to import `quipu_serializer::to_quipu` instead of `quipu_serializer::ser::to_quipu`. This enforces a clean boundary, adhering strictly to the crate's public Facade.

## Consequences
External consumers now interact strictly with the root facade of `quipu-serializer`. This improves encapsulation, reduces coupling to the crate's internal structure, and ensures compliance with the system's architectural guidelines.
