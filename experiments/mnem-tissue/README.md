# Mnem Tissue 🧫

> "The codebase graph physically wrestles with its own technical debt."

**Mnem Tissue** is a hybrid experiment splicing `mnem-rot` with `ferrous-tissue`.

## 🧬 Lineage

*   **Parent A:** `experiments/mnem-rot` (Abstract structural codebase decay visualization).
*   **Parent B:** `experiments/ferrous-tissue` (Soft-body physics dynamics via `physics_pbd`).
*   **Novel Trait:** **Tissue Necrosis**. As codebase files (represented as nodes/cells) increase in entropy, their physical constraints weaken and expand. The codebase physically sags, bloats, and eventually bursts apart under its own technical debt. The static structural data is directly translated into a biological soft-body system.

## 🔬 How it works

1.  **Nodes:** Represents individual files from the local Git repository's HEAD tree.
2.  **Constraints:** Files within the same directory share soft-body distance constraints, creating interconnected "tissue".
3.  **Rot:** Random "decay" strikes nodes, increasing their entropy. High entropy causes swelling, violent shaking, and physical deformation of the tissue.
4.  **Maintenance:** Occasional random "refactoring" restores health, tightening the tissue back up.

## 🎮 Controls

*   **No Controls:** Watch the tissue breathe and rot automatically.

## 📦 Run

```bash
cargo run -p mnem-tissue -- [path_to_repo]
```

Example:
```bash
cargo run -p mnem-tissue -- .
```
