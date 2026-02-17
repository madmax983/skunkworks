use soroban::Column;

#[test]
fn test_set_value_clamped() {
    let mut c = Column::default();
    // Should NOT panic, but clamp to 9
    c.set_value(10);
    assert_eq!(c.value(), 9);

    c.set_value(255);
    assert_eq!(c.value(), 9);

    c.set_value(5);
    assert_eq!(c.value(), 5);
}
