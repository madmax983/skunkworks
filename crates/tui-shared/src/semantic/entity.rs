pub use locus::Vec2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A semantic entity in the TUI (particle, player, enemy, UI element, etc.).
///
/// Entities are the nouns of your TUI story. They represent anything that has a presence
/// in the interface, whether it's a game character, a button, or a data point.
///
/// # Examples
///
/// ```
/// use tui_shared::semantic::Entity;
///
/// let player = Entity::new("hero")
///     .with_id("p1")
///     .at(40.0, 12.0)
///     .with_prop("hp", 100);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// Unique identifier for this entity type
    pub kind: String,
    /// Optional instance ID for tracking specific entities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Position in the TUI coordinate space
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Vec2>,
    /// Velocity if the entity moves
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity: Option<Vec2>,
    /// The character(s) representing this entity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    /// Arbitrary properties (health, mass, state, etc.)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub props: HashMap<String, PropValue>,
}

impl Entity {
    /// Creates a new entity with the given kind.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::semantic::Entity;
    /// let player = Entity::new("player");
    /// ```
    pub fn new(kind: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            id: None,
            position: None,
            velocity: None,
            display: None,
            props: HashMap::new(),
        }
    }

    /// Assigns a unique ID to the entity.
    ///
    /// The ID is crucial for object permanence. Without it, the LLM might see a "particle" in
    /// frame 1 and a "particle" in frame 2 but not know they are the same object.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the entity's position.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::semantic::Entity;
    /// let e = Entity::new("ball").at(5.0, 5.0);
    /// ```
    pub fn at(mut self, x: f64, y: f64) -> Self {
        self.position = Some(Vec2::new(x, y));
        self
    }

    /// Sets the entity's velocity.
    ///
    /// This helps the LLM predict future states.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::semantic::Entity;
    /// let e = Entity::new("bullet")
    ///     .at(10.0, 10.0)
    ///     .moving(1.0, 0.0);
    /// ```
    pub fn moving(mut self, vx: f64, vy: f64) -> Self {
        self.velocity = Some(Vec2::new(vx, vy));
        self
    }

    /// Sets the character(s) used to display the entity.
    pub fn display(mut self, c: impl Into<String>) -> Self {
        self.display = Some(c.into());
        self
    }

    /// Adds a property to the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::semantic::Entity;
    /// let enemy = Entity::new("orc")
    ///     .with_prop("health", 100)
    ///     .with_prop("elite", true);
    /// ```
    pub fn with_prop(mut self, key: impl Into<String>, value: impl Into<PropValue>) -> Self {
        self.props.insert(key.into(), value.into());
        self
    }
}

/// Property values that can be attached to entities.
///
/// This enum allows attaching arbitrary data to entities, which is crucial for
/// giving the LLM context about the entity's state (e.g., health, ammo, selected status).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    Text(String),
}

impl From<i32> for PropValue {
    fn from(v: i32) -> Self {
        PropValue::Int(v as i64)
    }
}
impl From<i64> for PropValue {
    fn from(v: i64) -> Self {
        PropValue::Int(v)
    }
}
impl From<usize> for PropValue {
    fn from(v: usize) -> Self {
        PropValue::Int(v as i64)
    }
}
impl From<f64> for PropValue {
    fn from(v: f64) -> Self {
        PropValue::Float(v)
    }
}
impl From<f32> for PropValue {
    fn from(v: f32) -> Self {
        PropValue::Float(v as f64)
    }
}
impl From<bool> for PropValue {
    fn from(v: bool) -> Self {
        PropValue::Bool(v)
    }
}
impl From<&str> for PropValue {
    fn from(v: &str) -> Self {
        PropValue::Text(v.to_string())
    }
}
impl From<String> for PropValue {
    fn from(v: String) -> Self {
        PropValue::Text(v)
    }
}
