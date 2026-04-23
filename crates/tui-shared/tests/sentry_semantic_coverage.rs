use tui_shared::semantic::{Action, Entity, Region, Snapshot, PropValue};

#[test]
fn test_semantic_coverage() {
    let action = Action::new("test_action").describe("A test action").key("Space");
    assert_eq!(action.name, "test_action");
    assert_eq!(action.description.unwrap(), "A test action");
    assert_eq!(action.key.unwrap(), "Space");

    let entity = Entity::new("test_entity")
        .with_id("test_id")
        .at(1.0, 2.0)
        .moving(3.0, 4.0)
        .display("X")
        .with_prop("test_prop", "test_value");

    assert_eq!(entity.kind, "test_entity");
    assert_eq!(entity.id.unwrap(), "test_id");
    assert_eq!(entity.position.unwrap().x, 1.0);
    assert_eq!(entity.position.unwrap().y, 2.0);
    assert_eq!(entity.velocity.unwrap().x, 3.0);
    assert_eq!(entity.velocity.unwrap().y, 4.0);
    assert_eq!(entity.display.unwrap(), "X");
    assert!(entity.props.contains_key("test_prop"));

    let region = Region::new("test_region", 1, 2, 3, 4).describe("A test region");
    assert_eq!(region.name, "test_region");
    assert_eq!(region.x, 1);
    assert_eq!(region.y, 2);
    assert_eq!(region.width, 3);
    assert_eq!(region.height, 4);
    assert_eq!(region.description.unwrap(), "A test region");

    let snapshot = Snapshot::new("test_app")
        .with_frame(1)
        .with_viewport(2, 3)
        .with_entity(Entity::new("entity1"))
        .with_entities(vec![Entity::new("entity2")])
        .with_region(Region::new("region1", 0, 0, 10, 10))
        .with_metric("metric1", 100)
        .with_state("playing")
        .with_action(Action::new("action1"));

    assert_eq!(snapshot.app, "test_app");
    assert_eq!(snapshot.frame.unwrap(), 1);
    assert_eq!(snapshot.viewport.unwrap(), (2, 3));
    assert_eq!(snapshot.entities.len(), 2);
    assert_eq!(snapshot.regions.len(), 1);
    assert!(snapshot.metrics.contains_key("metric1"));
    assert_eq!(snapshot.state.unwrap(), "playing");
    assert_eq!(snapshot.actions.len(), 1);
}

#[test]
fn test_semantic_propvalue_from() {
    let p1: PropValue = 42i32.into();
    assert!(matches!(p1, PropValue::Int(42)));

    let p2: PropValue = 42i64.into();
    assert!(matches!(p2, PropValue::Int(42)));

    let p3: PropValue = 42usize.into();
    assert!(matches!(p3, PropValue::Int(42)));

    let p4: PropValue = 42.0f32.into();
    assert!(matches!(p4, PropValue::Float(f) if (f - 42.0).abs() < f64::EPSILON));

    let p5: PropValue = 42.0f64.into();
    assert!(matches!(p5, PropValue::Float(f) if (f - 42.0).abs() < f64::EPSILON));

    let p6: PropValue = true.into();
    assert!(matches!(p6, PropValue::Bool(true)));

    let p7: PropValue = "test".into();
    assert!(matches!(p7, PropValue::Text(s) if s == "test"));

    let p8: PropValue = "test".to_string().into();
    assert!(matches!(p8, PropValue::Text(s) if s == "test"));
}
