use locus::vec4::Vec4;

#[test]
fn test_vec4_distance_squared() {
    let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
    let v2 = Vec4::new(2.0, 4.0, 6.0, 8.0);

    // (1-2)^2 + (2-4)^2 + (3-6)^2 + (4-8)^2
    // 1 + 4 + 9 + 16 = 30
    assert_eq!(v1.distance_squared(v2), 30.0);
}

#[test]
fn test_vec4_rotate_xw_fast() {
    let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
    // rotate 90 degrees
    let sin_theta = 1.0;
    let cos_theta = 0.0;

    let rotated = v.rotate_xw_fast(sin_theta, cos_theta);

    // x = x * cos - w * sin = 1*0 - 4*1 = -4.0
    // y = 2.0
    // z = 3.0
    // w = x * sin + w * cos = 1*1 + 4*0 = 1.0
    assert_eq!(rotated.x, -4.0);
    assert_eq!(rotated.y, 2.0);
    assert_eq!(rotated.z, 3.0);
    assert_eq!(rotated.w, 1.0);
}

#[test]
fn test_vec4_project_to_3d_overflow_clamp() {
    // scale = 2.0 / (camera_w - v.w).max(0.1)
    // To get a large scale to trigger overflow, we need x, y, z to be large.
    let v_positive_overflow = Vec4::new(f32::MAX, f32::MAX, f32::MAX, 0.0);

    let _camera_w = 2.0;
    // _camera_w - v.w = 2.0 - 0.0 = 2.0
    // max(2.0, 0.1) = 2.0
    // scale = 2.0 / 2.0 = 1.0
    // This won't overflow if we just multiply by 1.0. We need scale > 1.0, or v values to be very close to MAX.
    // wait, f32::MAX * 1.0 is still f32::MAX (finite).

    // Let's make scale > 1.0.
    // camera_w = 1.0. camera_w - 0.0 = 1.0.
    // scale = 2.0 / 1.0 = 2.0.
    // f32::MAX * 2.0 = +infinity.
    let proj1 = v_positive_overflow.project_to_3d(1.0);

    // The clamp logic says:
    // if res > 0.0 { f32::MAX }
    assert_eq!(proj1.x, f32::MAX);
    assert_eq!(proj1.y, f32::MAX);
    assert_eq!(proj1.z, f32::MAX);

    // Negative overflow
    let v_negative_overflow = Vec4::new(-f32::MAX, -f32::MAX, -f32::MAX, 0.0);
    let proj2 = v_negative_overflow.project_to_3d(1.0);

    // The clamp logic says:
    // if res < 0.0 { f32::MIN }
    assert_eq!(proj2.x, f32::MIN);
    assert_eq!(proj2.y, f32::MIN);
    assert_eq!(proj2.z, f32::MIN);

    // NaN case (0.0 * inf, or inf - inf, etc)
    // Actually, safe_mul handles NaN. Let's see if we can trigger res.is_finite() being false but not >0 or <0
    // i.e., res is NaN.
    // res > 0.0 is false for NaN.
    // res < 0.0 is false for NaN.
    // So it should hit the `else { 0.0 }` block.
    // How to get NaN? f32::NAN * scale = NaN.
    let v_nan = Vec4::new(f32::NAN, f32::NAN, f32::NAN, 0.0);
    let proj3 = v_nan.project_to_3d(1.0);
    assert_eq!(proj3.x, 0.0);
    assert_eq!(proj3.y, 0.0);
    assert_eq!(proj3.z, 0.0);
}
