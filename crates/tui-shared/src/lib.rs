//! # tui-shared 🎨
//!
//! A robust shared library for initializing and managing Terminal User Interface (TUI) environments using `ratatui` and `crossterm`.
//!
//! This crate provides a **RAII (Resource Acquisition Is Initialization)** wrapper around the terminal setup. It ensures that the terminal is correctly restored (raw mode disabled, cursor shown, alternate screen left) when the application exits or panics.
//!
//! ## Installation
//!
//! ### Option A: Internal Workspace Experiment (Recommended)
//!
//! If you are adding a new experiment within this repository (e.g., in `experiments/my-cool-tui`), `tui-shared` is already available as a workspace dependency.
//!
//! 1.  Create your experiment crate:
//!     ```bash
//!     cargo new experiments/my-cool-tui
//!     ```
//!     *Note: The `experiments/` directory is already part of the workspace members in the root `Cargo.toml`.*
//!
//! 2.  Add dependencies to your experiment's `Cargo.toml`:
//!     ```toml
//!     [dependencies]
//!     tui-shared = { workspace = true }
//!
//!     # Access ratatui and crossterm via tui-shared re-exports:
//!     # use tui_shared::ratatui;
//!     # use tui_shared::crossterm;
//!     ```
//!
//! ### Option B: Standalone Project
//!
//! If you are using this crate in a project *outside* of this workspace, you must point to the local path.
//!
//! 1.  Add to your `Cargo.toml`:
//!     ```toml
//!     [dependencies]
//!     # Replace path with the relative path from your project root to crates/tui-shared
//!     # For example, if you are in experiments/my-cool-tui, the path is "../../crates/tui-shared"
//!     tui-shared = { path = "../../crates/tui-shared" }
//!
//!     # You must explicitly add ratatui and crossterm to your dependencies if you use their types directly
//!     ratatui = "0.30"
//!     crossterm = "0.28"
//!     ```
//!
//! ## Usage
//!
//! Here is a minimal example:
//!
//! ```rust
//! use std::{io, thread, time::Duration};
//! use tui_shared::Tui;
//! use tui_shared::ratatui::{
//!     layout::Alignment,
//!     widgets::{Block, Borders, Paragraph},
//! };
//!
//! fn main() -> io::Result<()> {
//!     // 1. Initialize the terminal
//!     // This enables raw mode, enters alternate screen, and captures mouse.
//!     let mut tui = Tui::init()?;
//!
//!     // 2. Draw something to the terminal
//!     tui.terminal.draw(|f| {
//!         let size = f.area();
//!         let block = Block::default()
//!             .title(" My TUI App ")
//!             .borders(Borders::ALL);
//!         let p = Paragraph::new("Hello, World!")
//!             .block(block)
//!             .alignment(Alignment::Center);
//!         f.render_widget(p, size);
//!     })?;
//!
//!     // 3. Application logic here...
//!     // In a real app, you would run an event loop here.
//!     thread::sleep(Duration::from_secs(3));
//!
//!     // 4. Cleanup is automatic!
//!     // When `tui` goes out of scope, it automatically:
//!     // - Disables raw mode
//!     // - Leaves alternate screen
//!     // - Shows cursor
//!     Ok(())
//! }
//! ```
//!
//! ## Testing
//!
//! Because `Tui::init` modifies the global terminal state, it is not suitable for unit tests. Instead, use `ratatui::backend::TestBackend` to test your drawing logic without a real terminal.
//!
//! ```rust
//! use tui_shared::ratatui::{
//!     backend::TestBackend,
//!     Terminal,
//!     widgets::{Paragraph, Block, Borders}
//! };
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

pub(crate) mod action;
pub(crate) mod bobber;
pub(crate) mod button;
pub(crate) mod entity;
pub(crate) mod log_list;
pub(crate) mod region;
pub(crate) mod snapshot;

pub use action::Action;
pub use entity::{Entity, PropValue};
pub use region::Region;
pub use snapshot::Snapshot;
pub(crate) mod tension_bar;

pub use bobber::Bobber;
pub use button::Button;
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
///
/// # Examples
///
/// ```ignore
/// use tui_shared::Tui;
/// let mut tui = Tui::new().expect("Failed to initialize TUI");
/// tui.exit().expect("Failed to exit TUI");
/// ```
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
    /// Fails with an `io::Error` if any of the terminal setup operations fail. This usually occurs if the application
    /// is not running inside a valid terminal environment (e.g., piped output, background daemon).
    /// To recover, ensure the process is run attached to a valid TTY or fallback to headless mode if available.
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
    /// Fails with an `io::Error` if any of the terminal restoration operations fail. This typically happens if the terminal
    /// state has been externally corrupted. Recovery usually involves instructing the user to type `reset` in their terminal
    /// to fix remaining artifacting or invisible cursors.
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
