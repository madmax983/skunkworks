use platter::Platter;

#[test]
fn test_platter_zero_dimensions() {
    let p = Platter::new(0, 0);
    assert_eq!(p.width(), 0);
    assert_eq!(p.height(), 0);
    assert_eq!(p.magnetism().len(), 0);
}

#[test]
fn test_platter_bounds_check_safety() {
    let mut p = Platter::new(5, 5);

    // Out of bounds accumulate
    p.accumulate(5, 5, 1.0);
    p.accumulate(100, 100, 1.0);

    // Check that it didn't write anywhere
    for val in p.magnetism().iter() {
        assert_eq!(*val, 0.0);
    }
}
