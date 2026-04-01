# If that is the most macro/cfg-friendly way, how can we reduce the length of the function?
# We can extract the match into 4 helper functions:
# `fn handle_down_navigation(vm: &mut ChimeraVM, app_state: &mut AppState)`
# `fn handle_up_navigation(...)`
# ...
# Or we can extract the `if app_state.grid_cursor.1 < 15 { app_state.grid_cursor.1 += 1; }` into a macro or inline function.
# But even with helper functions, `handle_down_navigation` is still a 200+ line function, just split across 4 functions. It doesn't reduce duplication of the `if app_state.grid_cursor...` block, which appears ~80 times.

# We can define a trait or a helper function on `ViewMode` in `state.rs`:
# ```rust
# impl ViewMode {
#     pub fn is_grid_navigable(&self) -> bool {
#         match self {
#             ViewMode::Grid => true,
#             #[cfg(feature = "nova")]
#             ViewMode::Kaleidoscope => true,
#             #[cfg(feature = "elektra")]
#             ViewMode::Elektra => true,
#             ...
#             _ => false,
#         }
#     }
# }
# ```
# Then in `handle_navigation_input`, we can just do:
# ```rust
# if app_state.view_mode.is_grid_navigable() {
#     match key_code {
#         KeyCode::Down => if app_state.grid_cursor.1 < 15 { app_state.grid_cursor.1 += 1; },
#         KeyCode::Up => if app_state.grid_cursor.1 > 0 { app_state.grid_cursor.1 -= 1; },
#         KeyCode::Right => if app_state.grid_cursor.0 < 15 { app_state.grid_cursor.0 += 1; },
#         KeyCode::Left => if app_state.grid_cursor.0 > 0 { app_state.grid_cursor.0 -= 1; },
#         _ => {}
#     }
#     return Ok(false);
# }
# ```
# But wait, there are a few view modes that have `is_grid_navigable` AND some other logic!
# For example, `ViewMode::Genesis`:
# ```rust
# ViewMode::Genesis => {
#     if app_state.genesis_focus == 2 && app_state.grid_cursor.1 < 15 {
#         app_state.grid_cursor.1 += 1;
#     }
# }
# ```
# It only moves the grid if `genesis_focus == 2`.
