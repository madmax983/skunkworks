#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.context_loc = (8, 8); // Start in middle
        vm
    }

    #[test]
    fn test_plant_growth() {
        // [ push("X=F+F") push("X") plant() ]
        // Use X as growth tip to avoid infinite recursion on F
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("X=F+F".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("X".to_string())],
            },
            Gene {
                op: OpCode::Plant,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(3)], // Infinite loop to keep VM alive
            },
        ];

        let mut vm = make_vm(genes);

        // Run setup
        for _ in 0..3 {
            vm.step();
        }

        // Verify Seed spawned
        assert_eq!(vm.organelles.len(), 1);
        match &vm.organelles[0].kind {
            crate::vm::nova::OrganelleType::Seed => (),
            _ => panic!("Expected Seed organelle"),
        }

        // Verify initial grid is empty (Seed hasn't drawn yet maybe?)
        // Or maybe Plant doesn't draw immediately.

        // Run simulation for growth
        // F -> F+F -> ...
        // Seed logic:
        // Tick 1: Process F (draws #, moves)
        // Tick 2: Process = (expand?) or logic handles expansion differently
        // My implementation:
        // check_rule("F", "F=F+F") -> "F+F"
        // String becomes "F+F". Index stays 0.
        // Tick 2: Process "F" (draw #, move)
        // Tick 3: Process "+" (turn)
        // Tick 4: Process "F" (draw #, move)

        for _ in 0..20 {
            vm.step();
        }

        // Check grid for drawings
        // We expect some '#'
        let mut drawn_cells = 0;
        for r in 0..16 {
            for c in 0..16 {
                if let Value::Str(s) = &vm.grid[r][c] {
                    if s == "#" {
                        drawn_cells += 1;
                    }
                }
            }
        }

        assert!(drawn_cells > 0, "Plant should have grown");
        println!("Plant grew {} cells", drawn_cells);
    }
}
