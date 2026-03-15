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

## [Refactor God Functions and Deep Nesting]
**Learning:** Functions like `App::update`, `BlameAnalyzer::analyze`, and `draw_text_pane` suffered from mixing unrelated logic (e.g. calculation of code force, rendering UI logic) causing deep nesting. Extracting well-named functions (like `calculate_text_force`, `process_hunks`, and `format_line`) significantly simplifies reading flow and makes logic testing easier.
**Action:** When working with rendering loops or algorithms, aggressively extract block logic into smaller cohesive units to prevent pyramid-of-doom and simplify parent functions.

## [Struct Extraction Slice Coercion]
**Learning:** When performing "Struct Extraction" to group function arguments into a new struct (e.g. `ElektraArgs<'a>`), Rust does not perform implicit slice coercion (Deref coercion) during struct field initialization. If a variable is passed as a `&mut Vec<T>` and the field expects a `&mut [T]`, using field initialization shorthand (e.g., just `voltage_grid,`) will cause a `mismatched types` error.
**Action:** Explicitly deref and borrow the vector to coerce it into a slice when constructing the struct: `voltage_grid: &mut *voltage_grid,`.
