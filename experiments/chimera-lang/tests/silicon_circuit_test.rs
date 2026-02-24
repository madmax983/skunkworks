#[cfg(feature = "silicon")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_dna(strands: Vec<Strand>) -> Dna {
        Dna {
            evolution_config: None,
            helix: Helix { strands },
        }
    }

    #[test]
    fn test_circuit_interrupt() {
        // Strand 0: Setup Emitter and Receiver
        let setup_genes = vec![
            // Emitter at 0,0 (Freq 1)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)], // Freq
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // Y
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // X
            },
            Gene {
                op: OpCode::Emitter,
                args: vec![],
            },
            // Receiver at 0,2 (Target Strand 1)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)], // Strand Idx
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // Y
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)], // X
            },
            Gene {
                op: OpCode::Receiver,
                args: vec![],
            },
            // Infinite Loop to keep VM alive
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(0)],
            },
        ];

        // Strand 1: Payload
        let payload_genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(999)],
            },
            Gene {
                op: OpCode::Print,
                args: vec![],
            },
        ];

        let dna = make_dna(vec![
            Strand { genes: setup_genes },
            Strand {
                genes: payload_genes,
            },
        ]);

        let mut vm = ChimeraVM::new(dna);
        vm.silicon_mode = true; // Enable circuit simulation

        // Step 1: Execute setup code (Emitter/Receiver placement)
        // We need enough steps to execute the setup genes
        for _ in 0..10 {
            vm.step();
        }

        // Verify Grid Placement
        match &vm.grid[0][0] {
            Value::Str(s) => assert!(s.starts_with("EMIT:1:"), "Emitter not placed"),
            _ => panic!("Emitter missing"),
        }
        match &vm.grid[0][2] {
            Value::Str(s) => assert!(s.starts_with("RECV:1"), "Receiver not placed"),
            _ => panic!("Receiver missing"),
        }

        // Manually place a wire at 0,1 to connect them
        vm.grid[0][1] = Value::Int(1);

        // Step 2: Run simulation
        // Emitter freq 1 means it fires every tick (phase 0->0).
        // T0: Emitter fires (Head). Wire sees Head.
        // T1: Wire becomes Head. Receiver sees Head.
        // T2: Receiver triggers interrupt. Strand 1 executes.

        let mut success = false;
        for _ in 0..20 {
            vm.step();
            if vm.output.iter().any(|s| s == "999") {
                success = true;
                break;
            }
        }

        assert!(
            success,
            "Circuit failed to trigger payload. Output: {:?}",
            vm.output
        );
    }
}
