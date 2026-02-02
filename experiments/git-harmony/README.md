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
cargo run -p git-harmony
```

## "Wild Mode" Features
- **Generative Audio:** Uses `rodio` to synthesize sine waves based on the hash of the code content.
- **Visual Synesthesia:** A `ratatui` Canvas visualizes the "notes" flowing through time.
- **Resilient:** Works even without audio hardware (visual-only mode).
