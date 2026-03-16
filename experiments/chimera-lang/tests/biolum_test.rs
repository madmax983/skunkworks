#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::prelude::*;
    use chimera_lang::vm::ChimeraVM;

    #[test]
    fn test_luciferin() {
        // [ push(255) push(0) push(0) push(100) luciferin() ]
        // Stack pushes in reverse order compared to what's popped
        // We pop intensity, b, g, r
        // So we push r, g, b, intensity
        // Wait, stack is LIFO. So to pop intensity first, it must be pushed last.
        // Therefore pushing r, g, b, intensity means intensity is at top, b is below it, g below, r at bottom.
        // The genes list below pushes: 255, 0, 0, 100
        // Stack becomes: [255, 0, 0, 100] (top is 100)
        // Intensity 100, b = 0, g = 0, r = 255
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
                args: vec![Nucleotide::Number(100)],
            },
            Gene {
                op: OpCode::Luciferin,
                args: vec![],
            },
        ];
        let dna = Dna {
            evolution_config: None,
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
                args: vec![Nucleotide::Number(255)], // r
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // g
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // b
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(255)], // intensity
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
            evolution_config: None,
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
