use quipu::{Cord, Knot};

#[test]
fn test_deep_display_stack_overflow() {
    let mut root = Cord::new();
    let mut current = &mut root;

    for _ in 0..50000 {
        let mut child = Cord::new();
        child.clusters.push(vec![Knot::Simple]);
        current.subsidiaries.push(child);
        current = current.subsidiaries.first_mut().unwrap();
    }

    let output = format!("{}", root);
    assert!(output.contains("(max depth reached)"));
}
