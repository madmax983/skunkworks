use platter::Platter;

#[test]
#[should_panic(expected = "Platter size overflow")]
fn test_platter_overflow() {
    let _p = Platter::new(usize::MAX, 2);
}

#[test]
fn test_platter_clear() {
    let mut p = Platter::new(10, 10);
    p.accumulate(2, 2, 1.0);
    p.clear();
    assert_eq!(p.get_magnetism(2, 2), 0.0);
}

#[test]
fn test_sentry_get_magnetism_out_of_bounds() {
    let platter = Platter::new(10, 10);

    // Normal access
    assert_eq!(platter.get_magnetism(5, 5), 0.0);

    // Out of bounds access triggering unwrap_or(0.0)
    assert_eq!(platter.get_magnetism(10, 5), 0.0);
    assert_eq!(platter.get_magnetism(5, 10), 0.0);
    assert_eq!(platter.get_magnetism(99, 99), 0.0);
}
