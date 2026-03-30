//! # Action 🎬
//!
//! Provides the [`Action`] struct to define available user interactions in a semantic snapshot.
//!
//! Actions are the "verbs" of your TUI. By registering actions inside a
//! [`crate::semantic::Snapshot`], you tell the LLM (or a blind user interpreting the structure)
//! what affordances are currently available on the screen, like moving, selecting, or quitting.

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_new() {
        let action = Action::new("quit");
        assert_eq!(action.name, "quit");
        assert_eq!(action.description, None);
        assert_eq!(action.key, None);
    }

    #[test]
    fn test_action_describe() {
        let action = Action::new("attack").describe("Attack the nearest enemy");
        assert_eq!(action.name, "attack");
        assert_eq!(
            action.description,
            Some("Attack the nearest enemy".to_string())
        );
        assert_eq!(action.key, None);
    }

    #[test]
    fn test_action_key() {
        let action = Action::new("jump").key("Space");
        assert_eq!(action.name, "jump");
        assert_eq!(action.description, None);
        assert_eq!(action.key, Some("Space".to_string()));
    }

    #[test]
    fn test_action_chained() {
        let action = Action::new("fire").key("f").describe("Fire weapon");
        assert_eq!(action.name, "fire");
        assert_eq!(action.description, Some("Fire weapon".to_string()));
        assert_eq!(action.key, Some("f".to_string()));
    }
}
