#![allow(missing_docs)]
use gray_scott::GrayScott;

#[test]
fn test_getters_and_muts() {
    let mut gs = GrayScott::new(10, 10);
    assert_eq!(gs.width(), 10);
    assert_eq!(gs.height(), 10);
    assert_eq!(gs.u().len(), 100);
    assert_eq!(gs.v().len(), 100);
    assert_eq!(gs.u_mut().len(), 100);
    assert_eq!(gs.v_mut().len(), 100);
    assert_eq!(gs.diff_u, 0.16);
    assert_eq!(gs.diff_v, 0.08);
}
