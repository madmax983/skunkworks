use crate::vm::ChimeraVM;
use super::super::super::state::{AppState, InputMode, ViewMode};
use anyhow::Result;
use crossterm::event::KeyCode;

pub(crate) fn handle_action(
    code: KeyCode,
    vm: &mut ChimeraVM,
    app_state: &mut AppState,
) -> Result<bool> {
    match code {
        KeyCode::Tab =>
                        {
                            #[cfg(feature = "nova")]
                            if let ViewMode::Babel = app_state.view_mode {
                                app_state.babel_focus = (app_state.babel_focus + 1) % 2;
                            } else if let ViewMode::Crispr = app_state.view_mode {
                                app_state.crispr_focus = (app_state.crispr_focus + 1) % 3;
                            }
                        }
        KeyCode::Esc => {
                            app_state.input_mode = InputMode::Normal;
                            app_state.input_buffer.clear();
                        }
        KeyCode::Backspace =>
                        {
                            #[cfg(feature = "nova")]
                            if let ViewMode::Paradox = app_state.view_mode {
                                app_state.paradox_editor_buffer.pop();
                            } else if let ViewMode::Forge = app_state.view_mode {
                                if app_state.forge_focus == 1 {
                                    app_state.forge_editor_buffer.pop();
                                } else if app_state.forge_focus == 2 {
                                    app_state.forge_test_input.pop();
                                } else if app_state.forge_focus == 0 {
                                    app_state.forge_selected_rule.pop();
                                }
                            } else if let ViewMode::Genesis = app_state.view_mode {
                                if app_state.genesis_focus == 0 {
                                    app_state.genesis_editor_buffer.pop();
                                } else if app_state.genesis_focus == 1 {
                                    app_state.genesis_grammar_buffer.pop();
                                } else {
                                    app_state.input_buffer.pop();
                                }
                            } else if let ViewMode::Babel = app_state.view_mode {
                                let target = if app_state.babel_focus == 0 {
                                    &mut app_state.babel_pattern
                                } else {
                                    &mut app_state.babel_input
                                };
                                target.pop();
                            } else if let ViewMode::Crispr = app_state.view_mode {
                                if app_state.crispr_focus == 1 {
                                    app_state.crispr_guide.pop();
                                } else if app_state.crispr_focus == 2 {
                                    app_state.crispr_replace.pop();
                                }
                            } else {
                                app_state.input_buffer.pop();
                            }
                        }
        _ => return Ok(false),
    }
    Ok(true)
}
