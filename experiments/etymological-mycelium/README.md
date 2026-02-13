# Etymological Mycelium 🍄📜

> "The Etymology of Code visualized as a Fungal Network."

A hybrid experiment splicing `git-etymologist` with `chaotic-mycelium`.

## 🧬 Lineage

*   **Parent A:** `experiments/git-etymologist` (Parsing Git history, tracking line evolution).
*   **Parent B:** `experiments/chaotic-mycelium` (Fungal growth, branching, organic paths).
*   **Novel Trait:** **Semantic Mycology**. The code grows.
    *   **Stable Code:** Represented by straight, healthy green hyphae.
    *   **Mutating Code:** Represented by chaotic, branching red/yellow hyphae.
    *   **Deleted Code:** Dead ends.

## 🔬 How it works

1.  **Spores:** Every line of code in the first commit is a spore.
2.  **Growth:** Spores grow towards the corresponding line in the next commit.
3.  **Substrate:** The "nutrient field" is the Levenshtein distance between the lines.
    *   High similarity = Low resistance, fast growth.
    *   Low similarity = High resistance, chaotic growth.
4.  **Network:** Over time, the commit history forms a dense mycelial mat, showing the "health" and "volatility" of the codebase.

## 🎮 Controls

*   **Left Click + Drag:** Pan the camera.
*   **Mouse Wheel:** Zoom in/out.

## 📦 Run

```bash
cargo run -p etymological-mycelium -- [path_to_repo] [file_path]
```

Example:
```bash
cargo run -p etymological-mycelium -- . Cargo.toml
```
