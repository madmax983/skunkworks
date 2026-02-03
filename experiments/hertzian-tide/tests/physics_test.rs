use hertzian_tide::wave_tank::WaveTank;

#[test]
fn test_wave_propagation() {
    let mut tank = WaveTank::new(10, 10);

    // Poke the center (5, 5)
    tank.poke(5, 5, 1.0);

    assert_eq!(tank.get_height(5, 5), 1.0, "Center should be raised after poke");
    assert_eq!(tank.get_height(6, 5), 0.0, "Neighbor should be 0 before step");

    // Step simulation
    tank.step();

    // After step, the wave should propagate to neighbors
    // Note: Exact value depends on algorithm, but it should be non-zero
    let neighbor_height = tank.get_height(6, 5);
    assert!(neighbor_height != 0.0, "Wave should propagate to neighbor (6, 5). Got {}", neighbor_height);
}
