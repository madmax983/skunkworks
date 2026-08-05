cat << 'EOF2' > crates/arthropod/src/button.rs.new
//! # Interactive Immediate Mode Widgets
//!
//! Provides the primary interactive components, specifically the `Button`,
//! designed for zero-boilerplate use in immediate mode UIs within `macroquad`.

EOF2
cat crates/arthropod/src/button.rs >> crates/arthropod/src/button.rs.new
mv crates/arthropod/src/button.rs.new crates/arthropod/src/button.rs

cat << 'EOF3' > crates/poincare-disk/tests/properties.rs.new
//! # Poincaré Disk Property Tests
//!
//! Random generated tests checking Euclidean boundaries and Mobius logic.

EOF3
cat crates/poincare-disk/tests/properties.rs >> crates/poincare-disk/tests/properties.rs.new
mv crates/poincare-disk/tests/properties.rs.new crates/poincare-disk/tests/properties.rs

cat << 'EOF4' > experiments/git-locus/src/main.rs.new
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

EOF4
cat experiments/git-locus/src/main.rs >> experiments/git-locus/src/main.rs.new
mv experiments/git-locus/src/main.rs.new experiments/git-locus/src/main.rs

sed -i '1i //! # Narrative API\n//!\n//! Provides a fluent builder pattern to simplify the construction of a `ChimeraVM`\n//! populated with narrative elements, logic grids, and genes.\n//!\n//! This module abstracts away the low-level complexities of raw grid and AST manipulation.' experiments/chimera-lang/src/narrative.rs

git add crates experiments
