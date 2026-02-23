# ⚛️ Git Harmony

> "The code is the music." - Genesis

Git Harmony is a synesthetic translator that turns your git diffs into an ambient audiovisual experience.

## Concept
It reads the current `git diff`, parsing additions and deletions as musical events.
- **Additions (+):** High pitch, Green/Cyan, Major scale.
- **Deletions (-):** Low pitch, Red/Magenta, Minor scale.
- **Context:** Mid pitch, Blue.

## Usage
Run inside a git repository (or this one):

```bash
# Visual-only mode (Default)
cargo run -p git-harmony

# Audio-visual mode (Requires audio hardware)
cargo run -p git-harmony --features audio
```

## "Wild Mode" Features
- **Generative Audio:** Uses `rodio` to synthesize sine waves based on the hash of the code content (Requires `--features audio`).
- **Visual Synesthesia:** A `ratatui` Canvas visualizes the "notes" flowing through time.
- **Resilient:** Works even without audio hardware (visual-only mode).
