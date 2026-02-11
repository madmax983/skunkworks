#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::memetics::ViralState;
    use chimera_lang::vm::{ChimeraVM, Value};

    #[test]
    fn test_viral_injection() {
        // 1. Setup VM with genes to capture
        // We want to capture [Push 999].
        // But we also need to push args for Infect.
        // Stack args: [ ..., mutation_rate, pattern_str, name_str ]
        let setup_genes = vec![
            // Gene 0: Push 999 (The payload we want to verify)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(999)],
            },
            // Gene 1: Push Rate 0
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            // Gene 2: Push Pattern "X"
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("X".to_string())],
            },
            // Gene 3: Push Name "Retro"
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("Retro".to_string())],
            },
            // Gene 4: Infect (Captures genes 0-4)
            Gene {
                op: OpCode::Infect,
                args: vec![],
            },
        ];

        let strand = Strand { genes: setup_genes };
        let dna = Dna {
            helix: Helix {
                strands: vec![strand],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Run 5 steps to execute Infect
        for _ in 0..5 {
            vm.step();
        }

        // Verify Virus Created
        assert_eq!(vm.virus_library.len(), 1, "Virus not created");
        let virus = &vm.virus_library[0];
        assert_eq!(virus.name, "Retro");
        assert_eq!(virus.genes.len(), 5, "Payload size incorrect");
        assert_eq!(virus.genes[0].op, OpCode::Push); // Check first gene is Push 999

        // 3. Set High Infection manually at current location
        let (cy, cx) = vm.context_loc;
        vm.viral_grid[cy][cx] = Some(ViralState {
            infection_level: 100, // Critical
            virus_id: 0,
        });

        // 4. Trigger Outbreak
        // Append Outbreak gene
        let outbreak_gene = Gene {
            op: OpCode::Outbreak,
            args: vec![],
        };
        vm.dna.helix.strands[0].genes.push(outbreak_gene);

        // Run 1 step (Outbreak)
        // Current IP should be at index 5 (next after Infect)
        // But we just pushed Outbreak at index 5.
        // So step() should execute it.
        vm.step();

        // 5. Verify Injection
        // Strand length: 5 (initial) + 1 (Outbreak) + 5 (Injected) = 11
        let final_genes = &vm.dna.helix.strands[0].genes;
        println!("Final genes: {:?}", final_genes);
        assert_eq!(
            final_genes.len(),
            11,
            "Expected 11 genes, got {}",
            final_genes.len()
        );

        // The injected genes should be inserted at IP (index 5).
        // So index 5 should be Push 999 (start of payload).
        // Wait, inject_genes inserts AT vm.ip.1.
        // We executed Outbreak at index 5.
        // After execution, inject_genes inserts at 5.
        // So indices 5,6,7,8,9 should be the payload.
        // Index 10 should be the Outbreak gene (pushed down?).
        // Or if inject inserts BEFORE, current op stays at 5 but points to new gene?
        // insert(i, val) shifts elements at i to i+1.
        // So if we are executing 5, and insert at 5...
        // The old 5 (Outbreak) becomes 10 (after 5 inserts).
        // The new 5 is Push 999.

        assert_eq!(
            final_genes[5].op,
            OpCode::Push,
            "Expected injected payload at index 5"
        );
        if let Nucleotide::Number(n) = &final_genes[5].args[0] {
            assert_eq!(*n, 999, "Expected Push 999");
        } else {
            panic!("Expected Number arg");
        }
    }
}
