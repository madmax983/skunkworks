use crate::vm::ChimeraVM;
use crate::tui::state::{AppState, InputMode, ViewMode};
use anyhow::Result;
use crossterm::event::KeyCode;

pub(crate) fn handle_action_input(key_code: KeyCode, vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
    match key_code {
        KeyCode::Tab => {
                        if let ViewMode::Evolution = app_state.view_mode {
                            use crate::vm::evolution::Challenge;
                            app_state.evolution_state.challenge =
                                match app_state.evolution_state.challenge {
                                    Challenge::Target(_) => Challenge::Doubler,
                                    Challenge::Doubler => Challenge::Adder,
                                    Challenge::Adder => Challenge::Fibonacci,
                                    Challenge::Fibonacci => Challenge::Target(42),
                                    Challenge::Custom(_) => Challenge::Target(42), // Fallback/Cycle
                                };
                            if let Some(engine) = &mut app_state.evolution_state.engine {
                                engine.challenge = app_state.evolution_state.challenge.clone();
                            }
                            app_state.status_msg =
                                format!("Challenge set to {}", app_state.evolution_state.challenge);
                            return Ok(true);
                        }

                        #[cfg(feature = "nova")]
                        if let ViewMode::Genesis = app_state.view_mode {
                            app_state.genesis_focus = (app_state.genesis_focus + 1) % 3;
                            return Ok(true);
                        }

                        #[cfg(feature = "nova")]
                        if let ViewMode::Forge = app_state.view_mode {
                            app_state.forge_focus = (app_state.forge_focus + 1) % 3;
                            return Ok(true);
                        }

                        app_state.view_mode = app_state.view_mode.next_view();
                    }
        _ => {}
    }
    Ok(false)
}
