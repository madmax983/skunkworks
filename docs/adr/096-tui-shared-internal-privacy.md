# 096. Make tui-shared Internal Modules Private

Date: 2026-05-19

## Status
Proposed

## Context
Following the flattening of the `tui-shared` module hierarchy (ADR 093), all semantic submodules (`action.rs`, `entity.rs`, `region.rs`, `snapshot.rs`, etc.) were moved to the crate root. However, these modules were previously declared as `pub mod` (e.g. `pub mod action;`), which exposed their internal paths directly to library consumers (e.g., consumers could access types via both `tui_shared::action::Action` and the re-exported `tui_shared::Action`). This violated the Facade pattern by leaking the internal organization of the crate and potentially allowing consumers to bypass the intended, clean `pub use` public API provided in `lib.rs`.

## Decision
Modified the module declarations in `crates/tui-shared/src/lib.rs` from `pub mod` to `pub(crate) mod` for all submodules. The core structs (`Action`, `Entity`, `Snapshot`, etc.) remain publicly accessible exclusively through the root-level re-exports (e.g., `pub use action::Action;`).

## Consequences

### Positive
*   **Encapsulation:** Strictly enforces the Facade pattern. Consumers must use the clean, top-level API.
*   **Maintainability:** Internal file organization can now be refactored again without breaking downstream code that might have depended on the specific module paths.
*   **Reduced API Surface:** Cleans up the generated documentation and autocompletion suggestions by hiding redundant module paths.

### Negative
*   **Minor Refactoring Need:** Any internal tools or scripts that strictly relied on the `tui_shared::module_name::Type` path structure may need to be updated to use the root namespace.
