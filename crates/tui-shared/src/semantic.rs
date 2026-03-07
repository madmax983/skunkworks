//! # Semantic Bridge
//!
//! A bridge between TUI applications and LLMs. Apps expose their semantic state
//! (not just pixels) so AI can understand, reason about, and interact with them.
//!
//! This module provides pure data structures and does not depend on `ratatui`,
//! `crossterm`, or any specific TUI backend.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 2D position/vector used for positioning entities in the TUI space.
///
/// Re-exported from `locus` crate.
pub use locus::Vec2;

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

/// Complete semantic snapshot of the TUI state.
///
/// This is the "screenshot" of your application's logic. Instead of pixels, it captures
/// the meaning of what's on screen. The LLM uses this to decide what to do next.
///
/// # Examples
///
/// ```
/// use tui_shared::semantic::{Snapshot, Entity, Action};
///
/// let snap = Snapshot::new("space-invaders")
///     .with_entity(Entity::new("player").at(10.0, 10.0))
///     .with_action(Action::new("fire").key("space"));
/// ```
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
    /// Creates a new empty snapshot for the given app.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::semantic::Snapshot;
    /// let snap = Snapshot::new("my-game");
    /// ```
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

    /// Sets the current frame number.
    pub fn with_frame(mut self, frame: u64) -> Self {
        self.frame = Some(frame);
        self
    }

    /// Sets the viewport dimensions.
    pub fn with_viewport(mut self, width: u16, height: u16) -> Self {
        self.viewport = Some((width, height));
        self
    }

    /// Adds a single entity to the snapshot.
    pub fn with_entity(mut self, entity: Entity) -> Self {
        self.entities.push(entity);
        self
    }

    /// Adds multiple entities to the snapshot.
    ///
    /// Useful when you have a collection of entities (like a `Vec<Player>`) that you want to
    /// dump into the snapshot at once.
    pub fn with_entities(mut self, entities: impl IntoIterator<Item = Entity>) -> Self {
        self.entities.extend(entities);
        self
    }

    /// Adds a region of interest to the snapshot.
    pub fn with_region(mut self, region: Region) -> Self {
        self.regions.push(region);
        self
    }

    /// Adds a top-level metric (score, time, etc.).
    pub fn with_metric(mut self, key: impl Into<String>, value: impl Into<PropValue>) -> Self {
        self.metrics.insert(key.into(), value.into());
        self
    }

    /// Sets the application state (e.g., "menu", "playing", "game_over").
    pub fn with_state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Adds an available action.
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
        assert!(matches!(
            entity.props.get("health"),
            Some(PropValue::Int(100))
        ));
        assert!(matches!(
            entity.props.get("is_boss"),
            Some(PropValue::Bool(true))
        ));
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
        #[allow(clippy::approx_constant)]
        let expected = 3.14;
        assert!(matches!(val, PropValue::Float(v) if (v - expected).abs() < f64::EPSILON));

        // Test Bool
        let json = "true";
        let val: PropValue = serde_json::from_str(json).unwrap();
        assert!(matches!(val, PropValue::Bool(true)));

        // Test Text
        let json = "\"hello\"";
        let val: PropValue = serde_json::from_str(json).unwrap();
        assert!(matches!(val, PropValue::Text(s) if s == "hello"));
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

    #[test]
    fn test_from_traits_for_prop_value() {
        let p_i32: PropValue = 42i32.into();
        assert!(matches!(p_i32, PropValue::Int(42)));

        let p_i64: PropValue = 42i64.into();
        assert!(matches!(p_i64, PropValue::Int(42)));

        let p_usize: PropValue = 42usize.into();
        assert!(matches!(p_usize, PropValue::Int(42)));

        let p_f32: PropValue = 42.42f32.into();
        assert!(matches!(p_f32, PropValue::Float(f) if (f - 42.42).abs() < 0.0001));

        let p_f64: PropValue = 42.42f64.into();
        assert!(matches!(p_f64, PropValue::Float(f) if (f - 42.42).abs() < 0.0001));

        let p_bool: PropValue = true.into();
        assert!(matches!(p_bool, PropValue::Bool(true)));

        let p_str: PropValue = "test".into();
        assert!(matches!(p_str, PropValue::Text(s) if s == "test"));

        let p_string: PropValue = String::from("test").into();
        assert!(matches!(p_string, PropValue::Text(s) if s == "test"));
    }

    #[test]
    fn test_snapshot_with_entities_and_pretty() {
        let entity1 = Entity::new("e1");
        let entity2 = Entity::new("e2");
        let snap = Snapshot::new("test").with_entities(vec![entity1, entity2]);
        assert_eq!(snap.entities.len(), 2);

        let pretty = snap.to_json_pretty();
        assert!(pretty.contains("test"));
        assert!(pretty.contains("e1"));
        assert!(pretty.contains("e2"));
    }
}
