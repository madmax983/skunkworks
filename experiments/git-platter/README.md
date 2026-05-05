# Git Platter 🕊️🍽️

This experiment crosses `crates/git-associates` with `crates/platter` to explore the concept of a "Git History Heatmap".

## Lineage

*   **Parent A (git-associates)**: Provides git repository parsing, allowing us to extract discrete developer commits, file insertions, and deletions over time.
*   **Parent B (platter)**: Provides the continuous 2D scalar field representation and decay logic.
*   **Novel Trait**: Discrete codebase modifications (commits) dynamically heat and cool a continuous 2D scalar field. Insertions heat the field up (positive saturation), while deletions cool it down (negative accumulation). The field decays over time, allowing us to visualize localized codebase churn.

## Emergent Phenotype
A visual heatmap showing the evolution of a repository. Hotspots indicate massive code additions, while cold spots indicate refactoring and deletions, leaving a fading memory of the development lifecycle.
