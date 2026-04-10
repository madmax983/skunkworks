use origami::{generate_miura_mesh, MiuraParams, Orientation};

#[test]
fn test_havoc_miura_mesh_capacity_panic() {
    let params = MiuraParams {
        a: 1.0,
        b: 1.0,
        gamma: 1.4,
        orientation: Orientation::Horizontal,
    };

    // 🔒 Warden: Pass usize::MAX to trigger integer overflow on capacity calculation
    // It should now return an empty mesh instead of panicking.
    let mesh = generate_miura_mesh(params, (usize::MAX, usize::MAX), 0.5);
    assert!(mesh.vertices.is_empty());
    assert!(mesh.indices.is_empty());
}
