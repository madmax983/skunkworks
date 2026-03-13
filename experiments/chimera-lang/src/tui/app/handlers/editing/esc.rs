use super::super::super::state::{AppState, InputMode, ViewMode};
use super::super::super::get_all_views;
use anyhow::Result;

pub(crate) fn handle_esc_input(
    app_state: &mut AppState,
) -> Result<bool> {
                         {
                            app_state.input_mode = InputMode::Normal;
                            app_state.input_buffer.clear();
                        }
    Ok(true)
}
