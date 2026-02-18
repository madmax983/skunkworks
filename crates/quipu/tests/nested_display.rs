use quipu::{Color, Cord};

#[test]
fn test_nested_display() {
    let mut main_cord = Cord::from(100);
    main_cord.color = Color::Red;

    let mut sub_cord = Cord::from(10);
    sub_cord.color = Color::Blue;

    let sub_sub_cord = Cord::from(1);
    // sub_sub_cord color is Natural

    sub_cord.subsidiaries.push(sub_sub_cord);
    main_cord.subsidiaries.push(sub_cord);

    let display = format!("{}", main_cord);
    println!("Display:\n{}", display);

    // Check for color labels
    assert!(display.contains("[Red]"));
    assert!(display.contains("[Blue]"));

    // Check for knots
    assert!(display.contains("●")); // Simple knot
    assert!(display.contains("∞")); // Figure eight

    // We can't easily check indentation without knowing exact string, but visual check via println helps if we run with --nocapture
}
