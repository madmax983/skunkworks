//! # Snapshot 📸
//!
//! Provides the [`Snapshot`] struct to capture the entire semantic state of your TUI.
//!
//! A semantic snapshot represents the meaning of what's on screen rather than the pixels.
//! This allows Large Language Models to easily parse and interact with your application.

use crate::action::Action;
use crate::entity::Entity;
use crate::entity::PropValue;
use crate::region::Region;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
/// use tui_shared::{Snapshot, Entity, Action, Region};
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
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub metrics: BTreeMap<String, PropValue>,
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
    /// use tui_shared::Snapshot;
    /// let snap = Snapshot::new("my-game");
    /// ```
    pub fn new(app: impl Into<String>) -> Self {
        Self {
            app: app.into(),
            frame: None,
            viewport: None,
            // ⚡ Bolt: Pre-allocate vectors with typical capacities to reduce heap reallocations.
            entities: Vec::with_capacity(32),
            regions: Vec::with_capacity(8),
            // ⚡ Bolt: Use BTreeMap instead of HashMap for smaller structures (like properties and metrics)
            // to avoid the memory/hashing overhead of the default SipHasher, while gaining deterministic serialization.
            metrics: BTreeMap::new(),
            state: None,
            actions: Vec::with_capacity(8),
        }
    }

    /// Sets the current frame number.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::Snapshot;
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
    /// use tui_shared::Snapshot;
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
    /// use tui_shared::{Snapshot, Entity};
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
    /// use tui_shared::{Snapshot, Entity};
    /// let enemies = vec![Entity::new("enemy").at(1.0, 1.0), Entity::new("enemy").at(2.0, 2.0)];
    /// let snap = Snapshot::new("game").with_entities(enemies);
    /// ```
    pub fn with_entities(mut self, entities: impl IntoIterator<Item = Entity>) -> Self {
        let iter = entities.into_iter();
        let (lower, upper) = iter.size_hint();
        // 🔒 Warden: Cap the capacity allocation based on a safe upper bound limit to avoid capacity overflow
        let count = upper.unwrap_or(lower).min(100_000);
        self.entities.reserve(count);

        self.entities
            .extend(iter.take(100_000usize.saturating_sub(self.entities.len())));
        self
    }

    /// Adds a region of interest to the snapshot.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::{Snapshot, Region};
    /// let snap = Snapshot::new("app")
    ///     .with_region(Region::new("chat", 0, 20, 80, 4));
    /// ```
    pub fn with_region(mut self, region: Region) -> Self {
        self.regions.push(region);
        self
    }

    /// Adds a top-level metric (score, time, etc.).
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the metric (e.g., "score").
    /// * `value` - The value of the metric, conforming to [`PropValue`].
    ///
    /// ## Examples
    ///
    /// ```
    /// use tui_shared::Snapshot;
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
    /// use tui_shared::Snapshot;
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
    /// use tui_shared::{Snapshot, Action};
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
    /// use tui_shared::Snapshot;
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
    /// use tui_shared::Snapshot;
    /// let snap = Snapshot::new("game");
    /// let pretty_json = snap.to_json_pretty();
    /// ```
    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }

    fn fmt_metrics(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.metrics.is_empty() {
            return Ok(());
        }
        use comfy_table::{presets::UTF8_FULL, Cell, Color, Table};
        let mut metrics_table = Table::new();
        metrics_table.load_preset(UTF8_FULL).set_header(vec![
            Cell::new("Metric").fg(Color::Cyan),
            Cell::new("Value").fg(Color::Cyan),
        ]);
        for (k, v) in &self.metrics {
            let v_str = v.to_string();
            let mut v_cell = Cell::new(&v_str);

            v_cell = match v {
                PropValue::Bool(true) => v_cell.fg(Color::Green),
                PropValue::Bool(false) => v_cell.fg(Color::Yellow),
                PropValue::Text(_) => v_cell.fg(Color::Magenta),
                _ => v_cell.fg(Color::Blue),
            };

            metrics_table.add_row(vec![Cell::new(k).fg(Color::Yellow), v_cell]);
        }
        writeln!(f, "\n{}", metrics_table)
    }

    fn format_entity_props(props: &std::collections::BTreeMap<String, PropValue>) -> String {
        if props.is_empty() {
            return "".to_string();
        }
        use crossterm::style::Stylize;
        use std::fmt::Write;
        let mut props_str = String::with_capacity(props.len() * 16);
        let mut is_first = true;
        for (k, v) in props {
            if !is_first {
                props_str.push_str(", ");
            }
            let v_str = match v {
                PropValue::Bool(true) => "True".green().to_string(),
                PropValue::Bool(false) => "False".yellow().to_string(),
                _ => "".to_string(),
            };
            if !v_str.is_empty() {
                let _ = write!(&mut props_str, "{}: {}", k, v_str);
            } else {
                let _ = write!(&mut props_str, "{}: {}", k, v);
            }
            is_first = false;
        }
        props_str
    }

    fn fmt_entities(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.entities.is_empty() {
            return Ok(());
        }
        use comfy_table::{presets::UTF8_FULL, Cell, Color, Table};
        let mut entities_table = Table::new();
        entities_table.load_preset(UTF8_FULL).set_header(vec![
            Cell::new("Entity").fg(Color::Cyan),
            Cell::new("Kind").fg(Color::Cyan),
            Cell::new("Position").fg(Color::Cyan),
            Cell::new("Display").fg(Color::Cyan),
            Cell::new("Props").fg(Color::Cyan),
        ]);
        for (i, entity) in self.entities.iter().enumerate() {
            let pos = if let Some(p) = &entity.position {
                format!("{:.1}, {:.1}", p.x, p.y)
            } else {
                "None".to_string()
            };
            let fallback_id = i.to_string();
            let entity_id = entity.id.as_deref().unwrap_or(&fallback_id);
            let props_str = Self::format_entity_props(&entity.props);

            entities_table.add_row(vec![
                Cell::new(entity_id).fg(Color::Yellow),
                Cell::new(&entity.kind).fg(Color::Magenta),
                Cell::new(pos),
                Cell::new(entity.display.as_deref().unwrap_or("")),
                Cell::new(props_str),
            ]);
        }
        writeln!(f, "\n{}", entities_table)
    }
}

impl std::fmt::Display for Snapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, Color, Table};
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![
                Cell::new("Snapshot:").fg(Color::Cyan),
                Cell::new(&self.app).fg(Color::Yellow),
            ]);

        if let Some(frame) = self.frame {
            table.add_row(vec![Cell::new("Frame"), Cell::new(frame.to_string())]);
        }

        if let Some((w, h)) = self.viewport {
            table.add_row(vec![
                Cell::new("Viewport"),
                Cell::new(format!("{}x{}", w, h)),
            ]);
        }

        if let Some(state) = &self.state {
            table.add_row(vec![Cell::new("State"), Cell::new(state).fg(Color::Green)]);
        }

        writeln!(f, "{}", table)?;

        self.fmt_metrics(f)?;
        self.fmt_entities(f)?;

        Ok(())
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
