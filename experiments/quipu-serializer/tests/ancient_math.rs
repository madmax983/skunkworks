use quipu_serializer::quipu::{Cord, Knot};

#[test]
fn test_addition() {
    let c1 = Cord::from(5);
    let c2 = Cord::from(6);
    let c3 = c1 + c2;
    assert_eq!(c3.value(), 11);

    // Check internal representation
    // 11 should be 1 Simple at index 1, 1 FigureEight at index 0.
    assert_eq!(c3.clusters.len(), 2);
    assert_eq!(c3.clusters[1].len(), 1); // Tens
    assert_eq!(c3.clusters[1][0], Knot::Simple);
    assert_eq!(c3.clusters[0].len(), 1); // Units
    assert_eq!(c3.clusters[0][0], Knot::FigureEight);
}

#[test]
fn test_subtraction() {
    let c1 = Cord::from(10);
    let c2 = Cord::from(3);
    let c3 = c1 - c2;
    assert_eq!(c3.value(), 7);

    // 7 should be Long(7) at index 0
    assert_eq!(c3.clusters.len(), 1);
    assert_eq!(c3.clusters[0].len(), 1);
    match c3.clusters[0][0] {
        Knot::Long(v) => assert_eq!(v, 7),
        _ => panic!("Expected Long knot"),
    }
}
