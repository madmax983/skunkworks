pub(crate) mod actions;
pub(crate) mod chars;
pub(crate) mod enter;

use crate::vm::ChimeraVM;
use super::super::state::AppState;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

pub(crate) fn handle_editing_input(
    key: KeyEvent,
    vm: &mut ChimeraVM,
    app_state: &mut AppState,
) -> Result<bool> {
    match key.code {
        KeyCode::Enter => enter::handle_enter(vm, app_state),
        KeyCode::Char(c) => chars::handle_char(c, vm, app_state),
        code @ (KeyCode::Tab | KeyCode::Esc | KeyCode::Backspace) => {
            actions::handle_action(code, vm, app_state)
        }
        _ => Ok(false),
    }
}
