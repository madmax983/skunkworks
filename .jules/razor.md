## [Reduction]
**Bloat:** `nova_chaos` module (Chaos Cartridge / Alchemy System) in `chimera-lang`.
**Cut:** Deleted module, associated tests, 8 OpCodes, and integration logic in VM and TUI.
**Saved:** ~400 lines of code / Significant cognitive load (removed a feature silo that wasn't core to the biological simulation).
## [Reduction]
**Bloat:** `nova_quipu` module (Quipu Knot System) in `chimera-lang`.
**Cut:** Replaced complex `Cord` struct and `Knot` enum with a simple `Vec<i64>`. Removed knot parsing/rendering logic.
**Saved:** ~100 lines of code / Massive reduction in complexity (removed a physical simulation that was just storing integers).
