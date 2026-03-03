use platter::Platter;

#[test]
#[should_panic(expected = "Platter size overflow")]
fn test_platter_overflow() {
    let _ = Platter::new(usize::MAX, 2);
}
