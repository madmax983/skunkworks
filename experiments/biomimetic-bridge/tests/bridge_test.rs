use biomimetic_bridge::model::{State, Terrain, World};

#[test]
fn test_ant_bridge_formation() {
    let mut world = World::new(10, 10);

    // Create a Gap at x=5
    for y in 0..10 {
        world.set_terrain(5, y, Terrain::Gap);
    }

    // Place 20 ants at x=4, y=5 (Near the gap)
    // They are crowded.
    for _ in 0..20 {
        world.add_ant(4, 5);
    }

    // Run simulation for a few ticks
    for _ in 0..10 {
        world.update();
    }

    // Assert that at least one ant has become a bridge
    let bridge_formed = world.ants.iter().any(|a| a.state == State::Bridging);
    assert!(
        bridge_formed,
        "Ants should have formed a bridge due to crowding near the gap"
    );
}
