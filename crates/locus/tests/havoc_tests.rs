use locus::Topology;
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_topology_normalize_no_panic(
        x in i64::MIN..=i64::MAX,
        y in i64::MIN..=i64::MAX,
        width in 1usize..=usize::MAX,
        height in 1usize..=usize::MAX,
        topo_idx in 0usize..7usize
    ) {
        let topo = match topo_idx {
            0 => Topology::Plane,
            1 => Topology::Torus,
            2 => Topology::CylinderH,
            3 => Topology::CylinderV,
            4 => Topology::Klein,
            5 => Topology::Mobius,
            6 => Topology::Hyperbolic,
            _ => unreachable!(),
        };

        // This should not panic
        let _ = topo.normalize(y, x, width, height);
    }
}

#[test]
fn test_topology_normalize_panic_repro() {
    let topo = Topology::Torus;
    let width = usize::MAX;
    let height = 10;
    let x = i64::MIN;
    let y = 0;

    // This panicked before fix
    let _ = topo.normalize(y, x, width, height);
}
