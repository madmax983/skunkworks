use origami::{generate_miura_grid, generate_miura_mesh, MiuraParams, Orientation};

#[test]
fn test_sentry_generate_miura_grid_zero_size() {
    let params = MiuraParams {
        a: 1.0,
        b: 1.0,
        gamma: 1.4,
        orientation: Orientation::Horizontal,
    };

    let grid = generate_miura_grid(params, (0, 0), 0.5);
    assert_eq!(grid.len(), 1); // 1 point (0+1) * (0+1)

    let mesh = generate_miura_mesh(params, (0, 0), 0.5);
    assert_eq!(mesh.vertices.len(), 1);
    assert_eq!(mesh.indices.len(), 0);
}

#[test]
fn test_sentry_generate_miura_grid_nan_extension() {
    let params = MiuraParams {
        a: 1.0,
        b: 1.0,
        gamma: 1.4,
        orientation: Orientation::Horizontal,
    };

    // NaN extension factor should clamp correctly or not panic
    let grid = generate_miura_grid(params, (2, 2), f32::NAN);
    assert_eq!(grid.len(), 9);

    let mesh = generate_miura_mesh(params, (2, 2), f32::NAN);
    assert_eq!(mesh.vertices.len(), 9);
}
