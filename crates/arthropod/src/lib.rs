//! # Arthropod
//!
//! `arthropod` provides a collection of interactive UI components designed to
//! seamlessly integrate with the `macroquad` framework.
//!
//! Currently, it features a highly customizable, immediate-mode [`Button`] widget
//! that tracks mouse interactions and renders itself to the screen.
//!
//! ## Examples
//!
//! ```no_run
//! use arthropod::Button;
//! use macroquad::prelude::*;
//!
//! #[macroquad::main("UI Example")]
//! async fn main() {
//!     let start_btn = Button::new("Start", 100.0, 100.0, 200.0, 50.0)
//!         .with_colors(GREEN, LIME, DARKGREEN);
//!
//!     loop {
//!         clear_background(BLACK);
//!
//!         if start_btn.draw() {
//!             println!("Start button was clicked!");
//!         }
//!
//!         next_frame().await;
//!     }
//! }
//! ```

/// Provides the interactive button widget for the `macroquad` framework.
pub(crate) mod button;
pub use button::Button;
