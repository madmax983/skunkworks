# Git Cantata ⚛️🎨🎵

**"The Song of the Commit"**

A cross-sensory experiment that translates Git history into generative ambient music and abstract expressionist visual art.

## Concept
Every commit is a musical event.
- **Pitch**: Derived from the Commit Hash.
- **Timbre**: Derived from file extensions (Rust = Sine, TOML = Square, etc.).
- **Dynamics**: Derived from insertions/deletions.
- **Visuals**: A particle system "paints" the commit onto a canvas, creating a lasting image of the repository's evolution.

## Usage
```bash
cargo run --release -- <path-to-repo>
```
To enable audio (requires ALSA/libasound2 on Linux):
```bash
cargo run --release --features audio -- <path-to-repo>
```

## Controls
- **Space**: Pause/Resume auto-play.
- **n / Right Arrow**: Next Commit.
- **p / Left Arrow**: Previous Commit.
- **q / Esc**: Quit.

## Moonshot Status
- **Green Phase**: Core logic implemented. Audio synthesis is basic but functional. Visuals are generative.
- **Dependencies**: `ratatui` (TUI), `git2` (Git), `rodio` (Audio - Optional).
