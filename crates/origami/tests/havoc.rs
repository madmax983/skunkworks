use origami::{generate_miura_mesh, MiuraParams, Orientation};

#[test]
fn havoc_origami_overflow() {
    let params = MiuraParams {
        a: 1.0,
        b: 1.0,
        gamma: 1.4,
        orientation: Orientation::Horizontal,
    };

    // 🔒 WARDEN: Trigger an integer overflow when calculating capacity and indices.
    // The generator calculates `(rows + 1) * (cols + 1)` and `rows * cols * 6`.
    // It should now return an empty mesh instead of panicking.
    let mesh = generate_miura_mesh(params, (usize::MAX, usize::MAX), 0.5);
    assert!(mesh.vertices.is_empty());
    assert!(mesh.indices.is_empty());
}
