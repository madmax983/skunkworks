# Cargo Air Traffic Control (ATC)

**Cargo ATC** is a TUI game where you play as the scheduler for the Rust compiler.

## Concept
Dependencies are incoming "flights". Your CPU threads are "runways".
You must schedule dependencies to be built (landed) in the correct topological order.
If a dependency isn't ready (its dependencies aren't built), you can't schedule it.

## Controls
- **Up/Down**: Select a crate from the "Ready" queue.
- **1-4**: Assign the selected crate to Runway 1-4.
- **Q**: Quit.

## Mechanics
- **Weight**: Crate build time is determined by name length (heuristic).
- **Blocking**: You cannot build `B` until `A` is finished if `B` depends on `A`.
