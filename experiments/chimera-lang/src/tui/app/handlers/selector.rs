use crate::ast::Gene;
use crate::vm::ChimeraVM;
use crate::{ChimeraParser, Rule};
use super::super::state::{AppState, InputMode, ViewMode};
use super::super::get_all_views;
use super::super::parse_grid_value;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use pest::Parser;

pub(crate) fn handle_view_selector(
    key: KeyEvent,
    app_state: &mut AppState,
) -> bool {
    let views = get_all_views();
    let mut list_state = app_state.view_selector_state.borrow_mut();
    let selected = list_state.selected().unwrap_or(0);

    match key.code {
        KeyCode::Esc => app_state.show_view_selector = false,
        KeyCode::Up => {
            if selected > 0 {
                list_state.select(Some(selected - 1));
            } else {
                list_state.select(Some(views.len().saturating_sub(1)));
            }
        }
        KeyCode::Down => {
            if selected + 1 < views.len() {
                list_state.select(Some(selected + 1));
            } else {
                list_state.select(Some(0));
            }
        }
        KeyCode::Enter => {
            if selected < views.len() {
                app_state.view_mode = views[selected].0;
            }
            app_state.show_view_selector = false;
        }
        _ => {}
    }
    true
}
