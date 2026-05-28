use tui_shared::{Entity, Snapshot};

#[test]
fn test_snapshot_methods() {
    let mut snap = Snapshot::new("test");
    snap = snap.with_entity(Entity::new("item"));
    assert_eq!(snap.entities.len(), 1);

    let json = snap.to_json();
    assert!(!json.is_empty());

    let mut snap2 = Snapshot::new("test");
    snap2 = snap2.with_entity(Entity::new("item"));
    let json_pretty = snap2.to_json_pretty();
    assert!(!json_pretty.is_empty());
}
