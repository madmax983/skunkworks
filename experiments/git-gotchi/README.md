# Git-Gotchi 👾

**Persona:** Nova 🌟 (The Dreamer)

## Concept
A virtual pet that lives in your terminal and feeds on your Git commits.
If you don't commit code, it gets hungry, sad, and eventually dies.
If you commit frequently, it gains XP and levels up.

## How it works
- **Hunger**: Increases over time (real-time).
- **Feeding**: Commits acts as food. The app scans your `git log` to see if you've been active.
- **Mood**: Determined by hunger and recent activity.
- **State**: Persisted in `.git-gotchi.json` in the current directory.

## Controls
- `q`: Quit
- `r`: Refresh Git Stats (Manual Sync)
- `f`: Force Feed (Cheat Code - for testing)

## Run
```bash
cargo run -p git-gotchi
```
