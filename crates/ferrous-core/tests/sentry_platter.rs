use ferrous_core::Platter;

#[test]
fn test_platter_bounds_safety() {
    let mut p = Platter::new(10, 10);

    // Test out of bounds access
    assert_eq!(p.get_magnetism(10, 10), 0.0);
    assert_eq!(p.get_magnetism(100, 0), 0.0);
    assert_eq!(p.get_magnetism(0, 100), 0.0);

    // Test out of bounds writes (should not panic or affect state)
    p.magnetize(10, 10, 1.0);
    p.accumulate(100, 100, 1.0);

    // Verify internal state is clean (no unexpected writes)
    for y in 0..10 {
        for x in 0..10 {
            assert_eq!(
                p.get_magnetism(x, y),
                0.0,
                "Unexpected write at {},{}",
                x,
                y
            );
        }
    }
}

#[test]
fn test_platter_zero_size() {
    let mut p = Platter::new(0, 0);

    // Should handle 0x0 gracefully
    // Accessing public fields is fine as struct fields are pub
    assert_eq!(p.width, 0);
    assert_eq!(p.height, 0);
    assert_eq!(p.magnetism.len(), 0);

    // Accessing anything should return 0.0
    assert_eq!(p.get_magnetism(0, 0), 0.0);

    // Writing should be safe no-op
    p.magnetize(0, 0, 1.0);
    p.accumulate(0, 0, 1.0);
    p.decay(0.5);
}

#[test]
fn test_decay_threshold_edge_case() {
    let mut p = Platter::new(2, 2);

    // Set up values around the 0.001 threshold
    // Threshold check is: if *m < 0.001 { *m = 0.0; }

    // Case 1: Just above threshold after decay
    // Start at 0.002002, decay 0.5 -> 0.001001 (should stay)
    p.magnetize(0, 0, 0.002002);

    // Case 2: Exactly on threshold after decay
    // Start at 0.002, decay 0.5 -> 0.001 (should stay, since < 0.001 is strict)
    p.magnetize(1, 0, 0.002);

    // Case 3: Just below threshold after decay
    // Start at 0.001998, decay 0.5 -> 0.000999 (should become 0.0)
    p.magnetize(0, 1, 0.001998);

    p.decay(0.5);

    assert!(
        p.get_magnetism(0, 0) > 0.001,
        "Should be > 0.001, got {}",
        p.get_magnetism(0, 0)
    );
    assert!(
        (p.get_magnetism(1, 0) - 0.001).abs() < 1e-9,
        "Should be exactly 0.001"
    );
    assert_eq!(p.get_magnetism(0, 1), 0.0, "Should be clamped to 0.0");
}

#[test]
fn test_magnetize_negative_handling() {
    let mut p = Platter::new(5, 5);

    // magnetize does: (current + amount).min(1.0)
    // If we add negative amount, it reduces magnetism
    p.magnetize(0, 0, 0.5);
    p.magnetize(0, 0, -0.2);
    assert!((p.get_magnetism(0, 0) - 0.3).abs() < 1e-6);

    // If we go negative
    p.magnetize(1, 1, -0.5);
    assert!((p.get_magnetism(1, 1) + 0.5).abs() < 1e-6); // Is -0.5

    // Decay handles negatives?
    // decay: *m *= rate; if m.abs() < 0.001 { *m = 0.0; }
    // -0.5 * 0.5 = -0.25. |-0.25| = 0.25. 0.25 < 0.001 is FALSE.
    // It should stay as -0.25.
    p.decay(0.5);
    assert!((p.get_magnetism(1, 1) + 0.25).abs() < 1e-6);

    // If we decay to near zero
    p.decay(0.001); // -0.25 * 0.001 = -0.00025. Abs < 0.001. Should become 0.
    assert_eq!(p.get_magnetism(1, 1), 0.0);
}
