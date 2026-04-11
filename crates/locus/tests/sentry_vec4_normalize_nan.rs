use locus::vec4::Vec4;
use std::f32;

#[test]
fn test_vec4_normalize_nan() {
    let v = Vec4::new(f32::NAN, 0.0, 0.0, 0.0);
    let n = v.normalize();
    assert_eq!(n, Vec4::zero());
}
