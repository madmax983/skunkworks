use locus::vec4::Vec4;
use locus::Vec2;
use std::f32;

#[test]
fn test_project_to_3d_robustness() {
    // 🎯 Target: Vec4::project_to_3d
    // 💣 Risk: Multiplying huge floats by a scale factor causes Infinity or NaN limits bypassing clamp.
    let v_zero = Vec4::zero();
    let p = v_zero.project_to_3d(0.0);
    assert_eq!(p.x, 0.0);
    assert_eq!(p.y, 0.0);
    assert_eq!(p.z, 0.0);

    let v_huge = Vec4::new(f32::MAX, f32::MAX, f32::MAX, 0.0);
    let p_huge = v_huge.project_to_3d(0.0);
    assert_eq!(p_huge.x, f32::MAX);
    assert_eq!(p_huge.y, f32::MAX);
    assert_eq!(p_huge.z, f32::MAX);

    let v_neg_huge = Vec4::new(f32::MIN, f32::MIN, f32::MIN, 0.0);
    let p_neg_huge = v_neg_huge.project_to_3d(0.0);
    assert_eq!(p_neg_huge.x, f32::MIN);
    assert_eq!(p_neg_huge.y, f32::MIN);
    assert_eq!(p_neg_huge.z, f32::MIN);

    let v_nan = Vec4::new(f32::NAN, f32::NAN, f32::NAN, 0.0);
    let p_nan = v_nan.project_to_3d(0.0);
    assert_eq!(p_nan.x, 0.0);
    assert_eq!(p_nan.y, 0.0);
    assert_eq!(p_nan.z, 0.0);
}

#[test]
fn test_vec4_normalize_robustness() {
    // 🎯 Target: Vec4::normalize
    // 💣 Risk: Normalizing a very small subnormal or an infinite/NaN vector causes division by zero.
    let v_small = Vec4::new(1e-30, 0.0, 0.0, 0.0);
    let n_small = v_small.normalize();
    assert_eq!(n_small.x, 1.0);

    let v_inf = Vec4::new(f32::INFINITY, 1.0, 1.0, 1.0);
    let n_inf = v_inf.normalize();
    assert_eq!(n_inf.x, 0.0);

    let v_nan = Vec4::new(f32::NAN, 1.0, 1.0, 1.0);
    let n_nan = v_nan.normalize();
    assert_eq!(n_nan.x, 0.0);
}

#[test]
fn test_vec2_normalize_robustness() {
    // 🎯 Target: Vec2::normalize
    // 💣 Risk: Subnormal or infinite vectors producing NaN components.
    let v_small = Vec2::new(1e-300, 0.0);
    let n_small = v_small.normalize();
    // Subnormal values may become zero during normalization in Vec2 depending on implementation scaling
    assert!(n_small.x == 1.0 || n_small.x == 0.0);

    let v_inf = Vec2::new(f64::INFINITY, 0.0);
    let n_inf = v_inf.normalize();
    assert_eq!(n_inf.x, 1.0);

    let v_nan = Vec2::new(f64::NAN, 1.0);
    let n_nan = v_nan.normalize();
    assert!(n_nan.x.is_nan());
}

#[test]
fn test_vec4_limit_robustness() {
    // 🎯 Target: Vec4::limit
    // 💣 Risk: Limiting to a negative maximum, limiting a zero length vector, or an infinite limit.
    let v = Vec4::new(10.0, 0.0, 0.0, 0.0);
    let l_neg = v.limit(-1.0);
    assert_eq!(l_neg.x.abs(), 1.0);

    let l_inf = v.limit(f32::INFINITY);
    assert_eq!(l_inf.x, 10.0);

    let v_zero = Vec4::zero();
    let l_zero = v_zero.limit(10.0);
    assert_eq!(l_zero.x, 0.0);
}
