use locus::Vec2;

#[test]
fn test_vec2_math_operations() {
    let mut v1 = Vec2::new(10.0, 20.0);
    let v2 = Vec2::new(2.0, 4.0);

    // Sub
    let v_sub = v1 - v2;
    assert_eq!(v_sub, Vec2::new(8.0, 16.0));

    // SubAssign
    v1 -= v2;
    assert_eq!(v1, Vec2::new(8.0, 16.0));

    // Mul
    let v_mul = v1 * 2.0;
    assert_eq!(v_mul, Vec2::new(16.0, 32.0));

    // MulAssign
    v1 *= 2.0;
    assert_eq!(v1, Vec2::new(16.0, 32.0));

    // Div
    let v_div = v1 / 4.0;
    assert_eq!(v_div, Vec2::new(4.0, 8.0));

    // DivAssign
    v1 /= 4.0;
    assert_eq!(v1, Vec2::new(4.0, 8.0));

    // Neg
    let v_neg = -v1;
    assert_eq!(v_neg, Vec2::new(-4.0, -8.0));
}

#[test]
fn test_vec2_from_tuple() {
    let v: Vec2 = (1.0, 2.0).into();
    assert_eq!(v, Vec2::new(1.0, 2.0));
}

#[test]
fn test_vec2_add() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);
    assert_eq!(v1 + v2, Vec2::new(4.0, 6.0));
}

#[test]
fn test_vec2_add_assign() {
    let mut v1 = Vec2::new(10.0, 20.0);
    let v2 = Vec2::new(2.0, 4.0);
    v1 += v2;
    assert_eq!(v1, Vec2::new(12.0, 24.0));
}

#[test]
fn test_vec2_rotate_coverage() {
    let v = Vec2::new(1.0, 0.0);
    let rotated = v.rotate(std::f64::consts::PI);
    assert!((rotated.x - -1.0).abs() < 1e-10);
    assert!((rotated.y - 0.0).abs() < 1e-10);
}
