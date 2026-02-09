#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::nova::OrganelleType;
    use crate::vm::{ChimeraVM, Value};

    fn make_empty_vm() -> ChimeraVM {
        // Create a dummy strand to prevent immediate halt
        let dummy_strand = Strand {
            genes: vec![Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(0)],
            }],
        };
        let dna = Dna {
            helix: Helix {
                strands: vec![dummy_strand],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        // Enable Nova features implicitly by the fact we are testing them
        vm.energy = 1000;
        vm
    }

    #[test]
    fn test_bang_chain_reaction() {
        let mut vm = make_empty_vm();

        // Setup Grid
        // (0,0) -> "*"
        // (0,1) -> "*"
        // (0,2) -> "void"
        vm.grid[0][0] = Value::Str("*".to_string());
        vm.grid[0][1] = Value::Str("*".to_string());
        vm.grid[0][2] = Value::Str("void".to_string());

        // Spawn initial Ribosome at (0,0) manually
        use crate::vm::nova::Organelle;
        let root = Organelle {
            stack: Vec::new(),
            ip: (0, 0),
            context_loc: (0, 0),
            call_stack: Vec::new(),
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Ribosome,
            direction: (0, 1),
            ttl: None, // Persistent root
            name: "Test Root".to_string(),
            traits: vec![],
            id: 1,
            tissue_id: None,
            genome_id: 0,
        };
        vm.organelles.push(root);

        // Step 1: Root executes "*" at (0,0).
        // Should spawn ephemeral at (0,1) (and others, but valid is (0,1)).
        vm.step();

        // Verify we have more organelles now (Root + Ephemeral)
        // Neighbors of (0,0): (-1,0) out, (1,0) valid, (0,-1) out, (0,1) valid.
        // So spawns at (1,0) and (0,1).
        assert!(
            vm.organelles.len() > 1,
            "Should have spawned ephemeral ribosomes"
        );

        // Step 2: Ephemeral at (0,1) executes "*".
        // Should spawn at (0,2) and (0,0) and (1,1).
        vm.step();

        // Step 3: Ephemeral at (0,2) executes "void".
        // Should spawn a Void organelle.
        vm.step();

        // Check for Void organelle
        let has_void = vm
            .organelles
            .iter()
            .any(|o| matches!(o.kind, OrganelleType::Void));
        assert!(
            has_void,
            "Signal should have propagated to spawn Void organelle"
        );
    }
}
