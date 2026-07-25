# 🧬 git-strings

**Lineage:**
- **Parent A (Intent/Data):** `experiments/git-harmonograph`. Inherits the ability to parse local repository `git log` metadata (commit hashes, authors, messages).
- **Parent B (Physics/Medium):** `experiments/ferrous-strings`. Inherits the continuous acoustic-magnetic environment powered by `macroquad` and `ferrous-core`, including vibrating strings and magnetic particles.

**Phenotype (Emergent Behavior):**
Instead of static harmonograph curves or random particles, the Git commit history acts as a sequence of kinetic agents (particles) dropped into the continuous magnetic space. As these commit particles traverse the screen, they are influenced by the magnetic field and physically collide with the strings. The commits pluck the strings, translating the repository's evolution into physical acoustic twangs and generating localized magnetic interference patterns on the platter. The result is a continuous audio-visual translation of codebase activity.

## Running

```bash
cargo run -p git-strings
```

Make sure you run it from the root of a git repository with some history.
