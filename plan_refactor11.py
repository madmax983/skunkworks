# What if we extract the grid movement into a function `move_grid_cursor(app_state, dx, dy)`?
# And instead of a giant `match app_state.view_mode`, we keep the `match key_code { ... }` but replace the 187 `if app_state.grid_cursor.1 < 15 ...` with `move_grid_cursor(app_state, 0, 1)`?
# Wait, replacing 5 lines with 1 line still means we have a giant 1000-line function with hundreds of cases.
# "God Functions: Functions over 50 lines long (Needs extraction)."

# We can create `handle_grid_navigation_down(app_state)`, `handle_genome_navigation(app_state, key_code, vm)`, etc.
# Or better: Create a `fn handle_directional_input(key_code, app_state)` that encapsulates grid movement.
# But grid movement is ONLY for some view modes!

# The easiest and most idiomatic refactoring:
# Create a trait or method on `ViewMode` (or a helper function in `navigation.rs`):
#
# ```rust
# fn is_grid_navigable(mode: &ViewMode) -> bool {
#     matches!(mode, ViewMode::Grid | ViewMode::BioticChaos)
#     || { #[cfg(feature = "nova")] { matches!(mode, ViewMode::Kaleidoscope | ViewMode::Metazoa | ...) } #[cfg(not(feature="nova"))] { false } }
#     || { #[cfg(feature = "elektra")] { matches!(mode, ViewMode::Elektra) } #[cfg(not(feature="elektra"))] { false } }
#     || { #[cfg(feature = "silicon")] { matches!(mode, ViewMode::Foundry) } #[cfg(not(feature="silicon"))] { false } }
# }
# ```
# If `is_grid_navigable` is true, we do grid navigation for all 4 arrows!
# Wait, if we do that, we can remove 187 match arms completely from `handle_navigation_input` !
