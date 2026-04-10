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
    let _ = v_huge.length_squared();
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

#[test]
fn test_vec4_scale_and_math() {
    let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);

    // Scale
    let scaled = v1.scale(2.0);
    assert_eq!(scaled.x, 2.0);
    assert_eq!(scaled.y, 4.0);
    assert_eq!(scaled.z, 6.0);
    assert_eq!(scaled.w, 8.0);

    // Scale dim
    let scaled_dim = v1.scale_dim(2.0, 3.0, 4.0, 5.0);
    assert_eq!(scaled_dim.x, 2.0);
    assert_eq!(scaled_dim.y, 6.0);
    assert_eq!(scaled_dim.z, 12.0);
    assert_eq!(scaled_dim.w, 20.0);

    // Add / Sub as methods
    let v2 = Vec4::new(10.0, 10.0, 10.0, 10.0);
    let sum = Vec4::add(&v1, v2);
    assert_eq!(sum.x, 11.0);
    assert_eq!(sum.y, 12.0);
    assert_eq!(sum.z, 13.0);
    assert_eq!(sum.w, 14.0);

    let diff = Vec4::sub(&v2, v1);
    assert_eq!(diff.x, 9.0);
    assert_eq!(diff.y, 8.0);
    assert_eq!(diff.z, 7.0);
    assert_eq!(diff.w, 6.0);
}

#[test]
fn test_vec4_rotations_fast() {
    let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
    let theta = std::f32::consts::PI / 2.0;
    let sin_t = theta.sin();
    let cos_t = theta.cos();

    // XY
    let rot_xy = v.rotate_xy_fast(sin_t, cos_t);
    let expected_xy = v.rotate_xy(theta);
    assert!((rot_xy.x - expected_xy.x).abs() < 1e-6);
    assert!((rot_xy.y - expected_xy.y).abs() < 1e-6);

    // XZ
    let rot_xz = v.rotate_xz_fast(sin_t, cos_t);
    let expected_xz = v.rotate_xz(theta);
    assert!((rot_xz.x - expected_xz.x).abs() < 1e-6);
    assert!((rot_xz.z - expected_xz.z).abs() < 1e-6);

    // YW
    let rot_yw = v.rotate_yw_fast(sin_t, cos_t);
    let expected_yw = v.rotate_yw(theta);
    assert!((rot_yw.y - expected_yw.y).abs() < 1e-6);
    assert!((rot_yw.w - expected_yw.w).abs() < 1e-6);

    // YZ
    let rot_yz = v.rotate_yz_fast(sin_t, cos_t);
    let expected_yz = v.rotate_yz(theta);
    assert!((rot_yz.y - expected_yz.y).abs() < 1e-6);
    assert!((rot_yz.z - expected_yz.z).abs() < 1e-6);

    // ZW
    let rot_zw = v.rotate_zw_fast(sin_t, cos_t);
    let expected_zw = v.rotate_zw(theta);
    assert!((rot_zw.z - expected_zw.z).abs() < 1e-6);
    assert!((rot_zw.w - expected_zw.w).abs() < 1e-6);
}

#[test]
fn test_vec4_operator_overloading() {
    let mut v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
    let v2 = Vec4::new(1.0, 1.0, 1.0, 1.0);

    // AddAssign
    v1 += v2;
    assert_eq!(v1.x, 2.0);
    assert_eq!(v1.y, 3.0);
    assert_eq!(v1.z, 4.0);
    assert_eq!(v1.w, 5.0);

    // Div
    let v3 = v1 / 2.0;
    assert_eq!(v3.x, 1.0);
    assert_eq!(v3.y, 1.5);
    assert_eq!(v3.z, 2.0);
    assert_eq!(v3.w, 2.5);
}

#[test]
fn test_vec4_length_normalize_limit() {
    let v = Vec4::new(3.0, 4.0, 0.0, 0.0);

    // length
    assert_eq!(v.length(), 5.0);

    // normalize
    let n = v.normalize();
    assert_eq!(n.x, 0.6);
    assert_eq!(n.y, 0.8);
    assert_eq!(n.z, 0.0);
    assert_eq!(n.w, 0.0);

    // limit
    let limited = v.limit(2.5);
    assert_eq!(limited.x, 1.5);
    assert_eq!(limited.y, 2.0);
    assert_eq!(limited.z, 0.0);
    assert_eq!(limited.w, 0.0);

    // limit non-triggered
    let not_limited = v.limit(10.0);
    assert_eq!(not_limited.x, 3.0);
    assert_eq!(not_limited.y, 4.0);
    assert_eq!(not_limited.z, 0.0);
    assert_eq!(not_limited.w, 0.0);
}

#[test]
fn test_vec4_rotations_regular() {
    let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
    let theta = std::f32::consts::PI / 2.0;

    let r_xy = v.rotate_xy(theta);
    // PI/2 rotation: x' = x*cos - y*sin = 0 - 2 = -2.0, y' = x*sin + y*cos = 1 + 0 = 1.0
    assert!((r_xy.x + 2.0).abs() < 1e-6);
    assert!((r_xy.y - 1.0).abs() < 1e-6);
    assert_eq!(r_xy.z, 3.0);
    assert_eq!(r_xy.w, 4.0);

    let r_xz = v.rotate_xz(theta);
    // PI/2 rotation: x' = x*cos - z*sin = 0 - 3 = -3.0, z' = x*sin + z*cos = 1 + 0 = 1.0
    assert!((r_xz.x + 3.0).abs() < 1e-6);
    assert_eq!(r_xz.y, 2.0);
    assert!((r_xz.z - 1.0).abs() < 1e-6);
    assert_eq!(r_xz.w, 4.0);

    let r_xw = v.rotate_xw(theta);
    assert!((r_xw.x + 4.0).abs() < 1e-6);
    assert_eq!(r_xw.y, 2.0);
    assert_eq!(r_xw.z, 3.0);
    assert!((r_xw.w - 1.0).abs() < 1e-6);

    let r_yz = v.rotate_yz(theta);
    assert_eq!(r_yz.x, 1.0);
    assert!((r_yz.y + 3.0).abs() < 1e-6);
    assert!((r_yz.z - 2.0).abs() < 1e-6);
    assert_eq!(r_yz.w, 4.0);

    let r_yw = v.rotate_yw(theta);
    assert_eq!(r_yw.x, 1.0);
    assert!((r_yw.y + 4.0).abs() < 1e-6);
    assert_eq!(r_yw.z, 3.0);
    assert!((r_yw.w - 2.0).abs() < 1e-6);

    let r_zw = v.rotate_zw(theta);
    assert_eq!(r_zw.x, 1.0);
    assert_eq!(r_zw.y, 2.0);
    assert!((r_zw.z + 4.0).abs() < 1e-6);
    assert!((r_zw.w - 3.0).abs() < 1e-6);
}
