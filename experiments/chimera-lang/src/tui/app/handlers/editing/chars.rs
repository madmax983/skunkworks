use crate::vm::ChimeraVM;
use super::super::super::state::{AppState, ViewMode};
use anyhow::Result;

pub(crate) fn handle_char(
    c: char,
    vm: &mut ChimeraVM,
    app_state: &mut AppState,
) -> Result<bool> {
    let _ =
                        {
                            #[cfg(feature = "nova")]
                            if let ViewMode::Paradox = app_state.view_mode {
                                app_state.paradox_editor_buffer.push(c);
                            } else if let ViewMode::Forge = app_state.view_mode {
                                if app_state.forge_focus == 1 {
                                    app_state.forge_editor_buffer.push(c);
                                } else if app_state.forge_focus == 2 {
                                    app_state.forge_test_input.push(c);
                                } else if app_state.forge_focus == 0 {
                                    app_state.forge_selected_rule.push(c);
                                }
                            } else if let ViewMode::Genesis = app_state.view_mode {
                                if app_state.genesis_focus == 0 {
                                    app_state.genesis_editor_buffer.push(c);
                                } else if app_state.genesis_focus == 1 {
                                    app_state.genesis_grammar_buffer.push(c);
                                } else {
                                    app_state.input_buffer.push(c);
                                }
                            } else if let ViewMode::Babel = app_state.view_mode {
                                let target = if app_state.babel_focus == 0 {
                                    &mut app_state.babel_pattern
                                } else {
                                    &mut app_state.babel_input
                                };
                                target.push(c);
                            } else if let ViewMode::Crispr = app_state.view_mode {
                                if app_state.crispr_focus == 1 {
                                    app_state.crispr_guide.push(c);
                                } else if app_state.crispr_focus == 2 {
                                    app_state.crispr_replace.push(c);
                                }
                            } else {
                                app_state.input_buffer.push(c);
                            }
                        };
    Ok(true)
}
