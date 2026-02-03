use quipu_renderer::quipu::{Cord, Knot};

#[test]
fn test_ancient_addition() {
    // 123 + 45 = 168
    let c1 = Cord::from(123);
    let c2 = Cord::from(45);

    let sum = c1.add(&c2);

    assert_eq!(sum.value(), 168);

    // Verify structural correctness
    // 168:
    // Hundreds: 1 Simple (Index 2)
    // Tens: 6 Simple (Index 1)
    // Units: Long(8) (Index 0)

    let clusters = &sum.clusters;
    assert!(clusters.len() >= 3);

    // Check Hundreds (Index 2)
    assert_eq!(clusters[2].len(), 1, "Should have 1 knot in hundreds");
    assert!(matches!(clusters[2][0], Knot::Simple));

    // Check Tens (Index 1)
    assert_eq!(clusters[1].len(), 6, "Should have 6 knots in tens");
    for k in &clusters[1] {
        assert!(matches!(k, Knot::Simple));
    }

    // Check Units (Index 0)
    assert_eq!(clusters[0].len(), 1, "Should have 1 knot in units");
    if let Knot::Long(v) = clusters[0][0] {
        assert_eq!(v, 8);
    } else {
        panic!("Expected Long(8) in units, found {:?}", clusters[0][0]);
    }
}

#[test]
fn test_carry_over() {
    // 9 + 1 = 10
    let c1 = Cord::from(9);
    let c2 = Cord::from(1);

    let sum = c1.add(&c2);

    assert_eq!(sum.value(), 10);

    let clusters = &sum.clusters;
    // Units should be empty (0)
    assert!(clusters[0].is_empty());
    // Tens should be 1 Simple (Index 1)
    assert_eq!(clusters[1].len(), 1);
    assert!(matches!(clusters[1][0], Knot::Simple));
}

#[test]
fn test_multiple_carry() {
    // 99 + 1 = 100
    let c1 = Cord::from(99);
    let c2 = Cord::from(1);

    let sum = c1.add(&c2);
    assert_eq!(sum.value(), 100);

    let clusters = &sum.clusters;
    assert!(clusters[0].is_empty());
    assert!(clusters[1].is_empty());
    assert_eq!(clusters[2].len(), 1);
}
