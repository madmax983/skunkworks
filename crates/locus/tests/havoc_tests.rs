use locus::Topology;
use locus::Vec4;
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

proptest! {
    #[test]
    fn test_mobius_specifically(
        x in i64::MIN..=i64::MAX,
        y in i64::MIN..=i64::MAX,
        width in 1usize..=10_000usize,
        height in 1usize..=10_000usize
    ) {
        // Specifically hammer Mobius to ensure edge cases are covered
        let _ = Topology::Mobius.normalize(y, x, width, height);
    }
}

proptest! {
    // 👺 Havoc: Prove `length_squared` and `distance_squared` can overflow!
    // 🔒 Warden: Fixed.
    #[test]
    fn test_havoc_length_squared_overflow(
        x in (f32::MAX / 2.0)..=f32::MAX,
        y in (f32::MAX / 2.0)..=f32::MAX,
        z in (f32::MAX / 2.0)..=f32::MAX,
        w in (f32::MAX / 2.0)..=f32::MAX,
    ) {
        let v = Vec4::new(x, y, z, w);
        // This will result in an infinite f32, which might not strictly panic unless we assert!
        let sq = v.length_squared();
        assert!(sq.is_finite(), "👺 Havoc: Vector length squared overflowed into Infinity!");
    }
}
