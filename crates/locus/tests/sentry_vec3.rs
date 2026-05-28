use locus::Vec3;

#[cfg(feature = "macroquad")]
#[test]
fn test_vec3_macroquad_conversion_back() {
    use macroquad::prelude::Vec3 as MqVec3;
    let mq_vec = MqVec3::new(10.0, 20.0, 30.0);
    let locus_vec: Vec3 = mq_vec.into();
    assert_eq!(locus_vec.y, 20.0);
}

#[test]
fn test_vec3_default_and_ops() {
    let v = Vec3::new(0.0, 0.0, 0.0);
    assert_eq!(v.x, 0.0);
    assert_eq!(v.y, 0.0);
    assert_eq!(v.z, 0.0);
}
