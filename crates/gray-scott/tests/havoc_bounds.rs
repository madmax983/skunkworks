#![allow(missing_docs)]
use gray_scott::GrayScott;

#[test]
fn havoc_gray_scott_bounds() {
    let mut g = GrayScott::new(10, 10);
    g.add_chemical(1000, 1000, 1.0);
}
