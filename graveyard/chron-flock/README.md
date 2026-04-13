# Chron-Flock 🦋

**Status:** FRESH
**Lineage:** `chrontext` × `locus`
**Concept:** Codebase Swarming. A TUI visualization where boids flock around lines of code.

## Description

This experiment crosses the Git blame chronological age parsing of `chrontext` with the 2D boid flocking logic of `locus`.
The visual layout shows the source code on the left, colored by its chronological age (from cold/blue/old to hot/red/new).
On the right, a swarm of boids moves around a canvas.

- **Agents:** Locus Boids simulating flocking behavior (Separation, Alignment, Cohesion).
- **Landscape:** The terminal space represents a 2D Torus.
- **Rhythm:** As you scroll through the codebase, the "freshness" of the lines of code (their `age_score` from `chrontext`'s blame analyzer) exerts an attractive or repulsive force on the boids. New code attracts the swarm, while old code gently repels it.

## Controls

- **Q**: Quit
- **Up / K**: Scroll up
- **Down / J**: Scroll down

## Run

```bash
cargo run --bin chron-flock <path_to_file>
```

## Lineage

- **Parent A (chrontext):** Provided the `git2` based blame parsing, mapping line history to an `age_score` (0.0 to 1.0).
- **Parent B (locus):** Provided the continuous 2D `Topology::Torus` spatial representation and `compute_force` boid logic.
- **Novelty:** Codebase Swarming. A discrete non-linear particle system that is continuously and interactively affected by the static, historical metadata of a Git repository.

## Credits

Splice Surgeon 🧬
