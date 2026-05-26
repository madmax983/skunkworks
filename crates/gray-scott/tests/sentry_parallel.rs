use gray_scott::GrayScott;

#[test]
fn test_gray_scott_parallel_zero_width_does_not_panic() {
    let mut gs = GrayScott::new(0, 10);
    // Explicitly call update, which delegates to update_parallel because of the feature flag
    gs.update(0.1, 0.1, 1.0);
}
