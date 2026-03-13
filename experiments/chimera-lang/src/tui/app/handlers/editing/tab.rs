use super::super::super::state::{AppState, InputMode, ViewMode};
use super::super::super::get_all_views;
use anyhow::Result;

pub(crate) fn handle_tab_input(
    app_state: &mut AppState,
) -> Result<bool> {

                        {
                            #[cfg(feature = "nova")]
                            if let ViewMode::Babel = app_state.view_mode {
                                app_state.babel_focus = (app_state.babel_focus + 1) % 2;
                            } else if let ViewMode::Crispr = app_state.view_mode {
                                app_state.crispr_focus = (app_state.crispr_focus + 1) % 3;
                            }
                        }
    Ok(true)
}
