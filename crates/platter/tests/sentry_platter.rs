use platter::Platter;

#[test]
#[should_panic(expected = "Platter size overflow")]
fn test_platter_overflow() {
    let _ = Platter::new(usize::MAX, 2);
}

#[test]
fn test_platter_clear() {
    let mut p = Platter::new(10, 10);
    p.accumulate(2, 2, 1.0);
    p.clear();
    assert_eq!(p.get_magnetism(2, 2), 0.0);
}
