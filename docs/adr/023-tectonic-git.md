# 23. Tectonic Git (Geological Code Analysis)

Date: 2024-05-24

## Status

Accepted

## Context

Traditional version control history visualization tools (like `git log --graph` or GUI clients) focus on the *topology* of branches and merges. They often fail to convey the **qualitative** nature of the changes.

- A commit with 1000 lines changed feels the same in the log as a commit with 1 line changed.
- Periods of high instability (rapid "fix" commits, many TODOs added) look identical to periods of stable feature development.
- Technical debt is invisible until you open the files.

We need a way to visualize the "weight", "friction", and "instability" of the codebase history in an intuitive manner.

## Decision

We will implement `tectonic-git`, a TUI experiment that maps Git history to a geological simulation.

### The Geological Metaphor

1.  **Strata (Layers):** Each commit is represented as a horizontal layer of rock. The thickness or color can represent the size or age of the commit.
2.  **Stress (Pressure):** We calculate a "Stress Level" for each commit based on the content of the diff.
    - `TODO` / `FIXME` -> Adds Stress (Unresolved debt).
    - `panic!` / `unwrap()` -> Adds High Stress (Potential runtime failure).
    - `unsafe` -> Adds Moderate Stress (Memory safety risk).
3.  **Fissures (Cracks):** When a layer has high stress, it triggers the generation of "Fissures". These are jagged lines that grow through the strata, visually representing the instability of that period.
4.  **Tectonic Shift:** Commits with high stress cause horizontal displacement (earthquakes) in subsequent layers, disrupting the visual flow.

### Technical Implementation

- **Scanner:** `git-associates` (or internal module) scans `git log -p` to parse diff hunks.
- **Simulation:** A `World` struct manages a collection of `Strata` and `Fissure` objects.
- **Visualization:** `ratatui` renders the cross-section of the "earth".

## Consequences

### Positive
- **Intuitive Health Check:** A glance at the "geology" reveals if a project is "solid bedrock" or "crumbling shale".
- **Gamification:** Encourages developers to reduce "stress" (remove TODOs/panics) to smooth out the visualization.
- **Aesthetic:** Provides a unique, organic way to view code history.

### Negative
- **Performance:** Scanning diff content for every commit is slower than just reading the commit graph.
- **Heuristics:** The "Stress" definition is subjective and may flag legitimate code (e.g., `unsafe` in FFI code) as "bad".
