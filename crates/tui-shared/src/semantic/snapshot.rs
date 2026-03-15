use crate::semantic::action::Action;
use crate::semantic::entity::Entity;
use crate::semantic::entity::PropValue;
use crate::semantic::region::Region;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete semantic snapshot of the TUI state.
///
/// This is the "screenshot" of your application's logic. Instead of pixels, it captures
/// the meaning of what's on screen. The LLM uses this to decide what to do next.
///
/// A [`Snapshot`] is composed of:
/// *   **[`Entity`]** instances representing actors/objects.
/// *   **[`Region`]** instances defining layout areas.
/// *   **[`Action`]** instances enumerating possible user inputs.
/// *   High-level properties (app name, frame tick, metrics, viewport).
///
/// # Examples
///
/// ```
/// use tui_shared::semantic::{Snapshot, Entity, Action, Region};
///
/// let snap = Snapshot::new("space-invaders")
///     .with_frame(100)
///     .with_viewport(80, 24)
///     .with_region(Region::new("play_area", 0, 0, 80, 20).describe("Where aliens attack"))
///     .with_entity(Entity::new("player").at(40.0, 18.0).with_prop("lives", 3))
///     .with_entity(Entity::new("alien").at(10.0, 5.0).moving(1.0, 0.0))
///     .with_metric("score", 1500)
///     .with_action(Action::new("fire").key("space").describe("Shoot laser"))
///     .with_action(Action::new("move_left").key("left"))
///     .with_action(Action::new("move_right").key("right"));
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
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::semantic::Snapshot;
    /// let snap = Snapshot::new("game").with_frame(42);
    /// ```
    pub fn with_frame(mut self, frame: u64) -> Self {
        self.frame = Some(frame);
        self
    }

    /// Sets the viewport dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::semantic::Snapshot;
    /// let snap = Snapshot::new("app").with_viewport(80, 24);
    /// ```
    pub fn with_viewport(mut self, width: u16, height: u16) -> Self {
        self.viewport = Some((width, height));
        self
    }

    /// Adds a single entity to the snapshot.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::semantic::{Snapshot, Entity};
    /// let snap = Snapshot::new("game")
    ///     .with_entity(Entity::new("player").at(10.0, 5.0));
    /// ```
    pub fn with_entity(mut self, entity: Entity) -> Self {
        self.entities.push(entity);
        self
    }

    /// Adds multiple entities to the snapshot.
    ///
    /// Useful when you have a collection of entities (like a `Vec<Player>`) that you want to
    /// dump into the snapshot at once.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::semantic::{Snapshot, Entity};
    /// let enemies = vec![Entity::new("enemy").at(1.0, 1.0), Entity::new("enemy").at(2.0, 2.0)];
    /// let snap = Snapshot::new("game").with_entities(enemies);
    /// ```
    pub fn with_entities(mut self, entities: impl IntoIterator<Item = Entity>) -> Self {
        self.entities.extend(entities);
        self
    }

    /// Adds a region of interest to the snapshot.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::semantic::{Snapshot, Region};
    /// let snap = Snapshot::new("app")
    ///     .with_region(Region::new("chat", 0, 20, 80, 4));
    /// ```
    pub fn with_region(mut self, region: Region) -> Self {
        self.regions.push(region);
        self
    }

    /// Adds a top-level metric (score, time, etc.).
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::semantic::Snapshot;
    /// let snap = Snapshot::new("game").with_metric("score", 100);
    /// ```
    pub fn with_metric(mut self, key: impl Into<String>, value: impl Into<PropValue>) -> Self {
        self.metrics.insert(key.into(), value.into());
        self
    }

    /// Sets the application state (e.g., "menu", "playing", "game_over").
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::semantic::Snapshot;
    /// let snap = Snapshot::new("game").with_state("game_over");
    /// ```
    pub fn with_state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Adds an available action.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::semantic::{Snapshot, Action};
    /// let snap = Snapshot::new("game")
    ///     .with_action(Action::new("jump").key("Space"));
    /// ```
    pub fn with_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }

    /// Serializes the snapshot to a compact JSON string.
    ///
    /// This is the primary format used to send the snapshot to an LLM over a network connection.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::semantic::Snapshot;
    /// let snap = Snapshot::new("game");
    /// let json = snap.to_json();
    /// ```
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }

    /// Serializes the snapshot to a pretty-printed JSON string.
    ///
    /// Useful for debugging and viewing the semantic state locally.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::semantic::Snapshot;
    /// let snap = Snapshot::new("game");
    /// let pretty_json = snap.to_json_pretty();
    /// ```
    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}
