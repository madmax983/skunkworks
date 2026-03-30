# 068. Remove Ferrous Core Facade

* **Status:** Accepted
* **Context:**
  The `ferrous-core` crate was originally created (ADR 067) to centralize logic for magnetic field and fluid density simulations across the Ferrous ecosystem. However, its implementation simply consisted of re-exporting the `Platter` struct from the `platter` crate. This created a redundant "Micro-crate" that served only as an unnecessary indirection layer, violating the Atlas philosophy against creating crates for a single function.

* **Decision:**
  Deleted the `ferrous-core` crate and updated all dependent experiments to directly depend on and use the `platter` crate.

* **Consequences:**
  - **Positive:** Reduces workspace complexity by eliminating a redundant middleman crate. Flattens the dependency graph and aligns with YAGNI principles.
  - **Negative:** Dependent crates now rely directly on `platter`, requiring a direct understanding of the `platter` library rather than an abstracted `ferrous-core` domain facade.
