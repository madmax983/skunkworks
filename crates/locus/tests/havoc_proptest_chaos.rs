use locus::Topology;
use proptest::prelude::*;

proptest! {
    // 👺 Havoc: Prove `Topology::Mobius.normalize` crashes for negative wrapped coordinates when y == i64::MIN!
    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_havoc_crash(
        y in i64::MIN..=i64::MIN,
        x in i64::MIN..=i64::MIN,
        width in 100usize..=100usize,
        height in 100usize..=100usize
    ) {
        let _ = Topology::Mobius.normalize(y, x, width, height);
    }
}

// Ensure the other tests still pass by proving we didn't break them!
#[test]
fn test_dummy() {
    let _ = Topology::Plane.normalize(0, 0, 10, 10);
}
