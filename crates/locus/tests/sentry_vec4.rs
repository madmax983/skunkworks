use locus::vec4::Vec4;

#[test]
fn test_vec4_is_finite() {
    let finite = Vec4::new(1.0, 2.0, -3.0, 4.0);
    assert!(finite.is_finite());

    // Test Inf
    let inf_x = Vec4::new(f32::INFINITY, 0.0, 0.0, 0.0);
    assert!(!inf_x.is_finite());

    let inf_y = Vec4::new(0.0, f32::NEG_INFINITY, 0.0, 0.0);
    assert!(!inf_y.is_finite());

    // Test NaN
    let nan_z = Vec4::new(0.0, 0.0, f32::NAN, 0.0);
    assert!(!nan_z.is_finite());

    let nan_w = Vec4::new(0.0, 0.0, 0.0, f32::NAN);
    assert!(!nan_w.is_finite());
}

#[test]
fn test_vec4_length_underflow_overflow() {
    // Underflow case (very small numbers shouldn't panic, may just go to 0.0 due to f32 precision)
    let tiny = f32::MIN_POSITIVE;
    let v_tiny = Vec4::new(tiny, tiny, tiny, tiny);
    let len_sq_tiny = v_tiny.length_squared();
    assert!(len_sq_tiny >= 0.0 && len_sq_tiny.is_finite());

    // Overflow case (large numbers)
    let huge = 2.0e19; // huge * huge = 4e38 > f32::MAX
    let v_huge = Vec4::new(huge, 0.0, 0.0, 0.0);
    assert_eq!(v_huge.length_squared(), f32::INFINITY);
    assert_eq!(v_huge.length(), f32::INFINITY);
}

#[test]
fn test_vec4_normalize_extreme() {
    // Zero vector
    assert_eq!(Vec4::zero().normalize(), Vec4::zero());

    // Underflow case
    let tiny = f32::MIN_POSITIVE;
    let v_tiny = Vec4::new(tiny, 0.0, 0.0, 0.0);
    let norm_tiny = v_tiny.normalize();
    // Due to robust normalization, it should accurately normalize without underflowing completely
    assert_eq!(norm_tiny.x, 1.0);
    assert_eq!(norm_tiny.y, 0.0);

    // Overflow case (large finite components)
    let huge = f32::MAX;
    let v_huge = Vec4::new(huge, huge, huge, huge);
    let norm_huge = v_huge.normalize();
    assert!(norm_huge.is_finite());
    // Normalized vector should have length close to 1.0
    assert!((norm_huge.length() - 1.0).abs() < 1e-5);

    // Exactly f32::MAX on one axis
    let v_max = Vec4::new(f32::MAX, 0.0, 0.0, 0.0);
    let norm_max = v_max.normalize();
    assert_eq!(norm_max.x, 1.0);
    assert_eq!(norm_max.y, 0.0);
}

#[test]
fn test_vec4_project_to_3d_edge_cases() {
    let camera_w = 10.0;

    // When w is very close to camera_w, w_dist approaches 0.
    // The implementation clamps w_dist.max(0.1).
    let v_close = Vec4::new(1.0, 1.0, 1.0, 9.95);
    // w_dist = 10.0 - 9.95 = 0.05. Clamped to 0.1.
    // scale = 2.0 / 0.1 = 20.0
    let proj = v_close.project_to_3d(camera_w);
    assert!((proj.x - 20.0).abs() < 1e-4);

    // When w is exactly camera_w
    let v_exact = Vec4::new(2.0, 0.0, 0.0, 10.0);
    // w_dist = 0.0. Clamped to 0.1.
    // scale = 2.0 / 0.1 = 20.0. x = 2.0 * 20.0 = 40.0.
    let proj_exact = v_exact.project_to_3d(camera_w);
    assert!((proj_exact.x - 40.0).abs() < 1e-4);

    // When w is greater than camera_w (point is behind camera)
    let v_behind = Vec4::new(1.0, 1.0, 1.0, 15.0);
    // w_dist = -5.0. Clamped to 0.1.
    // scale = 2.0 / 0.1 = 20.0
    let proj_behind = v_behind.project_to_3d(camera_w);
    assert!((proj_behind.x - 20.0).abs() < 1e-4);
}
