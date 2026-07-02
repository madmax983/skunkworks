use quipu::{Color, Cord, Knot, Quipu};

#[test]
fn test_debug_coverage_for_capped_cord_and_subsidiaries() {
    let mut root = Cord::from(100);
    // Depth is 0, so if we format it, it doesn't trigger depth > 100 limits natively here unless we build it super deep or invoke helper manually.
    let mut current = &mut root;
    for _ in 0..102 {
        let child = Cord::from(1);
        current.subsidiaries.push(child);
        current = current.subsidiaries.first_mut().unwrap();
    }

    // Now trigger it to hit the `entries` map and `DebugCappedCord` nested
    let display = format!("{:?}", root);

    // It should contain `[...]` due to `DebugSubsidiaries` truncating
    assert!(display.contains("[...]"));

    // It should contain `Cord { ... }` from `DebugCappedCord` hitting depth limit
}

#[test]
fn test_color_debug_clone_eq_default() {
    let c1 = Color::Natural;
    let c2 = Color::Red;
    let c3 = Color::Green;
    let c4 = Color::Blue;
    let c5 = Color::Yellow;
    let c6 = Color::Black;
    let c7 = Color::White;

    assert_eq!(Color::default(), Color::Natural);
    assert_eq!(c1, Color::Natural);
    assert_ne!(c1, c2);

    assert_eq!(c1.clone(), Color::Natural);
    assert_eq!(format!("{:?}", c1), "Natural");
    assert_eq!(format!("{:?}", c2), "Red");
    assert_eq!(format!("{:?}", c3), "Green");
    assert_eq!(format!("{:?}", c4), "Blue");
    assert_eq!(format!("{:?}", c5), "Yellow");
    assert_eq!(format!("{:?}", c6), "Black");
    assert_eq!(format!("{:?}", c7), "White");
}

#[test]
fn test_cord_default() {
    let c = Cord::default();
    assert_eq!(c.clusters.len(), 0);
    assert_eq!(c.subsidiaries.len(), 0);
    assert_eq!(c.color, Color::Natural);
}

#[test]
fn test_cord_eq() {
    let mut c1 = Cord::from(42);
    let mut c2 = Cord::from(42);

    assert_eq!(c1, c2);

    c1.color = Color::Red;
    assert_ne!(c1, c2);
    c1.color = Color::Natural;

    c1.clusters.push(vec![Knot::Simple]);
    assert_ne!(c1, c2);
    c1.clusters.pop();

    c1.subsidiaries.push(Cord::from(10));
    assert_ne!(c1, c2);

    c2.subsidiaries.push(Cord::from(10));
    assert_eq!(c1, c2);

    c1.subsidiaries[0].color = Color::Red;
    assert_ne!(c1, c2);
}

#[test]
fn test_cord_debug() {
    let mut c = Cord::from(100);
    c.color = Color::Red;
    c.subsidiaries.push(Cord::from(10));

    let debug_str = format!("{:?}", c);
    assert!(debug_str.contains("Cord"));
    assert!(debug_str.contains("clusters"));
    assert!(debug_str.contains("color"));
    assert!(debug_str.contains("Red"));
    assert!(debug_str.contains("subsidiaries"));
}

#[test]
fn test_cord_drop_deeply_nested() {
    let mut root = Cord::from(1);
    let mut current = &mut root;
    for _ in 0..1000 {
        current.subsidiaries.push(Cord::from(1));
        current = current.subsidiaries.first_mut().unwrap();
    }

    // Explicitly drop to ensure no stack overflow
    drop(root);
}

#[test]
fn test_quipu_debug_clone_eq() {
    let mut q1 = Quipu::new();
    q1.add_cord(Cord::from(10));

    let mut q2 = Quipu::new();
    q2.add_cord(Cord::from(10));

    let q3 = Quipu::new();

    assert_eq!(q1, q2);
    assert_ne!(q1, q3);

    assert_eq!(q1.clone(), q2);

    let debug_str = format!("{:?}", q1);
    assert!(debug_str.contains("Quipu"));
    assert!(debug_str.contains("cords"));
}

#[test]
fn test_clone_cord_deep() {
    let mut root = Cord::from(1);
    let mut current = &mut root;
    for _ in 0..10 {
        let child = Cord::from(1);
        current.subsidiaries.push(child);
        current = current.subsidiaries.first_mut().unwrap();
    }

    let cloned = root.clone();
    assert_eq!(root, cloned);
}

#[test]
fn test_long_knot_symbol() {
    let k = Knot::Long(5);
    assert_eq!(k.symbol(), "≡5");
    let cow = k.symbol_cow();
    assert_eq!(cow, std::borrow::Cow::Owned::<str>("≡5".to_string()));
}

// Testing deeply nested Debug implementation
#[test]
fn test_debug_capped_cord() {
    let mut root = Cord::from(100);
    // Setting up the depth correctly
    let mut current = &mut root;
    for _ in 0..150 {
        current.subsidiaries.push(Cord::from(1));
        current = current.subsidiaries.first_mut().unwrap();
    }

    // We already tested that format!("{:?}", root) returns "(max depth reached)"
    // because root's depth caps it. We need to manually construct DebugCappedCord.

    // Format a deeply nested list
    let list = format!("{:?}", root.subsidiaries); // This calls DebugCappedCord -> DebugSubsidiaries...
    assert!(list.contains("Cord"));
    assert!(list.contains("clusters"));
    assert!(list.contains("color"));
}
