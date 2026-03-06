# 066. Extract Miller Lattice

* **Status:** Proposed
* **Context:**
  The `lattice.rs` logic containing `LatticePoint`, `Atom`, and `Crystal` was duplicated across three experiments (`miller-fs`, `miller-reaction`, and `ferro-file`), resulting in unnecessary code duplication, tangling the codebase, and violating the single source of truth principle.

* **Decision:**
  Extracted the common lattice scanning and representation logic into a shared workspace library crate, `crates/miller-lattice`. Refactored `miller-fs`, `miller-reaction`, and `ferro-file` to depend on this new crate. Also implemented `Default` for `Crystal` to satisfy Clippy.

* **Consequences:**
  - **Positive:** Code duplication is eliminated, enforcing a single source of truth for the lattice logic. It ensures consistent types across these experiments and prevents structural drift.
  - **Negative:** Adds a minor dependency link between these experiments and the new shared crate, slightly increasing workspace graph complexity.
