use serde::{Deserialize, Serialize};

/// A possible user action available in the current semantic state.
///
/// Actions are the "verbs" of your TUI. By registering actions inside a
/// [`crate::semantic::Snapshot`], you tell the LLM (or a blind user interpreting the structure)
/// what affordances are currently available on the screen, like moving, selecting, or quitting.
///
/// If an action has an associated `key`, it also tells the system *how* to invoke it.
///
/// # Examples
///
/// ```
/// use tui_shared::semantic::Action;
///
/// let jump = Action::new("jump")
///     .key("space")
///     .describe("Make the character jump");
///
/// let quit = Action::new("quit")
///     .key("q")
///     .describe("Exit the application without saving");
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
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::semantic::Action;
    /// let action = Action::new("attack").describe("Attack the nearest enemy");
    /// ```
    pub fn describe(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Sets the key binding for this action.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::semantic::Action;
    /// let action = Action::new("jump").key("Space");
    /// ```
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }
}
