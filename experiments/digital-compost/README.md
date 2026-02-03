# Digital Compost 🍂

> "Data doesn't last forever. Magnetic domains flip. Links rot. Code decays."

**Digital Compost** is a visualization of software entropy. It scans the current git repository and determines the "freshness" of each source file based on its last commit timestamp.

The older the file, the more it "decays" when viewed.

## Features

- **Entropy Scanning**: Uses `git log` to map the age of every file in the repository.
- **Visual Decay**: Text corruption algorithms simulate bit rot:
    - **FRESH** (< 1 day): Pristine code.
    - **STABLE** (< 1 week): Occasional bit flips.
    - **MOLDY** (< 1 month): Block characters and fungal growth.
    - **ROTTING** (< 6 months): Glitches and missing chunks.
    - **COMPOST** (> 1 year): Barely readable digital mulch.
- **TUI Interface**: Browse your rotting codebase in a terminal.

## Usage

```bash
cargo run --bin digital-compost
```

Controls:
- `j` / `k` / `Up` / `Down`: Navigate file list.
- `h` / `l` / `Left` / `Right`: Scroll content.
- `r`: Refresh entropy (re-roll corruption RNG).
- `q`: Quit.

## Concept

This experiment smashes together **Bit rot simulation** and **Git history reconstruction**. It serves as a reminder that code is a living organism that requires maintenance; without it, it returns to the void.
