use locust_net::World;

#[test]
fn test_locust_moves_towards_target() {
    let mut world = World::new(100.0, 100.0);

    // Create a server (target) at (50.0, 50.0)
    let server_idx = world.add_server(50.0, 50.0);

    // Create a locust at (0.0, 0.0) targeting the server
    world.add_locust(0.0, 0.0, Some(server_idx));

    let locust_start = world.get_locust(0).position;

    // Update the world (move locusts)
    world.update(1.0/60.0);

    let locust_end = world.get_locust(0).position;

    // Check if the locust moved closer to the target
    let start_dist = (locust_start.x - 50.0).hypot(locust_start.y - 50.0);
    let end_dist = (locust_end.x - 50.0).hypot(locust_end.y - 50.0);

    assert!(end_dist < start_dist, "Locust should move towards target");
}
