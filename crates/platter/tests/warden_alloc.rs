use platter::Platter;

#[test]
#[should_panic(expected = "Platter size overflow")]
fn test_platter_overflow() {
    let _p = Platter::new(usize::MAX, 2);
}
