//! Input Handlers for the TUI Application.
//!
//! This module acts as the router for all keyboard inputs coming from the user. It delegates
//! the keystrokes to specific handlers depending on the current application state (e.g., whether
//! the user is actively typing a command, or just navigating the grid).
//!
//! * [`editing`] - Handles input when the application is in an interactive text-entry mode.
//! * [`selector`] - Handles input specifically for selecting visual rendering modes or panels.
//! * [`normal`] - Handles input during the default, non-editing state (e.g., movement, triggers).

pub(crate) mod editing;
pub(crate) mod normal;
pub(crate) mod selector;

pub(crate) use editing::*;
pub(crate) use normal::*;
pub(crate) use selector::*;
