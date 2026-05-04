1. **Evaluate Phase 1**: Checked the latest attempted crosses in memory. Memory shows `flock-platter`, `neuro-platter`, and `market-platter` successfully compiled, proving that embedding discrete simulations into continuous scalar fields produces visually rich heatmaps. We will briefly note this evaluation.
2. **Phase 2 (Create New Hybrid)**: Propose and implement `git-platter`.
   - Concept: "Git History Heatmap". Code modifications heat/cool a continuous 2D scalar field to visualize repo evolution.
   - We will create `experiments/git-platter/`.
3. **Implementation Details**:
   - `Cargo.toml`: Depend on `git-associates`, `platter`, `ratatui`, `tui-shared`, `crossterm`, `tokio`, `anyhow`.
   - `src/main.rs`: Read git history (e.g., using `GitModel::history_with_diffs`). Loop through the history step by step. Each commit's insertions/deletions mapped to coordinates on the 2D platter grid. Since we only have files, we can assign a random or hash-based coordinate to each file to build the map over time, or use a spiral layout. Wait, simpler: map `file.path` hash to `(x, y)` coordinate. Insertions = `accumulate(x, y, amount)`, Deletions = `accumulate(x, y, -amount)`.
   - Render the `platter` grid using `tui-shared` or native Ratatui blocks.
4. **Lineage Documentation**: Update `MUTATIONS.md` by moving `git-platter` to Attempted Crosses (if it's not already proposed, we'll just write it as an attempted cross directly since the list of proposed is empty). Provide clear Concept, Novel Trait, Predicted Phenotype.
5. **GUESTBOOK**: Leave a trail.
6. **Pre-commit Checks**: Run `cargo check`, `cargo fmt`, `cargo clippy`, and tests.
7. **Commit & Submit**: `🧬 Splice: Cross git-associates × platter`
