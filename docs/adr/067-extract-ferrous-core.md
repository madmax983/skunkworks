# 067. Extract Ferrous Core

* **Status:** Proposed
* **Context:**
  Multiple experiments in the "Ferrous" series (`ferrous-fluid`, `ferrous-chimera`, `ferrous-neuron`, `ferrous-graph`, `ferrous-hologram`, `ferrous-strings`, `git-hologram`, `chaos-strings`, `gray-fluid`, `chron-fluid`, `myco-fluid`) share core logic and data structures for magnetic field and fluid density simulations. Keeping these structures duplicated across experiments leads to code bloat and maintenance overhead.

* **Decision:**
  Extracted the common logic and data structures (such as re-exporting `Platter` from `crates/platter`) into a centralized workspace library crate: `crates/ferrous-core`. Refactored dependent experiments to use this shared library.

* **Consequences:**
  - **Positive:** Centralizes magnetic field and fluid density logic into a single reusable core library. Enhances code reusability and reduces duplication.
  - **Negative:** Introduces a dependency coupling between `ferrous-core` and the various Ferrous experiments, increasing the complexity of the workspace dependency graph.
