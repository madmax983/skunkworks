#![allow(missing_docs)]
use gray_scott::GrayScott;

#[test]
fn test_gray_scott_overflow() {
    let p = GrayScott::new(usize::MAX, 2);
    assert_eq!(p.width(), 0);
    assert_eq!(p.height(), 0);
}
