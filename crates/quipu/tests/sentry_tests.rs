use quipu::Cord;

#[test]
fn test_checked_sub_none() {
    let c1 = Cord::from(50);
    let c2 = Cord::from(100);
    let result = c1.checked_sub(&c2);
    assert!(result.is_none());
}

#[test]
fn test_zero_roundtrip() {
    let cord = Cord::from(0);
    assert_eq!(cord.value(), 0);
    assert!(cord.clusters.is_empty());

    // Test default also
    let cord_default = Cord::default();
    assert_eq!(cord_default.value(), 0);
    assert!(cord_default.clusters.is_empty());
}

#[test]
fn test_max_u64_roundtrip() {
    let max_val = u64::MAX;
    let cord = Cord::from(max_val);
    assert_eq!(cord.value(), max_val);
}

// Simple LCG for deterministic pseudo-random numbers
struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        // Linear Congruential Generator parameters (from numerical recipes)
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }
}

#[test]
fn test_fuzz_roundtrip() {
    let mut lcg = Lcg::new(12345);

    for _ in 0..1000 {
        let val = lcg.next();
        let cord = Cord::from(val);
        assert_eq!(cord.value(), val, "Failed roundtrip for value: {}", val);
    }
}

#[test]
fn test_manual_construction_overflow() {
    // 22 clusters means 10^21 multiplier will be reached
    // 10^19 fits in u64, 10^20 overflows.
    // Cord::value() updates multiplier: multiplier *= 10.
    // With saturating arithmetic, this should NOT panic, but saturate.
    // Since clusters are empty, value is 0.
    let clusters = vec![Vec::new(); 22];
    let cord = Cord {
        clusters,
        subsidiaries: Vec::new(),
        color: Default::default(),
    };
    assert_eq!(cord.value(), 0);
}

#[test]
fn test_display_formatting() {
    // 101:
    // Units (0): 1 -> FigureEight
    // Tens (1): 0 -> Empty
    // Hundreds (2): 1 -> Simple
    let cord = Cord::from(101);
    let display = format!("{}", cord);

    // Expected output:
    // ●          (Hundreds)
    //   |        (Tens - empty)
    // ∞          (Units)

    let expected = "●\n  |  \n∞";
    assert_eq!(display, expected);
}

#[test]
fn test_multiple_subsidiaries_display() {
    let mut main_cord = Cord::from(100);

    let sub_cord1 = Cord::from(10);
    let sub_cord2 = Cord::from(1);

    main_cord.subsidiaries.push(sub_cord1);
    main_cord.subsidiaries.push(sub_cord2);

    let display = format!("{}", main_cord);

    // Check that we have the main cord and both subsidiaries formatted
    assert!(display.contains("●")); // Main cord 100
    assert!(display.contains("∞")); // Subsidiary 1
}

// Ensure we test equality directly without relying on stack implicitly through custom asserts
#[test]
fn should_not_stack_overflow_on_deep_equality() {
    let mut cord1 = Cord::new();
    let mut current1 = &mut cord1;
    // Lower bound so it does not overflow stack but is deep enough to test
    for _ in 0..100_000 {
        current1.subsidiaries.push(Cord::new());
        current1 = &mut current1.subsidiaries[0];
    }

    let mut cord2 = Cord::new();
    let mut current2 = &mut cord2;
    for _ in 0..100_000 {
        current2.subsidiaries.push(Cord::new());
        current2 = &mut current2.subsidiaries[0];
    }

    assert!(cord1 == cord2);
}
