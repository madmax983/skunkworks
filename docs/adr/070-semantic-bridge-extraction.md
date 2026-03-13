# 070. Semantic Bridge Extraction

## Status
Accepted

## Context
The `crates/tui-shared/src/semantic.rs` file grew to ~565 lines long, acting as a "Blob" module. It mixed several distinct domain concepts (`Entity`, `Region`, `Snapshot`, `Action`, and `PropValue`) into a single file, making it harder to navigate and maintain.

## Decision
Extracted the individual types into separate modules (`entity.rs`, `region.rs`, `snapshot.rs`, `action.rs`) under the new directory `crates/tui-shared/src/semantic/`. The original `crates/tui-shared/src/semantic.rs` was converted into a facade module (`semantic/mod.rs`) that publicly exports (`pub use`) these extracted types.

## Consequences

### Positive
*   **High Cohesion:** Each module now focuses on a single domain concept.
*   **Maintainability:** File lengths are reduced, making the codebase easier to read and modify.
*   **Backwards Compatibility:** By using a facade module (`semantic/mod.rs`), consumer imports remain unbroken.

### Negative
*   **Slight Overhead:** Finding the actual implementation of a type now requires following the `pub use` exports to the respective submodule, adding a small cognitive overhead for new developers navigating the crate.
