#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Helix, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Topology, Value};

    fn make_empty_vm() -> ChimeraVM {
        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_topology_switching() {
        let mut vm = make_empty_vm();
        assert_eq!(vm.topology, Topology::Torus); // Default

        // Switch to Plane (0)
        vm.stack.push(Value::Int(0));
        let _ = vm.execute_gene_inner(OpCode::Shape, &[]);
        assert_eq!(vm.topology, Topology::Plane);

        // Switch to Klein (4)
        vm.stack.push(Value::Int(4));
        let _ = vm.execute_gene_inner(OpCode::Shape, &[]);
        assert_eq!(vm.topology, Topology::Klein);
    }

    #[test]
    fn test_normalize_coords_torus() {
        let mut vm = make_empty_vm();
        vm.topology = Topology::Torus;
        assert_eq!(vm.normalize_coords(16, 16), Some((0, 0)));
        assert_eq!(vm.normalize_coords(-1, -1), Some((15, 15)));
    }

    #[test]
    fn test_normalize_coords_plane() {
        let mut vm = make_empty_vm();
        vm.topology = Topology::Plane;
        assert_eq!(vm.normalize_coords(16, 16), None);
        assert_eq!(vm.normalize_coords(15, 15), Some((15, 15)));
        assert_eq!(vm.normalize_coords(-1, 0), None);
    }

    #[test]
    fn test_normalize_coords_cylinder_h() {
        let mut vm = make_empty_vm();
        vm.topology = Topology::CylinderH;
        assert_eq!(vm.normalize_coords(0, 16), Some((0, 0))); // Wrap X
        assert_eq!(vm.normalize_coords(16, 0), None); // Bound Y
    }

    #[test]
    fn test_normalize_coords_klein() {
        let mut vm = make_empty_vm();
        vm.topology = Topology::Klein;
        // y = 16 (wrap once) -> y'=0. x=0 -> x'=15-0=15.
        assert_eq!(vm.normalize_coords(16, 0), Some((0, 15)));
        // y = -1 (wrap once backwards) -> y'=15. x=0 -> x'=15-0=15.
        assert_eq!(vm.normalize_coords(-1, 0), Some((15, 15)));
        // x wraps normally
        assert_eq!(vm.normalize_coords(0, 16), Some((0, 0)));
    }

    #[test]
    fn test_normalize_coords_mobius() {
        let mut vm = make_empty_vm();
        vm.topology = Topology::Mobius;
        // Wrap X (twist Y)
        // x = 16 -> x'=0. y=0 -> y'=15-0=15.
        assert_eq!(vm.normalize_coords(0, 16), Some((15, 0)));

        // Bounded Y
        assert_eq!(vm.normalize_coords(16, 0), None);
    }
}
