use gray_scott::GrayScott;

#[test]
fn test_gray_scott_properties() {
    let gs = GrayScott::new(10, 10);
    assert_eq!(gs.width(), 10);
    assert_eq!(gs.height(), 10);
    assert_eq!(gs.u().len(), 100);
    assert_eq!(gs.v().len(), 100);
}

#[test]
fn test_gray_scott_add_chemical_zero_bounds() {
    let mut gs = GrayScott::new(0, 0);
    // Add chemical to a zero sized grid, should not panic.
    gs.add_chemical(0, 0, 1.0);
}
