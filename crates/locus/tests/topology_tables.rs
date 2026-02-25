use locus::Topology;

struct TestCase {
    pub topology: Topology,
    pub width: usize,
    pub height: usize,
    pub input_y: i64,
    pub input_x: i64,
    pub expected: Option<(usize, usize)>,
    pub desc: &'static str,
}

#[test]
fn test_topology_wrapping_tables() {
    let cases = vec![
        // ============================================
        // PLANE
        // ============================================
        TestCase {
            topology: Topology::Plane,
            width: 10,
            height: 10,
            input_y: 5,
            input_x: 5,
            expected: Some((5, 5)),
            desc: "Plane: Center",
        },
        TestCase {
            topology: Topology::Plane,
            width: 10,
            height: 10,
            input_y: 5,
            input_x: -1,
            expected: None,
            desc: "Plane: Left of boundary",
        },
        TestCase {
            topology: Topology::Plane,
            width: 10,
            height: 10,
            input_y: 10,
            input_x: 5,
            expected: None,
            desc: "Plane: Bottom of boundary",
        },
        // ============================================
        // TORUS
        // ============================================
        TestCase {
            topology: Topology::Torus,
            width: 10,
            height: 10,
            input_y: -1,
            input_x: -1,
            expected: Some((9, 9)),
            desc: "Torus: Top-Left wrap",
        },
        TestCase {
            topology: Topology::Torus,
            width: 10,
            height: 10,
            input_y: 10,
            input_x: 10,
            expected: Some((0, 0)),
            desc: "Torus: Bottom-Right wrap",
        },
        TestCase {
            topology: Topology::Torus,
            width: 10,
            height: 10,
            input_y: 25,
            input_x: 25,
            expected: Some((5, 5)),
            desc: "Torus: Multiple wraps",
        },
        // ============================================
        // CYLINDER H (Wraps X, Bounded Y)
        // ============================================
        TestCase {
            topology: Topology::CylinderH,
            width: 10,
            height: 10,
            input_y: 5,
            input_x: 15,
            expected: Some((5, 5)),
            desc: "CylinderH: X wrap",
        },
        TestCase {
            topology: Topology::CylinderH,
            width: 10,
            height: 10,
            input_y: -1,
            input_x: 5,
            expected: None,
            desc: "CylinderH: Y bounded (top)",
        },
        TestCase {
            topology: Topology::CylinderH,
            width: 10,
            height: 10,
            input_y: 10,
            input_x: 5,
            expected: None,
            desc: "CylinderH: Y bounded (bottom)",
        },
        // ============================================
        // CYLINDER V (Bounded X, Wraps Y)
        // ============================================
        TestCase {
            topology: Topology::CylinderV,
            width: 10,
            height: 10,
            input_y: 15,
            input_x: 5,
            expected: Some((5, 5)),
            desc: "CylinderV: Y wrap",
        },
        TestCase {
            topology: Topology::CylinderV,
            width: 10,
            height: 10,
            input_y: 5,
            input_x: -1,
            expected: None,
            desc: "CylinderV: X bounded (left)",
        },
        // ============================================
        // KLEIN BOTTLE (Wraps X, Wraps Y with X-twist)
        // ============================================
        // y wrap even -> no twist
        TestCase {
            topology: Topology::Klein,
            width: 10,
            height: 10,
            input_y: 25,
            input_x: 2, // y=25 -> wrap=2 (even). ny=5. x=2 -> 2.
            expected: Some((5, 2)),
            desc: "Klein: Y wrap even (no twist)",
        },
        // y wrap odd -> twist x
        TestCase {
            topology: Topology::Klein,
            width: 10,
            height: 10,
            input_y: 15,
            input_x: 2, // y=15 -> wrap=1 (odd). ny=5. x=2 -> twist: 9-2=7.
            expected: Some((5, 7)),
            desc: "Klein: Y wrap odd (twist X)",
        },
        // negative y wrap odd -> twist x
        TestCase {
            topology: Topology::Klein,
            width: 10,
            height: 10,
            input_y: -5,
            input_x: 2, // y=-5. -5 div 10 = -1 (odd). ny=5. x=2 -> twist: 9-2=7.
            expected: Some((5, 7)),
            desc: "Klein: Negative Y wrap odd (twist X)",
        },
        // ============================================
        // MOBIUS STRIP (Wraps X with Y-twist, Bounded Y)
        // ============================================
        // x wrap even -> no twist
        TestCase {
            topology: Topology::Mobius,
            width: 10,
            height: 10,
            input_y: 2,
            input_x: 25, // x=25 -> wrap=2 (even). nx=5. y=2 -> 2.
            expected: Some((2, 5)),
            desc: "Mobius: X wrap even (no twist)",
        },
        // x wrap odd -> twist y
        TestCase {
            topology: Topology::Mobius,
            width: 10,
            height: 10,
            input_y: 2,
            input_x: 15, // x=15 -> wrap=1 (odd). nx=5. y=2 -> twist: 9-2=7.
            expected: Some((7, 5)),
            desc: "Mobius: X wrap odd (twist Y)",
        },
        // x wrap odd -> twist y out of bounds?
        TestCase {
            topology: Topology::Mobius,
            width: 10,
            height: 10,
            input_y: 12,
            input_x: 15, // x=15 (twist). y=12. twist: 9-12 = -3. Out of bounds.
            expected: None,
            desc: "Mobius: Twist Y out of bounds",
        },
        // ============================================
        // SPHERE (Wraps X, Wraps Y with Reflection/Shift)
        // ============================================
        // y wrap even -> normal
        TestCase {
            topology: Topology::Sphere,
            width: 10,
            height: 10,
            input_y: 25,
            input_x: 2, // y=25 -> wrap=2 (even). ny=5. x=2.
            expected: Some((5, 2)),
            desc: "Sphere: Y wrap even",
        },
        // y wrap odd -> reflect y, shift x
        TestCase {
            topology: Topology::Sphere,
            width: 10,
            height: 10,
            input_y: 15,
            input_x: 2, // y=15 -> wrap=1 (odd). ny=5.
            // reflect ny: 9-5=4.
            // shift x: (2 + 5) % 10 = 7.
            expected: Some((4, 7)),
            desc: "Sphere: Y wrap odd (Reflect Y, Shift X)",
        },
        // Negative odd wrap
        TestCase {
            topology: Topology::Sphere,
            width: 10,
            height: 10,
            input_y: -5,
            input_x: 2, // y=-5. wrap=-1 (odd). ny=5.
            // reflect ny: 9-5=4.
            // shift x: 7.
            expected: Some((4, 7)),
            desc: "Sphere: Negative Y wrap odd",
        },
        // ============================================
        // PROJECTIVE PLANE (Wraps both with twist)
        // ============================================
        // x odd wrap -> twist y
        TestCase {
            topology: Topology::Projective,
            width: 10,
            height: 10,
            input_y: 2,
            input_x: 15, // x=15 (odd). nx=5. y=2 -> twist: 9-2=7.
            expected: Some((7, 5)),
            desc: "Projective: X wrap odd (Twist Y)",
        },
        // y odd wrap -> twist x
        TestCase {
            topology: Topology::Projective,
            width: 10,
            height: 10,
            input_y: 15,
            input_x: 2, // y=15 (odd). ny=5. x=2 -> twist: 9-2=7.
            expected: Some((5, 7)),
            desc: "Projective: Y wrap odd (Twist X)",
        },
        // both odd wrap -> double twist
        TestCase {
            topology: Topology::Projective,
            width: 10,
            height: 10,
            input_y: 15,
            input_x: 15, // x=15 (odd), y=15 (odd). nx=5, ny=5.
            // twist y: 9-5=4.
            // twist x: 9-5=4.
            expected: Some((4, 4)),
            desc: "Projective: Both wrap odd (Double Twist)",
        },
        // ============================================
        // EDGE CASES
        // ============================================
        TestCase {
            topology: Topology::Plane,
            width: 0,
            height: 10,
            input_y: 0,
            input_x: 0,
            expected: None,
            desc: "Zero width",
        },
        TestCase {
            topology: Topology::Plane,
            width: 10,
            height: 0,
            input_y: 0,
            input_x: 0,
            expected: None,
            desc: "Zero height",
        },
        // Corner checks
        TestCase {
            topology: Topology::Torus,
            width: 5,
            height: 5,
            input_y: 4,
            input_x: 4,
            expected: Some((4, 4)),
            desc: "Max index",
        },
        TestCase {
            topology: Topology::Torus,
            width: 5,
            height: 5,
            input_y: 5,
            input_x: 5,
            expected: Some((0, 0)),
            desc: "Just over boundary",
        },
    ];

    for case in cases {
        let res = case
            .topology
            .normalize(case.input_y, case.input_x, case.width, case.height);
        assert_eq!(
            res, case.expected,
            "Failed test case: {} | Input (y, x): ({}, {}) | Width: {}, Height: {}",
            case.desc, case.input_y, case.input_x, case.width, case.height
        );
    }
}
