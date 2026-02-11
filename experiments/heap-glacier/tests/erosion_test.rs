use heap_glacier::simulation::HeapTerrain;

#[test]
fn test_allocation_increases_ice() {
    let mut terrain = HeapTerrain::new(10, 10);
    terrain.allocate(5, 5, 5.0);
    assert!(terrain.get_ice(5, 5) > 0.0, "Ice should increase after allocation");
}

#[test]
fn test_deallocation_creates_water() {
    let mut terrain = HeapTerrain::new(10, 10);
    // Simulate existing ice first if allocate is stubbed (though I'll test allocate too)
    // Directly setting ice to ensure dealloc has something to work with if allocate fails
    terrain.allocate(5, 5, 5.0);

    terrain.deallocate(5, 5, 5.0);
    assert!(terrain.get_water(5, 5) > 0.0, "Water should appear after deallocation");
}

#[test]
fn test_erosion_lowers_bedrock() {
    let mut terrain = HeapTerrain::new(10, 10);
    let initial_bedrock = terrain.get_bedrock(5, 5);

    // Add water
    // We access internal vector for setup because deallocate might be broken/stubbed
    // But since `water` is public field in struct definition I wrote, this is fine.
    terrain.water[5 * 10 + 5] = 100.0;

    // Run simulation
    terrain.tick();

    assert!(terrain.get_bedrock(5, 5) < initial_bedrock, "Bedrock should erode under water flow");
}
