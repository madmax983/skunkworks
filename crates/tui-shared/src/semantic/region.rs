use serde::{Deserialize, Serialize};

/// A rectangular region of interest in the UI.
///
/// Regions help the LLM understand the layout of the screen by defining semantic zones
/// (e.g., "inventory panel", "chat window", "map").
///
/// # Examples
///
/// ```
/// use tui_shared::semantic::Region;
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
    pub fn describe(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}
