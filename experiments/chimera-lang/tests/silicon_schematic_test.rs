#[cfg(feature = "silicon")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Helix, Strand, Gene, Nucleotide};
    use chimera_lang::vm::{ChimeraVM, Value};
    use chimera_lang::opcode::OpCode;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_latch_creation() {
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // State
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] }, // Y
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] }, // X
            Gene { op: OpCode::Latch, args: vec![] },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.silicon_mode = true;

        // Execute until halted
        for _ in 0..10 {
            if vm.halted { break; }
            vm.step();
        }

        assert_eq!(vm.grid[5][5], Value::Str("LATCH:1".to_string()));
    }

    #[test]
    fn test_adc_op() {
        // [ push(8) adc() ] -> Should turn North neighbor into Head if it's a wire
        // Neighbors: N=(7,8) bit 8
        let genes = vec![
            // Place wire at 7,8
            // Stack order for Wire: [y, x]
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(7)] }, // Y
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] }, // X
            Gene { op: OpCode::Wire, args: vec![] },

            // Push 8 (North bit)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] },
            Gene { op: OpCode::ADC, args: vec![] },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.context_loc = (8, 8);
        // Disable auto-simulation to prevent wire decay before verification
        // ADC runs at end of step 5.
        vm.silicon_mode = false;

        for _ in 0..10 {
            if vm.halted { break; }
            vm.step();
        }

        assert_eq!(vm.grid[7][8], Value::Int(2)); // Should be Head
    }

    #[test]
    fn test_dac_op() {
        // Place Head at North (7,8). Execute DAC at 8,8. Should push 8.
        let genes = vec![
            // Stack order for Pulse: [y, x]
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(7)] }, // Y
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] }, // X
            Gene { op: OpCode::Pulse, args: vec![] }, // Head

            Gene { op: OpCode::DAC, args: vec![] },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.context_loc = (8, 8);
        // Disable silicon mode so Pulse doesn't decay to Tail before DAC executes
        vm.silicon_mode = false;

        for _ in 0..10 {
            if vm.halted { break; }
            vm.step();
        }

        assert_eq!(vm.stack.last(), Some(&Value::Int(8)));
    }
}
