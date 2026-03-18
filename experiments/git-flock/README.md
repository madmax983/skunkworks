# 🧬 git-flock

Codebase Swarming. A TUI visualization crossing the Git commit metadata parsing of `git-harmonograph` with the 2D boid flocking logic of `luminous-flock`.

## Lineage

*   **From `git-harmonograph` (Parent A):** Git repository integration. Parses the commit history to extract commit hashes, authors, and messages, creating discrete data points of codebase activity.
*   **From `luminous-flock` (Parent B):** Swarm intelligence and 2D physics engine. Boids execute steering behaviors based on `Dna` traits (cohesion, alignment, separation).
*   **Novel Trait (Emergence):** The flock treats the git metadata as spatial targets. Instead of just wandering, the boids actively swarm around and "investigate" coordinate hotspots mapped directly from the cryptographic hash of each git commit, creating an organic visual indicator of codebase activity.

## Usage

```bash
cargo run -p git-flock
```
