use locus::Topology;

#[test]
#[cfg(target_pointer_width = "64")]
fn test_havoc_overflow() {
    let topo = Topology::Torus;
    let width = usize::MAX; // On 64-bit, this is u64::MAX, which casts to -1_i64
    let height = 100;

    // This previously panicked due to i64::MIN / -1 overflow
    // Now it should return None because width is too large
    let res = topo.normalize(50, i64::MIN, width, height);

    assert_eq!(res, None);
}
