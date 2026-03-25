# 🌋 Tectonic Git

> "The history of code is written in strata, but the debts we incur create the fault lines." - The Splice Surgeon

A hybrid experiment visualizing Git history as geological layers, where vulnerability keywords (`TODO`, `unwrap`, `panic`) act as seismic stress that fractures the landscape.

## 🧬 Lineage

This experiment is a cross between:

*   **Parent A:** `geologic-git` (The Geologist)
    *   *Inherited Trait:* Representation of Git commits as sediment/strata accumulating over time.
    *   *Allele:* `GitHistory` parsing logic (adapted for diff scanning).
*   **Parent B:** `fissure-tracker` (Nova)
    *   *Inherited Trait:* Visualization of code "stress" as jagged fissures.
    *   *Allele:* Vector-based fracture generation algorithms.

## 🔬 Emerging Phenotype

The hybrid exhibits a novel behavior: **Seismic Visualization of Technical Debt**.
Instead of just seeing a timeline or a static count of TODOs, `tectonic-git` shows how specific commits introduced structural instability that persists and grows through the geological record of the project.

## 🕹️ Controls

*   `Space`: Pause/Resume history playback
*   `+/-`: Increase/Decrease playback speed
*   `z/x`: Zoom In/Out
*   `Arrow Keys`: Scroll through the strata (time travel)
*   `q`: Quit

## 🏗️ Running

```bash
cargo run -p tectonic-git
```
