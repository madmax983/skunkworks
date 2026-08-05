sed -i '1i //! # Narrative API\n//!\n//! Provides a fluent builder pattern to simplify the construction of a `ChimeraVM`\n//! populated with narrative elements, logic grids, and genes.\n//!\n//! This module abstracts away the low-level complexities of raw grid and AST manipulation.' experiments/chimera-lang/src/narrative.rs

# inject readme into git-locus
cat << 'EOF3' > experiments/git-locus/src/main.rs.new
//! # Git Locus
//!
//! A hybrid experiment crossing `git-associates` with `locus`.
//!
//! **Lineage:**
//! - From `git-associates`: Discrete Git commit metadata parsing, measuring churn and developer intent over time.
//! - From `locus`: Continuous non-Euclidean boundary wrapping (Torus, Klein Bottle, Mobius, etc.)
//!
//! **Phenotype:**
//! The chronological commit history is projected onto a continuous 2D boundary grid governed by topological rules. As codebase activity expands past visual Euclidean boundaries, the history naturally wraps and intersects itself. This creates overlapping "ghost timelines" of the repository's evolution, allowing observers to see how deeply coupled files interact with each other even when separated by immense linear time.
EOF3
cat experiments/git-locus/src/main.rs >> experiments/git-locus/src/main.rs.new
mv experiments/git-locus/src/main.rs.new experiments/git-locus/src/main.rs

RUSTDOCFLAGS="-W missing_docs -W rustdoc::missing_crate_level_docs -D warnings" cargo doc --no-deps --workspace 2>&1 | grep "missing documentation" || true
