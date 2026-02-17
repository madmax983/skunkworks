use quipu::Cord;

#[test]
fn test_cord_multiplier_overflow() {
    let mut c = Cord::new();
    // 25 empty clusters
    // multiplier will saturate to u64::MAX eventually
    for _ in 0..25 {
        c.clusters.push(vec![]);
    }

    // With empty clusters, value contributes 0, so total should be 0.
    // Even if multiplier is saturated.
    assert_eq!(c.value(), 0);

    // Now add a knot at the end (highest power)
    // This knot will be multiplied by the saturated multiplier (u64::MAX)
    c.clusters.push(vec![quipu::Knot::Simple]);

    // The value should now be saturated at u64::MAX
    assert_eq!(c.value(), u64::MAX);
}
