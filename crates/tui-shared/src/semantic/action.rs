use serde::{Deserialize, Serialize};

/// An action the LLM can request.
///
/// Actions are the verbs of your TUI. They define the affordances available to the user
/// (and thus the LLM) at the current moment.
///
/// # Examples
///
/// ```
/// use tui_shared::semantic::Action;
/// let jump = Action::new("jump")
///     .key("space")
///     .describe("Make the character jump");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// Action identifier (e.g., "quit", "reset", "move_left")
    pub name: String,
    /// Human-readable description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Key binding if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl Action {
    /// Creates a new action with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::semantic::Action;
    /// let quit = Action::new("quit");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            key: None,
        }
    }

    /// Adds a human-readable description to the action.
    pub fn describe(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Sets the key binding for this action.
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }
}
