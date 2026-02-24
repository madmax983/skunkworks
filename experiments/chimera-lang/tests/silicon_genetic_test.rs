#[cfg(all(feature = "silicon", feature = "nova"))]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_trace_and_fabricate() {
        // 1. Setup VM
        // Strand 0: [ Trace(5, 5) ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)], // x
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)], // y
            },
            Gene {
                op: OpCode::Trace,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // 2. Build a circuit manually at (5, 5)
        // Simple line: (5,5) Wire -> (5,6) Wire -> (5,7) Emitter
        vm.grid[5][5] = Value::Int(1); // Wire
        vm.grid[5][6] = Value::Int(1); // Wire
        vm.grid[5][7] = Value::Str("EMIT:4:0".to_string()); // Emitter

        // 3. Execute Trace
        // Stack: [5, 5] -> Trace -> [strand_idx]
        vm.step(); // Push 5
        vm.step(); // Push 5
        vm.step(); // Trace

        // 4. Verify Trace Result
        let result = vm.stack.pop();
        assert!(result.is_some(), "Trace should return strand index");
        let strand_idx = match result.unwrap() {
            Value::Int(i) => i as usize,
            _ => panic!("Expected Int index"),
        };

        // Check if strand exists
        assert!(
            strand_idx < vm.dna.helix.strands.len(),
            "New strand should be added"
        );
        let new_strand = &vm.dna.helix.strands[strand_idx];
        println!("Generated Strand: {:?}", new_strand);

        // Strand should contain Wire, Migrate, Emitter ops
        let has_wire = new_strand.genes.iter().any(|g| g.op == OpCode::Wire);
        let has_emit = new_strand.genes.iter().any(|g| g.op == OpCode::Emitter);
        let has_migrate = new_strand.genes.iter().any(|g| g.op == OpCode::Migrate);

        assert!(has_wire, "Should have Wire op");
        assert!(has_emit, "Should have Emitter op");
        assert!(has_migrate, "Should have Migrate op");

        // 5. Clear Grid
        vm.grid = vec![vec![Value::Int(0); 16]; 16];
        assert_eq!(vm.grid[5][5], Value::Int(0));
        assert_eq!(vm.grid[5][7], Value::Int(0));

        // 6. Fabricate
        // Use a new strand to call Fabricate on the generated strand
        // Strand 2: [ Fabricate(strand_idx, 5, 5) ]
        // Stack order for Fabricate: [ ..., strand_idx, y, x ] (top)
        let fab_genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(strand_idx as i64)], // strand_idx
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)], // y
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)], // x
            },
            Gene {
                op: OpCode::Fabricate,
                args: vec![],
            },
        ];
        vm.dna.helix.strands.push(Strand { genes: fab_genes });
        let fab_strand_idx = vm.dna.helix.strands.len() - 1;

        // Jump to fabrication strand
        vm.ip = (fab_strand_idx, 0);
        vm.step(); // Push x
        vm.step(); // Push y
        vm.step(); // Push idx
        vm.step(); // Fabricate

        // 7. Verify Reconstruction
        // Note: Trace logic uses DFS, so order might vary, but topology should be preserved if path is valid.
        // Fabricate uses a turtle at (5,5).
        // (5,5) should be Wire
        assert_eq!(
            vm.grid[5][5],
            Value::Int(1),
            "Reconstructed (5,5) should be Wire"
        );

        // Check neighbors for Wire and Emitter
        // The exact position depends on the order of Migrate.
        // But since we started at 5,5 and the original was connected, it should be connected.
        // Original: (5,5)-(5,6)-(5,7).
        // Trace DFS should find this path.
        // (5,6) Wire
        assert_eq!(
            vm.grid[5][6],
            Value::Int(1),
            "Reconstructed (5,6) should be Wire"
        );
        // (5,7) Emitter
        match &vm.grid[5][7] {
            Value::Str(s) => assert!(
                s.starts_with("EMIT"),
                "Reconstructed (5,7) should be Emitter"
            ),
            _ => panic!("Expected Emitter at (5,7)"),
        }
    }
}
