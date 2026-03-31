1. *Create a new file `experiments/chimera-lang/src/vm/ops/misc.rs` or `prion.rs`, `scavenge.rs`, `digest.rs`, etc.*
   - Since `mod.rs` still has some loose OpCodes (`exec_prion_op`, `exec_scavenge_op`, `exec_digest_op`, `exec_havoc_op`, `exec_char_op`, `exec_mutagen_op`, `exec_findall_op`), I should extract them into the `experiments/chimera-lang/src/vm/ops/` submodule, potentially grouping them into a single `misc.rs` or separating them into `prion.rs`, `havoc.rs`, `bio_ext.rs` etc.
2. *Extract the loose operations*
   - Move `exec_prion_op` -> `prion.rs`
   - Move `exec_scavenge_op`, `exec_digest_op`, `exec_mutagen_op` -> `bio.rs` ? Or a new `mutation.rs` / `survival.rs`
   - Move `exec_havoc_op` -> `havoc.rs`
   - Move `exec_char_op` -> `string.rs`
   - Move `exec_findall_op` -> `search.rs`
3. *Complete pre commit steps*
   - Complete pre commit steps to make sure proper testing, verifications, reviews and reflections are done.
4. *Submit the change.*
