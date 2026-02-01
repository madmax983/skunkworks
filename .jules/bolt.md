# Bolt's Journal

**[Inlining SPH Kernels]**
**Learning:** In N^2 simulation loops, function call overhead and redundant `powi` calculations can dominate. Manual inlining and hoisting constants (like `h^9`) yields ~20% speedup.
**Action:** For hot physics loops, prefer inlining small kernel functions and precomputing all coefficients.
