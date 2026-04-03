use origami::{generate_miura_mesh, MiuraParams, Orientation};

#[test]
#[should_panic]
fn havoc_origami_overflow() {
    let params = MiuraParams {
        a: 1.0,
        b: 1.0,
        gamma: 1.4,
        orientation: Orientation::Horizontal,
    };

    // 👺 HAVOC: Trigger an integer overflow when calculating capacity and indices.
    // The generator calculates `(rows + 1) * (cols + 1)` and `rows * cols * 6`
    // using raw `usize` multiplication without checked math.
    let _mesh = generate_miura_mesh(params, (usize::MAX, usize::MAX), 0.5);
}
