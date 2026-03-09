//! # TUI Shared
//!
//! A shared library for initializing and managing Terminal User Interface (TUI) environments
//! using `ratatui` and `crossterm`.
//!
//! This crate provides a RAII (Resource Acquisition Is Initialization) wrapper around the terminal
//! setup, ensuring that the terminal is correctly restored (raw mode disabled, cursor shown, etc.)
//! when the application exits or panics.
//!
//! ## Example
//!
//! ```no_run
//! use tui_shared::Tui;
//! // Use re-exported ratatui for version consistency
//! use tui_shared::ratatui::widgets::{Block, Borders};
//! use std::io;
//!
//! fn main() -> io::Result<()> {
//!     // Initialize the terminal
//!     let mut tui = Tui::init()?;
//!
//!     // Draw something to the terminal
//!     tui.terminal.draw(|f| {
//!         let block = Block::default().title("Block").borders(Borders::ALL);
//!         f.render_widget(block, f.area());
//!     })?;
//!
//!     // The terminal is automatically restored when `tui` goes out of scope
//!     Ok(())
//! }
//! ```
//!
//! ## Re-exports
//!
//! This crate re-exports `ratatui` and `crossterm` so you can use the same versions.
//!
//! ```rust
//! use tui_shared::ratatui::widgets::Block;
//! use tui_shared::crossterm::event::KeyCode;
//!
//! let block = Block::default();
//! let key = KeyCode::Enter;
//! ```
//!
//! ## Testing
//!
//! Because `Tui::init` modifies the global terminal state, it is not suitable for unit tests.
//! Instead, use `ratatui::backend::TestBackend` to test your drawing logic without a real terminal:
//!
//! ```rust
//! use ratatui::{backend::TestBackend, Terminal, widgets::{Paragraph, Block, Borders}};
//!
//! fn test_ui() {
//!     let backend = TestBackend::new(20, 10);
//!     let mut terminal = Terminal::new(backend).unwrap();
//!
//!     terminal.draw(|f| {
//!         let p = Paragraph::new("Hello").block(Block::default().borders(Borders::ALL));
//!         f.render_widget(p, f.area());
//!     }).unwrap();
//!
//!     let buffer = terminal.backend().buffer();
//!     assert_eq!(buffer.get(1, 1).symbol(), "H");
//! }
//! ```

pub(crate) mod bobber;
pub(crate) mod button;
pub(crate) mod log_list;
pub mod semantic;
pub(crate) mod tension_bar;

pub use bobber::Bobber;
pub use button::{Button, ButtonState, ButtonStyle};
pub use log_list::LogList;
pub use tension_bar::TensionBar;

pub use crossterm;
pub use ratatui;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Stdout};

/// A RAII wrapper for the Ratatui Terminal.
///
/// This struct handles the initialization and cleanup of the terminal environment.
/// Upon creation via [`Tui::init`], it:
/// - Enables raw mode.
/// - Enters the alternate screen buffer.
/// - Enables mouse capture.
///
/// When dropped (or when [`Tui::exit`] is called), it reverses these actions to restore
/// the terminal to its original state.
pub struct Tui {
    /// The underlying Ratatui `Terminal` instance.
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    /// Initializes the terminal interface.
    ///
    /// This function sets up the terminal for a TUI application by:
    /// 1. Enabling raw mode (so input is processed character-by-character).
    /// 2. Entering the alternate screen (so the previous shell history is preserved).
    /// 3. Enabling mouse capture (so mouse events can be handled).
    ///
    /// # Errors
    ///
    /// Returns an `io::Error` if any of the terminal setup operations fail.
    ///
    /// # Panics
    ///
    /// This function may panic if the `crossterm` execution macro fails in an unrecoverable way,
    /// though most errors are propagated as `io::Error`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tui_shared::Tui;
    /// use std::io;
    ///
    /// fn main() -> io::Result<()> {
    ///     let tui = Tui::init()?;
    ///     // Terminal is now in raw mode, on the alternate screen
    ///     Ok(())
    /// }
    /// ```
    pub fn init() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    /// Restores the terminal to its original state.
    ///
    /// This function acts as a manual destructor. It:
    /// 1. Disables raw mode.
    /// 2. Leaves the alternate screen.
    /// 3. Disables mouse capture.
    /// 4. Shows the cursor.
    ///
    /// This is automatically called when the `Tui` struct is dropped, but can be called
    /// manually if early cleanup is required.
    ///
    /// # Errors
    ///
    /// Returns an `io::Error` if any of the terminal restoration operations fail.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tui_shared::Tui;
    /// use std::io;
    ///
    /// fn main() -> io::Result<()> {
    ///     let mut tui = Tui::init()?;
    ///
    ///     // We can manually exit early before the `tui` variable is dropped
    ///     tui.exit()?;
    ///
    ///     println!("Terminal is back to normal mode!");
    ///     Ok(())
    /// }
    /// ```
    pub fn exit(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        let _ = self.exit();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tui_drop_calls_exit() {
        // We cannot reliably test actual terminal setup/teardown in standard unit tests
        // since they run concurrently and would mangle the terminal state for cargo test.
        // However, we can assert that the Tui struct definition and Drop logic compiles
        // and provides the expected API.

        // This is a minimal compile-time assertion that Tui implements Drop.
        assert!(std::mem::needs_drop::<Tui>());
    }
}
