//! # Semantic Bridge
//!
//! A bridge between TUI applications and LLMs. Apps expose their semantic state
//! (not just pixels) so AI can understand, reason about, and interact with them.
//!
//! This module provides pure data structures and does not depend on `ratatui`,
//! `crossterm`, or any specific TUI backend.

pub mod action;
pub mod entity;
pub mod region;
pub mod snapshot;

pub use action::Action;
pub use entity::{Entity, PropValue};
pub use locus::Vec2;
pub use region::Region;
pub use snapshot::Snapshot;

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
