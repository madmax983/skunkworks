use proptest::prelude::*;
use quipu_renderer::quipu::{Cord, Knot};
use std::str::FromStr;

// Strategy to generate arbitrary Knots
fn arb_knot() -> impl Strategy<Value = Knot> {
    prop_oneof![
        Just(Knot::Simple),
        Just(Knot::FigureEight),
        (2u8..=9u8).prop_map(Knot::Long),
    ]
}

// Strategy to generate arbitrary Cords (random clusters of knots)
fn arb_cord() -> impl Strategy<Value = Cord> {
    prop::collection::vec(
        prop::collection::vec(arb_knot(), 0..20), // Up to 20 knots per cluster
        0..20 // Up to 20 clusters (powers of 10)
    ).prop_map(|clusters| Cord { clusters })
}

proptest! {
    // Fuzz the parser with garbage strings
    #[test]
    fn test_cord_from_str_no_panic(s in "\\PC*") {
        // We don't expect it to succeed, just not to panic.
        let _ = Cord::from_str(&s);
    }

    // Fuzz the addition logic with random structures
    #[test]
    fn test_cord_add_arbitrary(c1 in arb_cord(), c2 in arb_cord()) {
        let _ = c1.add(&c2);
    }

    // Fuzz addition with verified math (checking for correctness too, why not?)
    #[test]
    fn test_cord_math_correctness(v1 in any::<u32>(), v2 in any::<u32>()) {
        let c1 = Cord::from(v1);
        let c2 = Cord::from(v2);
        let sum_cord = c1.add(&c2);

        // Calculate expected sum with u64 to avoid overflow
        let expected = (v1 as u64) + (v2 as u64);

        // Cord::value() returns u32. If expected > u32::MAX, value() will truncate/overflow?
        // Let's check.
        // If the Cord supports arbitrary precision, value() returning u32 is lossy.
        // We only assert if expected fits in u32.

        if expected <= u32::MAX as u64 {
             prop_assert_eq!(sum_cord.value(), expected as u32);
        }
    }
}
