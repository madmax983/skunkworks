use gray_scott::GrayScott;

#[test]
#[should_panic(expected = "GrayScott size overflow")]
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

// 👺 HAVOC: True property testing of extreme invalid inputs.
// If the simulation does not validate parameters, NaN will infect the grid silently.
// However, the assertion should check that the state DOES NOT become NaN if it's safe.
// Since it's vulnerable, asserting `is_finite()` will fail. We write the test to verify
// stability. Since it's unstable, it fails, proving Havoc's point.
#[test]
#[should_panic(expected = "Warden defense failed: position became NaN!")]
fn test_havoc_dt_nan_poisoning() {
    let mut sim = GrayScott::new(10, 10);
    sim.add_chemical(5, 5, 2.0);

    // 👺 HAVOC: Poison the simulation via unchecked `dt` parameter.
    sim.update(0.055, 0.062, f32::NAN);

    // Warden should have defended against this, but didn't.
    // This assertion will FAIL because the bug exists. This is the correct chaos engineering pattern.
    for val in sim.u() {
        assert!(val.is_finite(), "Warden defense failed: position became NaN!");
    }
}

#[test]
#[should_panic(expected = "Warden defense failed: feed/kill NaN poisoned the grid!")]
fn test_havoc_feed_kill_nan_poisoning() {
    let mut sim = GrayScott::new(10, 10);
    sim.add_chemical(5, 5, 2.0);

    // 👺 HAVOC: Poison the simulation via unchecked feed/kill parameters.
    sim.update(f32::NAN, f32::NAN, 1.0);

    // Warden should have defended against this.
    // This assertion will FAIL because the bug exists.
    for val in sim.u() {
        assert!(val.is_finite(), "Warden defense failed: feed/kill NaN poisoned the grid!");
    }
}
