//! # tui-semantic
//!
//! A bridge between TUI applications and LLMs. Apps expose their semantic state
//! (not just pixels) so AI can understand, reason about, and interact with them.
//!
//! This crate is **framework-agnostic**: it provides pure data structures and does
//! not depend on `ratatui`, `crossterm`, or any specific TUI backend.
//!
//! ## Example
//!
//! ```
//! use tui_semantic::{SemanticState, Snapshot, Entity};
//!
//! struct Player { x: f64, y: f64, health: i64 }
//! struct MyApp { player: Player, score: i64 }
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
//!
//! let app = MyApp {
//!     player: Player { x: 10.0, y: 20.0, health: 100 },
//!     score: 500,
//! };
//!
//! let json = app.snapshot().to_json();
//! assert!(json.contains("my-app"));
//! assert!(json.contains("health"));
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
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<Entity>,
    /// Named regions of interest
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regions: Vec<Region>,
    /// Top-level metrics (score, time, counts)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metrics: HashMap<String, PropValue>,
    /// Current app state/mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Available actions the user/LLM can take
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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

    #[test]
    fn test_snapshot_roundtrip() {
        let original = Snapshot::new("roundtrip-app")
            .with_frame(100)
            .with_viewport(120, 40)
            .with_entity(
                Entity::new("enemy")
                    .with_id("e1")
                    .at(50.0, 10.0)
                    .with_prop("health", 100)
                    .with_prop("is_boss", true),
            )
            .with_region(Region::new("main_view", 0, 0, 80, 24).describe("The main game view"))
            .with_metric("fps", 60.0);

        let json = original.to_json();
        let recovered: Snapshot =
            serde_json::from_str(&json).expect("Failed to deserialize snapshot");

        assert_eq!(original.app, recovered.app);
        assert_eq!(original.frame, recovered.frame);
        assert_eq!(original.viewport, recovered.viewport);
        assert_eq!(original.entities.len(), recovered.entities.len());
        assert_eq!(original.regions.len(), recovered.regions.len());
        assert_eq!(original.metrics.len(), recovered.metrics.len());

        let entity = &recovered.entities[0];
        assert_eq!(entity.kind, "enemy");
        assert_eq!(entity.id.as_deref(), Some("e1"));

        // Check props
        match entity.props.get("health") {
            Some(PropValue::Int(v)) => assert_eq!(*v, 100),
            _ => panic!("Expected health to be Int(100)"),
        }
        match entity.props.get("is_boss") {
            Some(PropValue::Bool(v)) => assert!(v),
            _ => panic!("Expected is_boss to be Bool(true)"),
        }
    }

    #[test]
    fn test_command_deserialization() {
        // Test GetSnapshot
        let json = r#"{"type": "GetSnapshot"}"#;
        let cmd = Command::from_json(json).expect("Failed to parse GetSnapshot");
        assert!(matches!(cmd, Command::GetSnapshot));

        // Test SendKey
        let json = r#"{"type": "SendKey", "key": "Enter"}"#;
        let cmd = Command::from_json(json).expect("Failed to parse SendKey");
        if let Command::SendKey { key } = cmd {
            assert_eq!(key, "Enter");
        } else {
            panic!("Expected SendKey");
        }

        // Test InvokeAction
        let json = r#"{"type": "InvokeAction", "name": "fire"}"#;
        let cmd = Command::from_json(json).expect("Failed to parse InvokeAction");
        if let Command::InvokeAction { name } = cmd {
            assert_eq!(name, "fire");
        } else {
            panic!("Expected InvokeAction");
        }

        // Test Quit
        let json = r#"{"type": "Quit"}"#;
        let cmd = Command::from_json(json).expect("Failed to parse Quit");
        assert!(matches!(cmd, Command::Quit));
    }

    #[test]
    fn test_prop_value_types() {
        // Test Int
        let json = "42";
        let val: PropValue = serde_json::from_str(json).unwrap();
        assert!(matches!(val, PropValue::Int(42)));

        // Test Float
        let json = "3.14";
        let val: PropValue = serde_json::from_str(json).unwrap();
        assert!(matches!(val, PropValue::Float(v) if (v - 3.14).abs() < f64::EPSILON));

        // Test Bool
        let json = "true";
        let val: PropValue = serde_json::from_str(json).unwrap();
        assert!(matches!(val, PropValue::Bool(true)));

        // Test Text
        let json = "\"hello\"";
        let val: PropValue = serde_json::from_str(json).unwrap();
        if let PropValue::Text(s) = val {
            assert_eq!(s, "hello");
        } else {
            panic!("Expected Text");
        }
    }

    #[test]
    fn test_builder_methods() {
        let action = Action::new("jump").key("Space").describe("Jump up");
        assert_eq!(action.name, "jump");
        assert_eq!(action.key.as_deref(), Some("Space"));
        assert_eq!(action.description.as_deref(), Some("Jump up"));

        let region = Region::new("inventory", 0, 25, 20, 10).describe("Player inventory");
        assert_eq!(region.name, "inventory");
        assert_eq!(region.x, 0);
        assert_eq!(region.y, 25);
        assert_eq!(region.width, 20);
        assert_eq!(region.height, 10);
        assert_eq!(region.description.as_deref(), Some("Player inventory"));

        let entity = Entity::new("mob").moving(1.0, 2.0).display("@");
        assert!(entity.position.is_none());
        assert_eq!(entity.velocity.unwrap().x, 1.0);
        assert_eq!(entity.velocity.unwrap().y, 2.0);
        assert_eq!(entity.display.as_deref(), Some("@"));
    }
}
