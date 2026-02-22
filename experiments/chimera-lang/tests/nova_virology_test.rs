#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::memetics;
    use chimera_lang::vm::nova::Organelle;
    use chimera_lang::vm::nova::OrganelleType;
    use chimera_lang::vm::ChimeraVM;

    fn make_vm() -> ChimeraVM {
        let genes = vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }];
        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_transduction() {
        let mut vm = make_vm();

        // 1. Define Payload Strand (Index 1)
        let payload_genes = vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(999)],
        }];
        vm.dna.helix.strands.push(Strand {
            genes: payload_genes,
        });
        let payload_idx = 1;

        // 2. Setup Organelle at (5, 5) using Genome 0
        let organelle = Organelle {
            stack: Vec::new(),
            ip: (0, 0),
            context_loc: (5, 5),
            call_stack: Vec::new(),
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Ribosome,
            direction: (0, 1),
            genome_id: 0,
            energy: 10,
            experience: 0,
            stage: 00,
            name: "TestOrg".to_string(),
            traits: Vec::new(),
            ttl: None,
            id: 0,
            tissue_id: None,
        };
        vm.organelles.push(organelle);

        // 3. Infect (5, 5) with Virus carrying Payload
        let virus = memetics::Virus {
            name: "GeneTherapy".to_string(),
            color: (0, 255, 0),
            pattern: "X".to_string(),
            mutation_rate: 0,
            payload: Some(payload_idx),
            grammar: None,
            quorum_action: None,
            quorum_threshold: 0,
            mode: memetics::VirusMode::Overwrite,
        };
        vm.virus_library.push(virus);

        vm.viral_grid[5][5] = Some(memetics::ViralState {
            infection_level: 100,
            virus_id: 0,
        });

        // 4. Run Outbreak
        memetics::exec_memetics_op(&mut vm, OpCode::Outbreak, &[]);

        // 5. Verify Transduction
        let strand_0 = &vm.dna.helix.strands[0];
        assert!(strand_0.genes.len() > 1, "Genome 0 should have grown");
        let last_gene = strand_0.genes.last().unwrap();

        if let OpCode::Push = last_gene.op {
            if let Nucleotide::Number(n) = last_gene.args[0] {
                assert_eq!(n, 999, "Payload gene should be injected");
            } else {
                panic!("Wrong arg type");
            }
        } else {
            panic!("Wrong OpCode");
        }
    }
}
