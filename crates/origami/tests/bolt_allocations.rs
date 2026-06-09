use origami::{generate_miura_grid, generate_miura_mesh, MiuraParams, Orientation};

#[test]
fn test_exact_capacity_allocation() {
    let params = MiuraParams {
        a: 1.0,
        b: 1.0,
        gamma: 80.0f32.to_radians(),
        orientation: Orientation::Horizontal,
    };

    // Test mesh indices allocation
    let mesh = generate_miura_mesh(params, (100, 100), 0.5);

    // Before fix: capacity will be larger than length because flat_map hides size
    // After fix: capacity should exactly match length
    assert_eq!(
        mesh.indices.len(),
        mesh.indices.capacity(),
        "Indices capacity should exactly match length to avoid reallocation overhead"
    );

    // Test grid points allocation (Horizontal)
    let grid_h = generate_miura_grid(params, (100, 100), 0.5);
    assert_eq!(
        grid_h.len(),
        grid_h.capacity(),
        "Horizontal grid capacity should exactly match length"
    );

    // Test grid points allocation (Vertical)
    let params_v = MiuraParams {
        orientation: Orientation::Vertical,
        ..params
    };
    let grid_v = generate_miura_grid(params_v, (100, 100), 0.5);
    assert_eq!(
        grid_v.len(),
        grid_v.capacity(),
        "Vertical grid capacity should exactly match length"
    );
}
