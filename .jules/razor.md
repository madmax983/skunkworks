## [Reduction]
**Bloat:** Manual `match` statements for Enum <-> Integer conversion and verbose neighbor counting in `experiments/automata-warfare`.
**Cut:** `#[repr(usize)]`, `From<usize>`, and direct array indexing.
**Saved:** ~50 lines of code, reduced cognitive load in `update`.
