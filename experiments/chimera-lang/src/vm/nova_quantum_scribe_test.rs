#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_quantum_scan_scribe_loop() {
        let genes = vec![
            // 1. Write '*' to (8,8)
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("*".to_string())] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] },
            Gene { op: OpCode::GWrite, args: vec![] },

            // 2. QuantumScan (at context_loc 8,8)
            // Weight 10 (1.0)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
            Gene { op: OpCode::QuantumScan, args: vec![] },

            // 3. Clear Grid at (8,8)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] },
            Gene { op: OpCode::GWrite, args: vec![] },

            // 4. QuantumScribe (at context_loc 8,8)
            // Threshold 5 (0.5)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
            Gene { op: OpCode::QuantumScribe, args: vec![] },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 1000;

        // Step
        // 1. Push '*'
        // 2. Push 8
        // 3. Push 8
        // 4. GWrite (Grid[8][8] = '*')
        // 5. Push 10
        // 6. Scan (H[8][8] += 1.0 * phase('*'))
        // 7. Push 0
        // 8. Push 8
        // 9. Push 8
        // 10. GWrite (Grid[8][8] = 0)
        // 11. Push 5
        // 12. Scribe (Grid[8][8] = char(phase(H[8][8])))

        for _ in 0..15 {
            vm.step();
        }

        // Check Hologram State
        let (re, im) = vm.hologram_grid[8][8];
        let mag = (re * re + im * im).sqrt();
        assert!(mag > 0.9, "Hologram magnitude should be high (was {})", mag);

        // Check Grid Restoration
        let val = &vm.grid[8][8];
        match val {
            Value::Str(s) => assert_eq!(s, "*"),
            _ => panic!("Expected string '*', got {:?}", val),
        }
    }
}
