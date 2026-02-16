#[cfg(feature = "nova")]
mod memetic_integration_test {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand, JunctionType};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_biohack_workflow() {
        // 1. Setup: DNA that creates a Meme, defines a Grammar, and BioHacks a Virus.
        // Strand 0:
        //   [
        //     push(10) push(10) add()  // Code to be meme-ified (3 genes)
        //     push(100) push(100) push(3) conceive() // Meme 0: Len 3, Vir 100, Fid 100
        //
        //     // Grammar: Match "A"
        //     push("A") grammar("Match")
        //
        //     // BioHack
        //     push(0) // Meme ID
        //     // (Grammar is on stack)
        //     push("TestVirus")
        //     bio_hack()
        //   ]

        let genes = vec![
            // Conceive Meme (Captures next 3 genes)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(3)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] },
            Gene { op: OpCode::Conceive, args: vec![] },

            // Meme Payload (3 genes)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
            Gene { op: OpCode::Add, args: vec![] },

            // Push Name (Bottom)
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("TestVirus".to_string())] },

            // Create Grammar (Middle)
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("A".to_string())] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("Match".to_string())] },
            Gene { op: OpCode::Grammar, args: vec![] },

            // Push Meme ID (Top)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },

            // BioHack: Pops [Meme, Grammar, Name]
            Gene { op: OpCode::BioHack, args: vec![] },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));

        // Run until done
        for _ in 0..20 {
            vm.step();
        }

        // Verify Meme Creation
        assert_eq!(vm.meme_pool.memes.len(), 1, "Meme should be created");
        let meme = &vm.meme_pool.memes[0];
        assert_eq!(meme.genes.len(), 3, "Meme should have 3 genes");

        // Verify Virus Creation
        if vm.virus_library.len() != 1 {
            println!("VM Output: {:?}", vm.output);
        }
        assert_eq!(vm.virus_library.len(), 1, "Virus should be created");
        let virus = &vm.virus_library[0];
        assert_eq!(virus.name, "TestVirus");
        assert!(virus.payload.is_some(), "Virus should have payload");
        assert!(virus.grammar.is_some(), "Virus should have grammar");

        // Verify Infection
        let (cy, cx) = vm.context_loc;
        let viral_state = &vm.viral_grid[cy][cx];
        assert!(viral_state.is_some(), "Local cell should be infected");
        assert_eq!(viral_state.unwrap().virus_id, 0);
    }
}
