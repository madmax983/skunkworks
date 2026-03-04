use poincare_disk::{neighbor_transform_a, TilingConsts};

#[test]
fn test_neighbor_transform_a() {
    let consts = TilingConsts::new_4_5();

    // Direction 0: Right (0 rad)
    let right_neighbor = neighbor_transform_a(0, &consts);
    assert!((right_neighbor.re - consts.neighbor_offset).abs() < 1e-9);
    assert!(right_neighbor.im.abs() < 1e-9);

    // Direction 1: Up (PI/2 rad)
    let up_neighbor = neighbor_transform_a(1, &consts);
    assert!(up_neighbor.re.abs() < 1e-9);
    assert!((up_neighbor.im - consts.neighbor_offset).abs() < 1e-9);

    // Direction 2: Left (PI rad)
    let left_neighbor = neighbor_transform_a(2, &consts);
    assert!((left_neighbor.re + consts.neighbor_offset).abs() < 1e-9);
    assert!(left_neighbor.im.abs() < 1e-9);

    // Direction 3: Down (3*PI/2 rad)
    let down_neighbor = neighbor_transform_a(3, &consts);
    assert!(down_neighbor.re.abs() < 1e-9);
    assert!((down_neighbor.im + consts.neighbor_offset).abs() < 1e-9);
}
