# Chaotic Mycelium

**Lineage:** `bifurcation-probe` × `mycelial-path`

A simulation of fungal growth on a substrate defined by the Lyapunov fractal of a chaotic map.

## Concept

The world is a "Lyapunov Fractal". Each point $(x, y)$ on the grid corresponds to a pair of parameters $(a, b)$ for the Logistic Map $x_{n+1} = r x_n (1 - x_n)$. The parameter $r$ alternates between $a$ and $b$ according to a sequence (e.g., "AB", "AAB", "ABB").

- **Blue Regions:** Stable (Negative Lyapunov Exponent). The fungus finds it easy to grow here.
- **Red/Yellow Regions:** Chaotic (Positive Lyapunov Exponent). The fungus struggles to grow here (high cost).

## Controls

- **Left Click:** Set a new food target.
- **Space:** Regenerate the chaos map with a new random sequence and parameter range.

## Implementation Details

- **Chaos Substrate:** Uses `rayon` to compute the Lyapunov exponent for every pixel in parallel. The result is stored in a texture and used as the cost map for pathfinding.
- **Fungal Network:** Uses A* pathfinding (or a greedy variation) to grow hyphae towards the target. The cost function is derived from the local Lyapunov exponent.

## Hybrid Vigor

This experiment combines the mathematical visualization of chaos (`bifurcation-probe`) with the organic growth algorithms of `mycelial-path`. The result is an organism that visualizes the "path of least resistance" through a chaotic mathematical landscape.
