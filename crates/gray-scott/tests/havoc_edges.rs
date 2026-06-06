use gray_scott::GrayScott;

#[test]
fn havoc_gray_scott_negative() {
    let mut g = GrayScott::new(100, 100);
    g.update(f32::NEG_INFINITY, f32::INFINITY, f32::NAN);
}
