# Amazing, we drop 700 lines!
# But to make this actually work in Rust, we have to PRE-HANDLE grid navigation for those views!
# So before the big match, we just do:
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
# Then all views that returned `true` for `is_grid_navigable()` will NOT reach the big `match key_code`.
# But wait, does ANY view need both grid movement AND other keys inside `navigation.rs`?
# NO, `navigation.rs` ONLY receives Arrow keys.
# So ANY view that ONLY moves grid in ALL 4 directions can just return true.
# What about `ViewMode::Genesis`? It has:
# `if app_state.genesis_focus == 2 && app_state.grid_cursor.1 < 15 { app_state.grid_cursor.1 += 1; }`
# It will NOT return true for `is_grid_navigable()`. So it will fall through to the remaining `match app_state.view_mode` block!
# This is PERFECT!
