#![allow(missing_docs)]
use gray_scott::GrayScott;

#[test]
fn test_update_sequential_and_parallel() {
    // To ensure both paths are hit for test coverage when different features are active.
    let mut gs_seq = GrayScott::new(10, 10);
    gs_seq.add_chemical(5, 5, 1.0);
    gs_seq.update(0.055, 0.062, 1.0);
    assert!(gs_seq.u()[gs_seq.get_index(5, 5)] < 1.0);
}
