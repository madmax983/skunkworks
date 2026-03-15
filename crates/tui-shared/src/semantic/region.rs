//! # Region 🗺️
//!
//! Provides the [`Region`] struct to define named bounds on the screen.
//!
//! Regions are the "where" in your TUI story. They provide layout context to an LLM reading
//! the [`crate::semantic::Snapshot`], acting like named bounds or panels.

use serde::{Deserialize, Serialize};

/// A named rectangular boundary on the screen indicating a specific functional area.
///
/// Regions are the "where" in your TUI story. They provide layout context to an LLM reading
/// the [`crate::semantic::Snapshot`], acting like named bounds or panels. For instance, knowing
/// there's an "inventory panel" helps the LLM understand *why* certain [`crate::semantic::Entity`]
/// instances are positioned where they are.
///
/// # Examples
///
/// ```
/// use tui_shared::semantic::Region;
///
/// let chat = Region::new("chat_box", 0, 20, 80, 5)
///     .describe("Area where messages appear");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub name: String,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Region {
    /// Creates a new region with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `name` - The identifier for this region.
    /// * `x` - The horizontal starting position.
    /// * `y` - The vertical starting position.
    /// * `width` - The width of the region.
    /// * `height` - The height of the region.
    ///
    /// ## Examples
    ///
    /// ```
    /// use tui_shared::semantic::Region;
    ///
    /// let region = Region::new("minimap", 0, 0, 20, 10);
    /// ```
    pub fn new(name: impl Into<String>, x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            name: name.into(),
            x,
            y,
            width,
            height,
            description: None,
        }
    }

    /// Adds a human-readable description to the region.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::semantic::Region;
    ///
    /// let region = Region::new("stats", 80, 0, 20, 24)
    ///     .describe("Shows player statistics");
    /// ```
    pub fn describe(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}
