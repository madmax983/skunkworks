with open("test_navigation_clean.rs", "r") as f:
    text = f.read()

# We need to prepend the grid movement block.
new_code = """use crate::tui::state::{AppState, InputMode, ViewMode};
use crate::vm::ChimeraVM;
use anyhow::Result;
use crossterm::event::KeyCode;

pub(crate) fn handle_navigation_input(
    key_code: KeyCode,
    vm: &mut ChimeraVM,
    app_state: &mut AppState,
) -> Result<bool> {
    if app_state.view_mode.is_grid_navigable() {
        match key_code {
            KeyCode::Down => {
                if app_state.grid_cursor.1 < 15 {
                    app_state.grid_cursor.1 += 1;
                }
            }
            KeyCode::Up => {
                if app_state.grid_cursor.1 > 0 {
                    app_state.grid_cursor.1 -= 1;
                }
            }
            KeyCode::Right => {
                if app_state.grid_cursor.0 < 15 {
                    app_state.grid_cursor.0 += 1;
                }
            }
            KeyCode::Left => {
                if app_state.grid_cursor.0 > 0 {
                    app_state.grid_cursor.0 -= 1;
                }
            }
            _ => {}
        }
        return Ok(false);
    }
"""

text = text.replace("""pub(crate) fn handle_navigation_input(
    key_code: KeyCode,
    vm: &mut ChimeraVM,
    app_state: &mut AppState,
) -> Result<bool> {""", new_code)

with open("test_navigation_clean.rs", "w") as f:
    f.write(text)
