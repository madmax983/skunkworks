use nile_scheduler::EgyptianFraction;
use num_rational::Ratio;

#[test]
fn test_greedy_conversion_simple() {
    // 3/4 = 1/2 + 1/4
    let val = Ratio::new(3, 4);
    let egypt = EgyptianFraction::from(val);
    assert_eq!(egypt.parts, vec![2, 4]);
}

#[test]
fn test_greedy_conversion_complex() {
    // 3/7 = 1/3 + 1/11 + 1/231
    let val = Ratio::new(3, 7);
    let egypt = EgyptianFraction::from(val);
    assert_eq!(egypt.parts, vec![3, 11, 231]);
}

#[test]
fn test_addition() {
    // 1/2 + 1/3 = 5/6 = 1/2 + 1/3
    let a = EgyptianFraction { parts: vec![2] };
    let b = EgyptianFraction { parts: vec![3] };
    let sum = a + b;
    assert_eq!(sum.parts, vec![2, 3]);
}

#[test]
fn test_subtraction() {
    // 1/2 - 1/3 = 1/6
    let a = EgyptianFraction { parts: vec![2] };
    let b = EgyptianFraction { parts: vec![3] };
    let diff = a - b;
    assert_eq!(diff.parts, vec![6]);
}

#[test]
fn test_display() {
    let ef = EgyptianFraction { parts: vec![2, 4] };
    let s = format!("{}", ef);
    // Expect hieroglyphic-like output or at least readable
    assert!(s.contains("1/2"));
    assert!(s.contains("1/4"));
}
