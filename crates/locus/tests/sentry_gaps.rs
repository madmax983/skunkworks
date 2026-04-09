use locus::{Topology, Vec2, Vec4};

#[test]
fn test_topology_sphere_overflow() {
    let topo = Topology::Sphere;
    let width = i64::MAX as usize;
    let height = 100;
    let x = (width - 1) as i64;
    let y = height as i64;

    let res = topo.normalize(y, x, width, height);

    if let Some((ny, nx)) = res {
        assert_eq!(ny, 99);
        assert_eq!(nx, (width / 2) - 1);
    } else {
        panic!("Should have returned Some");
    }
}

#[test]
fn test_vec2_limit_negative() {
    let v = Vec2::new(10.0, 0.0);
    let limited = v.limit(-5.0);
    assert_eq!(limited.x, 5.0);
    assert_eq!(limited.y, 0.0);
}

#[test]
fn test_vec4_limit_negative() {
    let v = Vec4::new(10.0, 0.0, 0.0, 0.0);
    let limited = v.limit(-5.0);
    assert_eq!(limited.x, 5.0);
}

#[test]
fn test_vec2_reflect_zero_len() {
    let v = Vec2::new(0.0, 0.0);
    let n = Vec2::new(1.0, 0.0);
    let r = v.reflect(n);
    assert_eq!(r, Vec2::new(0.0, 0.0));
}

#[test]
fn test_vec2_reflect_non_zero() {
    let v = locus::Vec2::new(1.0, 1.0);
    let n = locus::Vec2::new(0.0, 1.0);
    let r = v.reflect(n);
    assert_eq!(r, locus::Vec2::new(1.0, -1.0));
}

#[test]
fn test_vec2_normalize_finite_inf() {
    let v = Vec2::new(1e300, 1e300);
    let n = v.normalize();
    assert!((n.x - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-6);
    assert!((n.y - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-6);
}

#[test]
fn test_vec4_normalize_finite_inf() {
    let v = Vec4::new(1e38, 1e38, 1e38, 1e38);
    let n = v.normalize();
    assert!((n.x - 0.5).abs() < 1e-6);
}

#[test]
fn test_vec2_limit_finite_inf() {
    let v = Vec2::new(1e300, 1e300);
    let max = 1e300;
    let l = v.limit(max);
    assert!((l.x - max * std::f64::consts::FRAC_1_SQRT_2).abs() < 1e290);
}

#[test]
fn test_vec4_limit_finite_inf() {
    let v = Vec4::new(1e38, 1e38, 1e38, 1e38);
    let max = 1e38;
    let l = v.limit(max);
    assert!((l.x - max * 0.5).abs() < 1e30);
}

#[test]
fn test_vec2_limit_infinite() {
    let v = Vec2::new(f64::INFINITY, 0.0);
    let l = v.limit(f64::INFINITY);
    assert_eq!(l.x, f64::INFINITY);
}

#[test]
fn test_vec4_limit_infinite() {
    let v = Vec4::new(f32::INFINITY, 0.0, 0.0, 0.0);
    let l = v.limit(f32::INFINITY);
    assert_eq!(l.x, f32::INFINITY);
}

#[test]
fn test_vec2_normalize_inf_neg_inf() {
    let v = Vec2::new(f64::NEG_INFINITY, f64::INFINITY);
    let n = v.normalize();
    assert!((n.x - -std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-6);
    assert!((n.y - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-6);

    let v2 = Vec2::new(f64::INFINITY, f64::NEG_INFINITY);
    let n2 = v2.normalize();
    assert!((n2.x - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-6);
    assert!((n2.y - -std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-6);

    let v3 = Vec2::new(f64::NEG_INFINITY, f64::NEG_INFINITY);
    let n3 = v3.normalize();
    assert!((n3.x - -std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-6);
    assert!((n3.y - -std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-6);
}

#[test]
fn test_vec4_normalize_overflow() {
    let v = Vec4::new(f32::MAX, f32::MAX, f32::MAX, f32::MAX);
    let l = v.limit(f32::MAX);
    assert!((l.x - f32::MAX * 0.5).abs() < 1e30);
}

#[test]
fn test_vec2_normalize_inf_finite() {
    let v = Vec2::new(0.0, f64::INFINITY);
    let n = v.normalize();
    assert_eq!(n.y, 1.0);
    assert_eq!(n.x, 0.0);

    let v2 = Vec2::new(f64::INFINITY, 0.0);
    let n2 = v2.normalize();
    assert_eq!(n2.x, 1.0);
    assert_eq!(n2.y, 0.0);
}
