# Git Ghost 👻

> "The code is dead, long live the code."

**Git Ghost** is a TUI tool that visualizes deleted files in a git repository as "ghosts".
It simulates bit rot and data decay based on how long ago the file was deleted.

## Concept
- **Archaeology**: Scans git history for `Deleted` events.
- **Entropy**: Applies a visual decay filter to the file content. Older ghosts are more corrupted.
- **Resurrection**: Allows you to restore a deleted file (copy it to a new file).

## Usage
Run inside a git repository:
```bash
cargo run -p git-ghost
```

Controls:
- `Up/Down`: Navigate the graveyard.
- `Enter` / `t`: Resurrect the selected ghost (saves to `resurrected_<filename>`).
- `q`: Quit.

## Moonshot Elements
- **CRT Scanlines**: Visual effect to simulate an old monitor.
- **Bit Rot**: Procedural corruption of text based on time delta.
