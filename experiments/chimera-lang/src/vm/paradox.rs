use crate::vm::ChimeraVM;
use crate::vm::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
/// Enum for `Trigger`.
pub enum Trigger {
    /// Always
    Always,
    /// Signal
    Signal(String), // Match active signal in Prologue
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
/// Enum for `Action`.
pub enum Action {
    /// Log
    Log(String),
    /// Set
    Set(i64, i64, Value), // Relative x, y, value (to context_loc)
    /// Glitch
    Glitch(f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
/// Represents a `Rule`.
pub struct Rule {
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    /// The `name` field.
    pub name: String,
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    /// The `trigger` field.
    pub trigger: Trigger,
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    /// The `actions` field.
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
/// Represents a `Paradox`.
pub struct Paradox {
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    /// The `rules` field.
    pub rules: Vec<Rule>,
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    /// The `active` field.
    pub active: bool,
}

impl Default for Paradox {
    fn default() -> Self {
        Self {
            rules: Vec::new(),
            active: true,
        }
    }
}

impl Paradox {
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    /// Performs the `tick` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tick
    /// ```
    pub fn tick(&mut self, vm: &mut ChimeraVM) {
        if !self.active {
            return;
        }

        // Clone rules to avoid borrowing self while mutating vm
        let rules = self.rules.clone();

        for rule in rules {
            let triggered = match &rule.trigger {
                Trigger::Always => true,
                Trigger::Signal(s) => {
                    let mut found = false;
                    #[cfg(feature = "nova")]
                    {
                        // Check Prologue signals
                        for row in &vm.prologue_state.signal_grid {
                            for val in row.iter().flatten() {
                                if let Value::Str(sig) = val {
                                    if sig == s {
                                        found = true;
                                        break;
                                    }
                                }
                            }
                            if found {
                                break;
                            }
                        }
                    }
                    found
                }
            };

            if triggered {
                for action in &rule.actions {
                    match action {
                        Action::Log(msg) => {
                            vm.output.push(format!("PARADOX: {}", msg));
                        }
                        Action::Set(dx, dy, val) => {
                            let (cy, cx) = vm.context_loc;
                            // normalize_coords handles topology
                            if let Some((ny, nx)) = {
                                #[cfg(feature = "nova")]
                                let c = {
                                    #[cfg(feature = "nova")]
                                    let c = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx);
                                    #[cfg(not(feature = "nova"))]
                                    let c = (
                                        (cy as i64 + dy).rem_euclid(crate::vm::GRID_SIZE as i64)
                                            as usize,
                                        (cx as i64 + dx).rem_euclid(crate::vm::GRID_SIZE as i64)
                                            as usize,
                                    );
                                    c
                                };
                                #[cfg(not(feature = "nova"))]
                                let c = (
                                    (cy as i64 + dy).rem_euclid(crate::vm::GRID_SIZE as i64)
                                        as usize,
                                    (cx as i64 + dx).rem_euclid(crate::vm::GRID_SIZE as i64)
                                        as usize,
                                );
                                c
                            } {
                                vm.grid[ny][nx] = val.clone();
                            }
                        }
                        Action::Glitch(amount) => {
                            vm.glitch_level += amount;
                        }
                    }
                }
            }
        }
    }

    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    /// Performs the `parse_rule` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of parse_rule
    /// ```
    pub fn parse_rule(&mut self, input: &str) -> Result<(), String> {
        // Syntax: "rule NAME triggers TRIGGER do ACTION"
        // Example: "rule Test triggers always do log Hello"
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 5 {
            return Err(
                "Invalid rule format. Usage: rule NAME triggers TRIGGER do ACTION".to_string(),
            );
        }

        if parts[0] != "rule" {
            return Err("Expected 'rule'".to_string());
        }

        let name = parts[1].to_string();
        let trigger_idx = parts
            .iter()
            .position(|&x| x == "triggers")
            .ok_or("Missing 'triggers'")?;
        let do_idx = parts
            .iter()
            .position(|&x| x == "do")
            .ok_or("Missing 'do'")?;

        if trigger_idx > do_idx {
            return Err("triggers must come before do".to_string());
        }

        let trigger_str = parts.get(trigger_idx + 1).ok_or("Missing trigger type")?;
        let trigger = if *trigger_str == "always" {
            Trigger::Always
        } else {
            Trigger::Signal(trigger_str.to_string())
        };

        if do_idx + 1 >= parts.len() {
            return Err("Missing action type".to_string());
        }

        let action_type = parts
            .get(do_idx + 1)
            .copied()
            .ok_or("Missing action type")?;
        let action = match action_type {
            "log" => {
                let msg = if parts.len() > do_idx + 2 {
                    parts[do_idx + 2..].join(" ")
                } else {
                    String::new()
                };
                Action::Log(msg)
            }
            "glitch" => {
                let amount = parts
                    .get(do_idx + 2)
                    .ok_or("Missing glitch amount")?
                    .parse::<f32>()
                    .map_err(|_| "Invalid glitch amount")?;
                Action::Glitch(amount)
            }
            "set" => {
                if parts.len() < do_idx + 5 {
                    return Err("Set requires: set dx dy val".to_string());
                }
                let dx = parts[do_idx + 2].parse::<i64>().map_err(|_| "Invalid dx")?;
                let dy = parts[do_idx + 3].parse::<i64>().map_err(|_| "Invalid dy")?;
                let val_str = parts[do_idx + 4];
                let val = if let Ok(n) = val_str.parse::<i64>() {
                    Value::Int(n)
                } else {
                    Value::Str(val_str.to_string())
                };
                Action::Set(dx, dy, val)
            }
            _ => return Err(format!("Unknown action: {}", action_type)),
        };

        // Replace existing rule with same name or append
        if let Some(idx) = self.rules.iter().position(|r| r.name == name) {
            self.rules[idx] = Rule {
                name,
                trigger,
                actions: vec![action],
            };
        } else {
            self.rules.push(Rule {
                name,
                trigger,
                actions: vec![action],
            });
        }

        Ok(())
    }
}
