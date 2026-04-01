# Look at `ViewMode` methods. There's none right now in state.rs for grid navigation.
# I can implement `app_state.view_mode.supports_grid_cursor()` in state.rs, OR I can just do a match with multiple patterns inside `navigation.rs` but outside the main `match key_code` match or inside it but group all those modes.

# Alternatively, the most straightforward Forge refactoring for "Pyramid of Doom" and "God Functions" is to extract the giant `match key_code { ... }` into smaller, descriptive helper functions like:
# fn handle_down(vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool>
# fn handle_up(vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool>
# fn handle_left(...)
# fn handle_right(...)

# BUT inside `handle_down`, there's still a huge match.
# Let's write a python script to rewrite `navigation.rs`.
