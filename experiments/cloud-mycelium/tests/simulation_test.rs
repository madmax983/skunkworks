use cloud_mycelium::World;
use glam::Vec2;

#[test]
fn test_load_transfer() {
    let mut world = World::new(800.0, 600.0);

    // Add two mushrooms
    let m1 = world.add_mushroom(Vec2::new(100.0, 100.0));
    let m2 = world.add_mushroom(Vec2::new(200.0, 100.0));

    // Manually connect them to ensure graph
    world.connect(m1, m2);

    // Overload m1
    // We need to access mushrooms. Since they are in a Vec, we can index.
    world.mushrooms[m1].load = 150.0; // Capacity is 100
    world.mushrooms[m2].load = 0.0;

    // Update
    world.update(0.1);

    // Check if packet was spawned (Transfer happens)
    // Packet should have spawned from m1 to m2
    let packet_exists = world.packets.iter().any(|p| p.target_index == m2);

    // Check if load decreased on m1
    let load_decreased = world.mushrooms[m1].load < 150.0;

    assert!(load_decreased, "Source load should decrease (Current: {})", world.mushrooms[m1].load);
    // Note: Packet might not spawn if transfer amount is < epsilon, but with 50 excess and 0.1 dt, it should be 25.
    assert!(packet_exists, "Packet should be spawned to transfer load");
}
