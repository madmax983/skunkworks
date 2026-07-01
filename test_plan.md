1. **Analyze the problem:** The user reported `capacity overflow` panic in origami, but checking `crates/origami`, that was already solved based on the journal `.jules/atlas.md`.
Looking closer at the prompt, I am asked to enforce high cohesion and low coupling across the codebase, identifying and fixing "The Knot", "The Leak", "The Bloat", "The Sprawl", "The Shotgun" etc.
Wait, the `literary-boids` crate had an unresolved import issue with `tui_shared::math::Vec2`. I fixed it by changing it to `locus::Vec2`.
Also I ran `grep -rnw -e "pub mod" crates experiments graveyard alleles` which showed that many `graveyard` and `alleles` crates are leaking their internal submodules directly via `pub mod`. I applied the Facade pattern by replacing `pub mod` with `pub(crate) mod` in `alleles/literary-boids/src/main.rs`. Wait, I did `pub(crate) mod` for `boid`, `critic`, `syntax_physics`, `traces` and `world` in `literary-boids/src/main.rs`. But that wasn't an explicit PR request yet, I should probably do it properly and record the ADR/journal.
Wait, let's review `.jules/atlas.md` again. It mentions I only add journal entries when I discover:
- A circular dependency that required a "Facade" or "Bridge" pattern to fix.
- A module that has grown too large and needs splitting (The "Blob" anti-pattern).
- A violation of the Orphan Rule that forced a specific trait design.
- An Architectural Decision Record (ADR) that needs to be captured.

The core problem was `literary-boids` failed to compile because `tui_shared::math` was removed (presumably moved to `locus`). I fixed `literary-boids`, `repo-rover`, `laban-rover`, `metric-marionette`, and `git_galaxy` to use `locus::Vec2`.
And I encapsulated the modules in `literary-boids` with `pub(crate) mod`.
Let's see if there are any other leaks or tangles.
