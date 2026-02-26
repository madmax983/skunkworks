use locus::{Topology, Vec2, Vec4};

#[test]
// This should NO LONGER panic after the fix
fn test_topology_sphere_overflow() {
    let topo = Topology::Sphere;
    // Use a large width that fits in i64
    let width = i64::MAX as usize;
    let height = 100;

    // x = 0.
    // y = height (100). wrap_y = 100/100 = 1 (odd).
    // This triggers the sphere pole crossing logic.
    // nx = (nx + w/2) % w
    // If nx is large, nx + w/2 overflows i64.

    // We need nx to be large. nx = x % w.
    // Let's pick x = width - 1.
    let x = (width - 1) as i64;
    let y = height as i64;

    // This call should NOT panic now
    let res = topo.normalize(y, x, width, height);

    // Let's verify the result manually.
    // wrap_y = 1 (odd). ny = 100 % 100 = 0.
    // Pole crossing: ny = (h-1) - ny = 99 - 0 = 99.
    // nx:
    // x = width - 1.
    // shift = width / 2.
    // x >= width - shift ?
    // width - 1 >= width - width/2
    // width - 1 >= width/2. (True for large width).
    // So nx -= width - shift;
    // nx = (width - 1) - (width - width/2)
    // nx = width - 1 - width + width/2
    // nx = width/2 - 1.

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
    // Passing a negative limit should probably limit the magnitude to abs(max)
    // without flipping the vector.
    // Currently, if max is -5.0:
    // mag_sq (100) > max*max (25).
    // result = normalize(1,0) * -5.0 = (-5, 0).
    // This flips the vector! Expected behavior for "limit" is usually capping magnitude.
    // If I limit my speed to -5, it doesn't mean run backwards.
    let limited = v.limit(-5.0);

    // We expect it to be (5.0, 0.0), maintaining direction but capping length.
    assert_eq!(limited.x, 5.0);
    assert_eq!(limited.y, 0.0);
}

#[test]
fn test_vec4_limit_negative() {
    let v = Vec4::new(10.0, 0.0, 0.0, 0.0);
    let limited = v.limit(-5.0);

    // Same expectation as Vec2
    assert_eq!(limited.x, 5.0);
}
