use crate::vm::ChimeraVM;
use super::super::super::state::{AppState, ViewMode};
use anyhow::Result;

pub(crate) fn handle_char(
    c: char,
    _vm: &mut ChimeraVM,
    app_state: &mut AppState,
) -> Result<bool> {
    #[cfg(feature = "nova")]
    match app_state.view_mode {
        ViewMode::Paradox => {
            app_state.paradox_editor_buffer.push(c);
        }
        ViewMode::Forge => {
            match app_state.forge_focus {
                1 => app_state.forge_editor_buffer.push(c),
                2 => app_state.forge_test_input.push(c),
                0 => app_state.forge_selected_rule.push(c),
                _ => {}
            }
        }
        ViewMode::Genesis => {
            match app_state.genesis_focus {
                0 => app_state.genesis_editor_buffer.push(c),
                1 => app_state.genesis_grammar_buffer.push(c),
                _ => app_state.input_buffer.push(c),
            }
        }
        ViewMode::Babel => {
            if app_state.babel_focus == 0 {
                app_state.babel_pattern.push(c);
            } else {
                app_state.babel_input.push(c);
            }
        }
        ViewMode::Crispr => {
            match app_state.crispr_focus {
                1 => app_state.crispr_guide.push(c),
                2 => app_state.crispr_replace.push(c),
                _ => {}
            }
        }
        _ => {
            app_state.input_buffer.push(c);
        }
    }

    #[cfg(not(feature = "nova"))]
    app_state.input_buffer.push(c);

    Ok(true)
}
