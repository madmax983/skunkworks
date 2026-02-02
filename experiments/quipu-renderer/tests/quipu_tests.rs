use quipu_renderer::quipu::{Cord, Knot};
use std::str::FromStr;

#[test]
fn test_conversion() {
    let val = 12345;
    let cord = Cord::from(val);
    assert_eq!(cord.value(), val);
}

#[test]
fn test_structure_units() {
    // 1 -> FigureEight
    let c1 = Cord::from(1);
    assert_eq!(c1.clusters[0], vec![Knot::FigureEight]);

    // 5 -> Long(5)
    let c5 = Cord::from(5);
    assert_eq!(c5.clusters[0], vec![Knot::Long(5)]);
}

#[test]
fn test_structure_tens() {
    // 20 -> 2 Simple at index 1
    let c20 = Cord::from(20);
    if c20.clusters.len() > 1 {
        assert_eq!(c20.clusters[1], vec![Knot::Simple, Knot::Simple]);
    } else {
        panic!("Expected clusters for tens");
    }
    // Units should be empty for 0
    assert!(c20.clusters[0].is_empty());
}

#[test]
fn test_addition() {
    let c1 = Cord::from(15);
    let c2 = Cord::from(17);
    let sum = c1.add(&c2);

    assert_eq!(sum.value(), 32);

    // Check structure of 32: 3 tens, 2 units (Long(2))
    assert_eq!(sum.clusters[1].len(), 3); // 3 Simple
    assert_eq!(sum.clusters[0], vec![Knot::Long(2)]);
}

#[test]
fn test_ascii_serialization_roundtrip() {
    let val = 1205; // 1 s, 2 s, 0 (empty), 5 L
    let c = Cord::from(val);
    let ascii = c.to_string();

    // Debug print
    println!("ASCII for {}:\n{}", val, ascii);

    let parsed = Cord::from_str(&ascii).expect("Failed to parse");
    assert_eq!(parsed.value(), val);

    // Ensure structure is preserved
    assert_eq!(parsed.clusters.len(), c.clusters.len());
}

#[test]
fn test_ascii_serialization_simple() {
    let val = 12; // 1 s, 2 L
    let c = Cord::from(val);
    let ascii = c.to_string();

    assert!(ascii.contains("s"));
    assert!(ascii.contains("|"));
    assert!(ascii.contains("L2"));

    let parsed = Cord::from_str(&ascii).expect("Failed to parse");
    assert_eq!(parsed.value(), val);
}
