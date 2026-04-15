use gray_scott::GrayScott;

#[test]
fn test_get_index_in_bounds() {
    let gs = GrayScott::new(10, 10);
    assert_eq!(gs.get_index(0, 0), 0);
    assert_eq!(gs.get_index(9, 9), 99);
}

#[test]
#[should_panic(expected = "coordinate out of bounds")]
fn test_get_index_out_of_bounds_x() {
    let gs = GrayScott::new(10, 10);
    // This should panic instead of silently wrapping to the next row
    let _idx = gs.get_index(10, 0);
}

#[test]
#[should_panic(expected = "coordinate out of bounds")]
fn test_get_index_out_of_bounds_y() {
    let gs = GrayScott::new(10, 10);
    let _idx = gs.get_index(0, 10);
}

#[test]
fn test_gray_scott_overflow() {
    let gs = GrayScott::new(usize::MAX, 2);
    assert_eq!(gs.width(), 0);
    assert_eq!(gs.height(), 0);
}
