#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Helix, Strand, Gene, Nucleotide};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value, microscope};
    use crate::vm::nova::{OrganelleType, Organelle};

    fn make_empty_dna() -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        }
    }

    #[test]
    fn test_microscope_scan() {
        let mut vm = ChimeraVM::new(make_empty_dna());

        // Setup environment at (5, 5)
        let y = 5;
        let x = 5;

        // 1. Set Grid Value
        vm.grid[y][x] = Value::Int(42);

        // 2. Set Hormones
        vm.hormone_grid[y][x] = [10, 20, 30];

        // 3. Set Waste
        vm.waste_grid[y][x] = 50;

        // 4. Set Mutagen
        vm.mutagen_grid[y][x] = 100;

        // 5. Add an Organelle
        let organelle = Organelle {
            stack: vec![Value::Int(99)],
            ip: (0, 0),
            context_loc: (y, x),
            call_stack: vec![],
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Chloroplast,
            direction: (0, 0),
            ttl: None,
        };
        vm.organelles.push(organelle);

        // Scan
        let data = microscope::scan(&vm, y, x);

        // Assertions
        assert_eq!(data.coords, (y, x), "Coords match");
        assert_eq!(data.value, Value::Int(42), "Grid value matches");
        assert_eq!(data.hormone_levels, [10, 20, 30], "Hormones match");
        assert_eq!(data.waste_level, 50, "Waste matches");
        assert_eq!(data.mutagen_level, 100, "Mutagen matches");

        assert_eq!(data.organelles.len(), 1, "Organelle found");
        assert_eq!(data.organelles[0].kind, "Chloroplast", "Organelle kind matches");
        assert_eq!(data.organelles[0].stack_depth, 1, "Stack depth matches");
    }
}
