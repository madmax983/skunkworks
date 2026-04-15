use platter::Platter;

#[test]
fn test_platter_overflow() {
    let p = Platter::new(usize::MAX, 2);
    assert_eq!(p.width(), 0);
    assert_eq!(p.height(), 0);
    assert_eq!(p.magnetism().len(), 0);
}
