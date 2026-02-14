use locus::Topology;
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_topology_normalize_no_panic(
        x in i64::MIN..=i64::MAX,
        y in i64::MIN..=i64::MAX,
        width in 1usize..=10_000usize,
        height in 1usize..=10_000usize,
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
