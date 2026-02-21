#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::prelude::*;
    use chimera_lang::vm::ChimeraVM;

    #[test]
    fn test_luciferin() {
        // [ push(255) push(0) push(0) push(100) luciferin() ]
        // Intensity 100, Red Color
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(255)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(255)],
            },
            Gene {
                op: OpCode::Luciferin,
                args: vec![],
            },
        ];
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Execute
        vm.step(); // push
        vm.step(); // push
        vm.step(); // push
        vm.step(); // push
        vm.step(); // luciferin

        // Check grid at context (8,8)
        assert!(vm.light_grid[8][8] >= 100);
        assert_eq!(vm.light_color_grid[8][8], (255, 0, 0));
    }

    #[test]
    fn test_photophore() {
        // [ push(255) push(0) push(0) push(255) luciferin() push(2) photophore() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(255)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(255)],
            },
            Gene {
                op: OpCode::Luciferin,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::Photophore,
                args: vec![],
            },
        ];
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        for _ in 0..7 {
            vm.step();
        }

        // Center should be high
        assert!(vm.light_grid[8][8] >= 100);
        // Neighbor (8,9) should be lit
        assert!(vm.light_grid[8][9] >= 50);
        // Color should propagate
        assert_eq!(vm.light_color_grid[8][9], (255, 0, 0));
    }
}
