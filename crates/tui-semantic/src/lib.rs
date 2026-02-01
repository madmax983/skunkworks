//! # tui-semantic
//!
//! A bridge between TUI applications and LLMs. Apps expose their semantic state
//! (not just pixels) so AI can understand, reason about, and interact with them.
//!
//! ## Example
//!
//! ```ignore
//! use tui_semantic::{SemanticState, Snapshot, Entity, Vec2};
//!
//! impl SemanticState for MyApp {
//!     fn snapshot(&self) -> Snapshot {
//!         Snapshot::new("my-app")
//!             .with_entity(Entity::new("player")
//!                 .at(self.player.x, self.player.y)
//!                 .with_prop("health", self.player.health))
//!             .with_metric("score", self.score)
//!     }
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 2D position/vector
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// A semantic entity in the TUI (particle, player, enemy, UI element, etc.)
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
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub props: HashMap<String, PropValue>,
}

impl Entity {
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

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn at(mut self, x: f64, y: f64) -> Self {
        self.position = Some(Vec2::new(x, y));
        self
    }

    pub fn moving(mut self, vx: f64, vy: f64) -> Self {
        self.velocity = Some(Vec2::new(vx, vy));
        self
    }

    pub fn display(mut self, c: impl Into<String>) -> Self {
        self.display = Some(c.into());
        self
    }

    pub fn with_prop(mut self, key: impl Into<String>, value: impl Into<PropValue>) -> Self {
        self.props.insert(key.into(), value.into());
        self
    }
}

/// Property values that can be attached to entities
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    Text(String),
}

impl From<i32> for PropValue {
    fn from(v: i32) -> Self { PropValue::Int(v as i64) }
}
impl From<i64> for PropValue {
    fn from(v: i64) -> Self { PropValue::Int(v) }
}
impl From<usize> for PropValue {
    fn from(v: usize) -> Self { PropValue::Int(v as i64) }
}
impl From<f64> for PropValue {
    fn from(v: f64) -> Self { PropValue::Float(v) }
}
impl From<f32> for PropValue {
    fn from(v: f32) -> Self { PropValue::Float(v as f64) }
}
impl From<bool> for PropValue {
    fn from(v: bool) -> Self { PropValue::Bool(v) }
}
impl From<&str> for PropValue {
    fn from(v: &str) -> Self { PropValue::Text(v.to_string()) }
}
impl From<String> for PropValue {
    fn from(v: String) -> Self { PropValue::Text(v) }
}

/// A rectangular region of interest
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

    pub fn describe(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

/// Complete semantic snapshot of the TUI state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// Application identifier
    pub app: String,
    /// Frame/tick number for tracking state over time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame: Option<u64>,
    /// Viewport dimensions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<(u16, u16)>,
    /// All semantic entities
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<Entity>,
    /// Named regions of interest
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub regions: Vec<Region>,
    /// Top-level metrics (score, time, counts)
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub metrics: HashMap<String, PropValue>,
    /// Current app state/mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Available actions the user/LLM can take
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<Action>,
}

impl Snapshot {
    pub fn new(app: impl Into<String>) -> Self {
        Self {
            app: app.into(),
            frame: None,
            viewport: None,
            entities: Vec::new(),
            regions: Vec::new(),
            metrics: HashMap::new(),
            state: None,
            actions: Vec::new(),
        }
    }

    pub fn with_frame(mut self, frame: u64) -> Self {
        self.frame = Some(frame);
        self
    }

    pub fn with_viewport(mut self, width: u16, height: u16) -> Self {
        self.viewport = Some((width, height));
        self
    }

    pub fn with_entity(mut self, entity: Entity) -> Self {
        self.entities.push(entity);
        self
    }

    pub fn with_entities(mut self, entities: impl IntoIterator<Item = Entity>) -> Self {
        self.entities.extend(entities);
        self
    }

    pub fn with_region(mut self, region: Region) -> Self {
        self.regions.push(region);
        self
    }

    pub fn with_metric(mut self, key: impl Into<String>, value: impl Into<PropValue>) -> Self {
        self.metrics.insert(key.into(), value.into());
        self
    }

    pub fn with_state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    pub fn with_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }

    /// Serialize to pretty JSON
    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}

/// An action the LLM can request
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
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            key: None,
        }
    }

    pub fn describe(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }
}

/// Trait for TUI apps to expose their semantic state
pub trait SemanticState {
    /// Generate a snapshot of the current semantic state
    fn snapshot(&self) -> Snapshot;
}

/// Commands that can be sent to the TUI app
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Command {
    /// Request a snapshot
    GetSnapshot,
    /// Send a key press
    SendKey { key: String },
    /// Invoke a named action
    InvokeAction { name: String },
    /// Quit the application
    Quit,
}

impl Command {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_serialization() {
        let snapshot = Snapshot::new("test-app")
            .with_frame(42)
            .with_viewport(80, 24)
            .with_entity(
                Entity::new("particle")
                    .with_id("p1")
                    .at(10.5, 20.3)
                    .moving(0.1, -0.2)
                    .display("*")
                    .with_prop("mass", 1.5),
            )
            .with_metric("score", 100)
            .with_state("running")
            .with_action(Action::new("quit").key("q").describe("Exit the app"));

        let json = snapshot.to_json();
        assert!(json.contains("test-app"));
        assert!(json.contains("particle"));
        assert!(json.contains("10.5"));
    }
}
