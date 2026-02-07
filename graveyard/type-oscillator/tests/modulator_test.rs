use type_oscillator::modulator::Oscillator;

#[test]
fn test_modulator() {
    let mut osc = Oscillator::new();
    let points = vec![(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)];

    // Initial modulation (phase 0)
    // sin(0) = 0.
    let mod_points = osc.modulate(&points);
    // Should be close to 0 if phase is 0 and x*freq is small.
    // freq = 0.1. x=0 -> 0. x=1 -> 0.1. sin(0.1) ~= 0.099.

    assert_eq!(mod_points.len(), 3);
    assert_eq!(mod_points[0], (0.0, 0.0));

    // Update physics
    osc.update(1.0);
    assert!(osc.phase > 0.0);

    let mod_points_2 = osc.modulate(&points);
    // Should change
    assert_ne!(mod_points_2[0].1, 0.0);
}
