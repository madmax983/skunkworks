pub(crate) mod enter;
pub(crate) mod tab;
pub(crate) mod esc;
pub(crate) mod char;
pub(crate) mod backspace;
use crate::ast::Gene;
use crate::vm::ChimeraVM;
use crate::{ChimeraParser, Rule};
use super::super::super::state::{AppState, InputMode, ViewMode};
use super::super::super::get_all_views;
use super::super::super::parse_grid_value;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use pest::Parser;

pub(crate) fn handle_editing_input(
    key: KeyEvent,
    vm: &mut ChimeraVM,
    app_state: &mut AppState,
) -> Result<bool> {
                match key.code {
                    KeyCode::Enter => return enter::handle_enter_input(vm, app_state),
                    KeyCode::Tab => return tab::handle_tab_input(app_state),
                    KeyCode::Esc => return esc::handle_esc_input(app_state),
                    KeyCode::Char(c) => return char::handle_char_input(c, app_state),
                    KeyCode::Backspace => return backspace::handle_backspace_input(app_state),
                    _ => return Ok(false),
                }
                Ok(true)
}
