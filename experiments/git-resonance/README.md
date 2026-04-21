# Acoustic Git History (`git-resonance`)

**Lineage:** `crates/git-associates` × `crates/resonance-audio`

**Concept:** The discrete, structured data of a git repository's commit history acts as a sequence of physical plucks within a 2D continuous acoustic wave tank.

**Novel Trait:** Continuous Acoustic Translation of Git Intent. By parsing the actual repository (insertions, deletions), we drop acoustic "plucks" into the resonant wave tank. The size of the commit and the type of change determine the strength, location, and polarity of the acoustic pressure wave, resulting in a visual and auditory representation of the repository's evolution over time.

## Usage

```bash
cargo run -p git-resonance -- [path_to_repo]
```

## How it works

1. It parses the target git repository's commit history and diffs using the `git-associates` library.
2. For each commit, it extracts the total insertions and deletions.
3. It maps the commit's hash to a physical spatial location on the 2D physics wave tank.
4. It creates a physical acoustic "pluck" whose strength corresponds to the number of changes and whose polarity (positive/negative pressure) corresponds to whether the change was mostly insertions or deletions.
5. The `resonance-audio` grid propagates these acoustic waves, rendering an evolving snapshot of the codebase history as cymatic-like interference patterns.
