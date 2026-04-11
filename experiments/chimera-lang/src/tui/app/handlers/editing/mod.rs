//! Handlers for the "Editing" Application State.
//!
//! This module processes keyboard inputs when the application state is configured for active text
//! input (such as typing command strings or inserting multi-character structures). It handles the
//! aggregation of characters, confirming submissions, or cancelling the action.
//!
//! * [`actions`] - Handles special keys for backspace, deletions, and escape.
//! * [`chars`] - Handles generic character insertion into the input buffer.
//! * [`enter`] - Handles the submission of the input buffer.

pub(crate) mod actions;
pub(crate) mod chars;
pub(crate) mod enter;

use crate::tui::state::AppState;
use crate::vm::ChimeraVM;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

pub(crate) fn handle_editing_input(
    key: KeyEvent,
    vm: &mut ChimeraVM,
    app_state: &mut AppState,
) -> Result<bool> {
    match key.code {
        KeyCode::Enter => enter::handle_enter_key(vm, app_state),
        KeyCode::Char(c) => chars::handle_char(c, vm, app_state),
        code @ (KeyCode::Tab | KeyCode::Esc | KeyCode::Backspace) => {
            actions::handle_action(code, vm, app_state)
        }
        _ => Ok(false),
    }
}
