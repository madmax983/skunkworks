use gray_scott::GrayScott;

#[test]
fn havoc_gray_scott_init_overflow() {
    // 👺 Havoc: Using size large enough to overflow usize during initialization
    let _ = GrayScott::new(usize::MAX, 2);
}

#[test]
#[should_panic(expected = "coordinate out of bounds")]
fn havoc_gray_scott_scanline_wrapping() {
    // 👺 Havoc: If bounds are not explicitly checked on BOTH axes before calculating
    // y * width + x, it could panic here because it was recently fixed.
    // Wait, let's look at get_index.
    let gs = GrayScott::new(10, 10);
    // If x = 15, y = 0, y*10 + 15 = 15.
    // But get_index panics if x >= width.
    // Let's call get_index(15, 0)
    let _ = gs.get_index(15, 0);
}
