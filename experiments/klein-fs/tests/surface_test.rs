use klein_fs::surface;
use std::f32::consts::PI;

#[test]
fn test_periodicity() {
    // Check u-periodicity
    let p0 = surface::figure_8_immersion(0.0, 0.0);
    let p2pi = surface::figure_8_immersion(2.0 * PI, 0.0);

    let diff = (p0 - p2pi).length();
    assert!(diff < 1e-5, "u=0 and u=2pi should be the same point, diff: {}", diff);

    // Check v-periodicity
    let p0v0 = surface::figure_8_immersion(1.0, 0.0);
    let p0v2pi = surface::figure_8_immersion(1.0, 2.0 * PI);
    let diff_v = (p0v0 - p0v2pi).length();
    assert!(diff_v < 1e-5, "v=0 and v=2pi should be the same point, diff: {}", diff_v);
}

#[test]
fn test_mesh_generation() {
    let (vertices, indices) = surface::generate_wireframe_mesh(10, 10);
    assert!(!vertices.is_empty());
    assert!(!indices.is_empty());
}
