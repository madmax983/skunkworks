## [Reduction]
**Bloat:** Manual `match` statements for Enum <-> Integer conversion and verbose neighbor counting in `experiments/automata-warfare`.
**Cut:** `#[repr(usize)]`, `From<usize>`, and direct array indexing.
**Saved:** ~50 lines of code, reduced cognitive load in `update`.

## [Reduction]
**Bloat:** `AgentLogic` trait in `experiments/ram-bazaar` was a "One-Time Trait" implemented only by `Agent`.
**Cut:** Deleted the trait, moved `decide_bids` and `update_budget` to `impl Agent`.
**Saved:** 5 lines of boilerplate, removed unnecessary abstraction layer.
