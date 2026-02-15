use locus::Topology;

#[test]
fn test_havoc_mobius_overflow() {
    let topo = Topology::Mobius;
    let width = 10;
    let height = 10;

    // This used to panic in debug mode due to overflow: (height - 1) - y
    // Now it should return None
    let res = topo.normalize(i64::MIN, 10, width, height);
    assert_eq!(res, None);
}

#[test]
fn test_havoc_mobius_max() {
    let topo = Topology::Mobius;
    let width = 10;
    let height = 10;

    // 9 - i64::MAX -> Large negative number. Should return None (out of bounds).
    let res = topo.normalize(i64::MAX, 10, width, height);
    assert_eq!(res, None);
}

#[test]
fn test_havoc_klein_min() {
    let topo = Topology::Klein;
    let width = 10;
    let height = 10;

    // Klein uses rem_euclid and div_euclid, which handle negative numbers correctly.
    // i64::MIN might still be tricky for division if divisor is -1, but h is positive.
    let res = topo.normalize(i64::MIN, i64::MIN, width, height);
    // We don't assert the exact result, just that it doesn't panic.
    assert!(res.is_some());
}
