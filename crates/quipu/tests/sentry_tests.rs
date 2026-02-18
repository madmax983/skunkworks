use quipu::Cord;

#[test]
#[should_panic(expected = "Quipu subtraction resulted in negative value")]
fn test_sub_panic() {
    let c1 = Cord::from(50);
    let c2 = Cord::from(100);
    // This should panic
    let _ = c1 - c2;
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
