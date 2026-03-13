pub(crate) mod chars;
pub(crate) mod navigation;
pub(crate) mod actions;

use crate::ast::Gene;
use crate::vm::ChimeraVM;
use crate::{ChimeraParser, Rule};
use super::super::state::{AppState, InputMode, ViewMode};
use super::super::get_all_views;
use super::super::parse_grid_value;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use pest::Parser;

pub(crate) fn handle_normal_input(
    key: KeyEvent,
    vm: &mut ChimeraVM,
    app_state: &mut AppState,
) -> Result<bool> {
                #[cfg(feature = "nova")]
                if let KeyCode::Char(c) = key.code {
                    if app_state.view_mode == ViewMode::Orca
                        || app_state.view_mode == ViewMode::Prologue
                    {
                        if c == ' ' {
                            // Let Space fall through
                        } else if c.is_ascii_graphic() {
                            let (x, y) = app_state.grid_cursor;
                            vm.grid[y][x] = crate::vm::Value::Str(c.to_string());
                            return Ok(true);
                        }
                    }

                    if c != 'q'
                        && c != ' '
                        && c != 'm'
                        && c != 'c'
                        && c != 'C'
                        && c != 'K'
                        && vm.handle_input(c)
                    {
                        return Ok(true);
                    }
                }


                if let KeyCode::Char(c) = key.code {
                    if crate::tui::app::handlers::normal::chars::handle_char_input(c, vm, app_state)? {
                        return Ok(true);
                    }
                } else if matches!(key.code, KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right) {
                    if crate::tui::app::handlers::normal::navigation::handle_navigation_input(key.code, vm, app_state)? {
                        return Ok(true);
                    }
                } else {
                    if crate::tui::app::handlers::normal::actions::handle_action_input(key.code, vm, app_state)? {
                        return Ok(true);
                    }
                }
 Ok(false)
}
