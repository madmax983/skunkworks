use locus::{Topology, Vec2};
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_vec2_normalize_unit(
        x in -1e10..1e10,
        y in -1e10..1e10,
    ) {
        let v = Vec2::new(x, y);
        let n = v.normalize();
        let mag = n.magnitude();

        // If v is zero, n is zero.
        if v.magnitude_squared() == 0.0 {
             prop_assert_eq!(n.x, 0.0);
             prop_assert_eq!(n.y, 0.0);
        } else {
             // Otherwise magnitude should be 1.0 (approx)
             prop_assert!((mag - 1.0).abs() < 1e-6, "Magnitude not 1.0: {}", mag);
        }
    }

    #[test]
    fn test_vec2_limit(
        x in -1e10..1e10,
        y in -1e10..1e10,
        max in 0.0..1e10
    ) {
        let v = Vec2::new(x, y);
        let l = v.limit(max);
        let l_mag = l.magnitude();

        if v.magnitude() > max {
             prop_assert!((l_mag - max).abs() < 1e-5, "Limit failed: expected {}, got {}", max, l_mag);
        } else {
             prop_assert_eq!(l, v);
        }
    }
}

proptest! {
    #[test]
    fn test_topology_plane(
        x in i64::MIN..i64::MAX,
        y in i64::MIN..i64::MAX,
        w in 1usize..1000,
        h in 1usize..1000
    ) {
        let topo = Topology::Plane;
        let res = topo.normalize(y, x, w, h);

        if x >= 0 && x < w as i64 && y >= 0 && y < h as i64 {
            prop_assert_eq!(res, Some((y as usize, x as usize)));
        } else {
            prop_assert_eq!(res, None);
        }
    }

    #[test]
    fn test_topology_torus(
        x in -10000i64..10000,
        y in -10000i64..10000,
        w in 1usize..1000,
        h in 1usize..1000
    ) {
        let topo = Topology::Torus;
        let res = topo.normalize(y, x, w, h);

        prop_assert!(res.is_some());
        let (ny, nx) = res.unwrap();

        // Torus wrapping check
        let expected_x = x.rem_euclid(w as i64) as usize;
        let expected_y = y.rem_euclid(h as i64) as usize;

        prop_assert_eq!(nx, expected_x);
        prop_assert_eq!(ny, expected_y);
    }

    #[test]
    fn test_topology_cylinder_h(
        x in -10000i64..10000,
        y in -10000i64..10000,
        w in 1usize..1000,
        h in 1usize..1000
    ) {
        let topo = Topology::CylinderH;
        let res = topo.normalize(y, x, w, h);

        if y >= 0 && y < h as i64 {
            prop_assert!(res.is_some());
            let (ny, nx) = res.unwrap();

            // X wraps, Y bounded
            let expected_x = x.rem_euclid(w as i64) as usize;
            prop_assert_eq!(nx, expected_x);
            prop_assert_eq!(ny, y as usize);
        } else {
            prop_assert_eq!(res, None);
        }
    }

    #[test]
    fn test_topology_cylinder_v(
        x in -10000i64..10000,
        y in -10000i64..10000,
        w in 1usize..1000,
        h in 1usize..1000
    ) {
        let topo = Topology::CylinderV;
        let res = topo.normalize(y, x, w, h);

        if x >= 0 && x < w as i64 {
            prop_assert!(res.is_some());
            let (ny, nx) = res.unwrap();

            // Y wraps, X bounded
            let expected_y = y.rem_euclid(h as i64) as usize;
            prop_assert_eq!(ny, expected_y);
            prop_assert_eq!(nx, x as usize);
        } else {
            prop_assert_eq!(res, None);
        }
    }

    #[test]
    fn test_topology_klein_invariants(
        x in -10000i64..10000,
        y in -10000i64..10000,
        w in 1usize..1000,
        h in 1usize..1000
    ) {
        let topo = Topology::Klein;
        let res = topo.normalize(y, x, w, h);

        // Klein is closed, so always returns Some
        prop_assert!(res.is_some());
        let (ny, nx) = res.unwrap();

        // Check range
        prop_assert!(nx < w);
        prop_assert!(ny < h);
    }
}
