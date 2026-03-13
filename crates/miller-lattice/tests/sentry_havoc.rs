#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn havoc_test() {
    let h: u64 = std::hint::black_box(u64::MAX);
    let i = std::hint::black_box(1);
    let _idx = (h as usize) + i;
}
