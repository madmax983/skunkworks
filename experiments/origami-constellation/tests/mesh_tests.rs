use origami_constellation::mesh_gen;

#[test]
fn test_generate_miura_ori() {
    let mesh = mesh_gen::generate_miura_ori(5, 5);
    assert!(mesh.system.particles.len() > 0);
    // Grid: (5+1)x(5+1) = 36 particles
    assert_eq!(mesh.system.particles.len(), 36);
}

#[test]
fn test_generate_yoshimura() {
    let mesh = mesh_gen::generate_yoshimura(5, 4.0);
    assert!(mesh.system.particles.len() > 0);
    // 5 segments => 6 rings (0 to 5). Each ring 6 vertices.
    // 36 particles.
    assert_eq!(mesh.system.particles.len(), 36);
}

#[test]
fn test_generate_solar_array() {
    let mesh = mesh_gen::generate_solar_array();
    assert!(mesh.system.particles.len() > 8); // Hub (8) + Wings
}
