# The only views with grid movement but NO other keys or different behavior:
# Nova: Garden, Orca, Virology, BioMesh, Hydra, Metazoa, Void, Signals, Hologram, Chronos, Kaleidoscope, Logos, Biolum, Reactor, Ecology, Spectrogram, Sovereignty
# Silicon: Foundry
# Elektra: Elektra
# None: Grid, BioticChaos
# Plus, Genesis in Nova: moves grid if focus == 2.

# I can just implement:
# fn move_grid_cursor(app_state: &mut AppState, dx: i32, dy: i32) {
#    let nx = (app_state.grid_cursor.0 as i32 + dx).clamp(0, 15) as usize;
#    let ny = (app_state.grid_cursor.1 as i32 + dy).clamp(0, 15) as usize;
#    app_state.grid_cursor = (nx, ny);
# }
# Wait, let's implement `impl ViewMode { pub fn is_grid_navigable(&self) -> bool { ... } }` in `state.rs`.

# Then in `navigation.rs`, I can do:
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
# Wait! This skips `ViewMode::Genesis` which has *conditional* grid movement (`app_state.genesis_focus == 2`). `Genesis` does NOT return true from `is_grid_navigable()`. Instead, `Genesis` is handled separately in the match.
# Wait! If I do `return Ok(false)`, then ALL other keys (like non-navigation keys) would be ignored?
# No, `handle_navigation_input` ONLY handles Arrow keys (Up, Down, Left, Right).
# Wait, in `normal/mod.rs`, `handle_navigation_input` is ONLY called if `matches!(key.code, KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right)`.
# So YES, we can do:
