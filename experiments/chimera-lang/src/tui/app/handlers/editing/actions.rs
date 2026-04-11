use crate::tui::state::{AppState, InputMode, ViewMode};
use crate::vm::ChimeraVM;
use anyhow::Result;
use crossterm::event::KeyCode;

pub(crate) fn handle_action(
    code: KeyCode,
    _vm: &mut ChimeraVM,
    app_state: &mut AppState,
) -> Result<bool> {
    match code {
        KeyCode::Tab =>
        {
            #[cfg(feature = "nova")]
            match app_state.view_mode {
                ViewMode::Babel => {
                    app_state.babel_focus = (app_state.babel_focus + 1) % 2;
                }
                ViewMode::Crispr => {
                    app_state.crispr_focus = (app_state.crispr_focus + 1) % 3;
                }
                _ => {}
            }
        }
        KeyCode::Esc => {
            app_state.input_mode = InputMode::Normal;
            app_state.input_buffer.clear();
        }
        KeyCode::Backspace => {
            #[cfg(feature = "nova")]
            match app_state.view_mode {
                ViewMode::Paradox => {
                    app_state.paradox_editor_buffer.pop();
                }
                ViewMode::Forge => match app_state.forge_focus {
                    1 => {
                        app_state.forge_editor_buffer.pop();
                    }
                    2 => {
                        app_state.forge_test_input.pop();
                    }
                    0 => {
                        app_state.forge_selected_rule.pop();
                    }
                    _ => {}
                },
                ViewMode::Genesis => match app_state.genesis_focus {
                    0 => {
                        app_state.genesis_editor_buffer.pop();
                    }
                    1 => {
                        app_state.genesis_grammar_buffer.pop();
                    }
                    _ => {
                        app_state.input_buffer.pop();
                    }
                },
                ViewMode::Babel => {
                    let target = if app_state.babel_focus == 0 {
                        &mut app_state.babel_pattern
                    } else {
                        &mut app_state.babel_input
                    };
                    target.pop();
                }
                ViewMode::Crispr => match app_state.crispr_focus {
                    1 => {
                        app_state.crispr_guide.pop();
                    }
                    2 => {
                        app_state.crispr_replace.pop();
                    }
                    _ => {}
                },
                _ => {
                    app_state.input_buffer.pop();
                }
            }

            #[cfg(not(feature = "nova"))]
            app_state.input_buffer.pop();
        }
        _ => return Ok(false),
    }
    Ok(true)
}
