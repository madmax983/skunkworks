# git-platter

A genetic cross created by the Splice Surgeon 🧬.

## Lineage
- **Parent A**: `crates/git-associates` (Codebase metadata, commit history, file churn)
- **Parent B**: `crates/platter` (Continuous 2D scalar field, accumulative density, decay)

## Concept
**Git History Heatmap**: A visualization of codebase evolution mapped onto a 2D scalar field. As commits occur, they drop "heat" (insertions and deletions) onto specific coordinates derived from the commit metadata. Over time, this heat decays. The final result is a heatmap of codebase activity—where hot areas represent recent, high-churn development, and cold areas represent stable, untouched code.

## Execution
```bash
cargo run -p git-platter
```
