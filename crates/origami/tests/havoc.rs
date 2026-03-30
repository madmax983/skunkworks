use origami::{generate_miura_mesh, MiuraParams, Orientation};

#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn havoc_miura_grid_overflow() {
    let params = MiuraParams {
        a: 1.0,
        b: 1.0,
        gamma: 0.5,
        orientation: Orientation::Horizontal,
    };
    // 👺 Havoc: Proving that grid dimensions are not checked against overflows before allocating
    // The expression `(rows + 1) * (cols + 1)` will panic when one of them is usize::MAX.
    let _ = generate_miura_mesh(params, (usize::MAX, 2), 1.0);
}
