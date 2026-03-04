# Forge's Journal

## [Refactor Platter Logic]
**Learning:** `Platter` had repeated bounds checking and magic numbers (`0.001`, `1.0`). Extracting `get_index` and constants `DECAY_THRESHOLD` / `SATURATION_LIMIT` improved readability and centralized logic.
**Action:** Look for other grid-based structures in `ferrous-core` or `chimera-lang` that might benefit from similar `get_index` helpers or constant extraction.

## [Workspace Dependency Ambiguity]
**Learning:** `bevy_reflect` failed to compile due to `glam` version ambiguity in the workspace. Explicitly adding `glam = "0.27.0"` to `platter/Cargo.toml` (even if not strictly needed for `platter`'s own tests) can help resolve resolution conflicts in the wider workspace.
**Action:** When seeing "cannot find type" errors in `bevy_reflect` related to `glam`, check for multiple `glam` versions in the dependency tree and pin the version if necessary.

## [Refactor Wrong Self Convention]
**Learning:** In Rust, methods starting with `to_` on types that implement `Copy` generally shouldn't take `&self` by reference. They should take `self` by value. This is caught by Clippy's `wrong_self_convention` lint. The correct naming for converting a non-Copy type by reference is `to_`, and for converting by value is `into_`. For a Copy type, taking it by value is cheap and idiomatic.
**Action:** When defining `to_*` or `into_*` methods, consider if the type is `Copy`. If it is, use `into_*` or `to_*` and take `self` by value.
