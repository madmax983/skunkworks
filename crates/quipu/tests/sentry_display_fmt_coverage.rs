use quipu::{Color, Cord, Knot, Quipu};

#[test]
fn test_display_formatting_uncovered_paths() {
    let mut q = Quipu::new();

    // Testing colored cord to cover Color != Natural
    let mut c1 = Cord::new();
    c1.color = Color::Red;
    // Testing multiple knots in one cluster with spacing
    c1.clusters.push(vec![Knot::Simple, Knot::Long(3)]);

    // Testing multiple subsidiaries to cover the subsidiary separator
    c1.subsidiaries.push(Cord::from(10));
    c1.subsidiaries.push(Cord::from(20));

    q.add_cord(c1);

    let display = format!("{}", q);

    // Ensure all variants output properly
    assert!(display.contains("Quipu with 1 cords"));
    assert!(display.contains("Cord 0:"));
    assert!(display.contains("[Red]"));
    assert!(display.contains("● ≡3"));
}
