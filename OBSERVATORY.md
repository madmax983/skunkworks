# Observatory

Document patterns, shared bottlenecks, and emerging abstractions here.

## 🔭 Observations

### The `rand` Schism
Many experiments use `macroquad::rand` (which wraps `quad-rand`) for deterministic cross-platform RNG, while others use the standard `rand` crate. Mixing them can lead to confusion (e.g., `gen_range` signatures differ).
**Hypothesis:** A unified `Randomness` trait in `tui-shared` or `locus` could abstract this, allowing experiments to swap backends easily.

### The Interaction Dance
A recurring pattern in agent simulations (`lingua-franca`, `chimera-bridge`, `slime-train`) is the need for agents in a single `Vec` to interact.
This leads to:
1. `split_at_mut` gymnastics.
2. Index-based interaction (store indices, mutate later).
3. `unsafe` blocks (bad!).
**Proposal:** A `InteractionArena` or `AgentPairIter` helper in `locus` that safely yields mutable pairs would be valuable infrastructure.

### The "World" Struct
Almost every experiment re-invents a `World` struct containing `agents`, `resources`, `grid`.
**Prediction:** We will eventually need a lightweight ECS (Entity Component System) that fits `macroquad`'s immediate mode style better than full `bevy`. `hecs` or a custom `SlotMap` based solution might emerge.

### Visualizing Invisible State
Experiments like `chimera-bridge` (color = vm state) and `lingua-franca` (text = internal lexicon) rely heavily on visualizing internal state.
**Pattern:** "Debug Overlay" toggles are becoming essential. Press 'D' to see the brain.
