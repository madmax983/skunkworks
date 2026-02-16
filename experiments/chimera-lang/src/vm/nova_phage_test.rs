#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::nova::OrganelleType;
    use crate::vm::nova_signals::process_signals;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_phage_spawn_and_move() {
        let mut vm = make_vm();
        // Create a dummy strand for the phage to carry
        let strand = Strand {
            genes: vec![Gene {
                op: OpCode::Nop,
                args: vec![],
            }],
        };
        vm.dna.helix.strands.push(strand);

        // Spawn Phage (Type 11) at (1,1) using Spawn OpCode logic simulation
        // Or manually inject for test
        vm.organelle_id_counter += 1;
        let organelle = crate::vm::nova::Organelle {
            stack: Vec::new(),
            ip: (0, 0),
            context_loc: (1, 1),
            call_stack: Vec::new(),
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Phage,
            direction: (0, 1), // East
            ttl: None,
            name: "TestPhage".to_string(),
            traits: vec![],
            id: vm.organelle_id_counter,
            tissue_id: None,
            genome_id: 0, energy: 100, experience: 0, stage: 0,
        };
        vm.organelles.push(organelle);

        // Grid is empty, so it should move East to (1,2)
        process_signals(&mut vm);

        assert_eq!(vm.organelles[0].context_loc, (1, 2));
    }

    #[test]
    fn test_phage_mutation_on_bang() {
        let mut vm = make_vm();
        // Strand with push(10)
        let strand = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }],
        };
        vm.dna.helix.strands.push(strand);

        // Phage at (1,1) moving East (0,1)
        vm.organelles.push(crate::vm::nova::Organelle {
            stack: Vec::new(),
            ip: (0, 0),
            context_loc: (1, 1),
            call_stack: Vec::new(),
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Phage,
            direction: (0, 1),
            ttl: None,
            name: "MutantPhage".to_string(),
            traits: vec![],
            id: 1,
            tissue_id: None,
            genome_id: 0, energy: 100, experience: 0, stage: 0,
        });

        // Place Bang '*' at (1,2)
        vm.grid[1][2] = Value::Str("*".to_string());

        process_signals(&mut vm);

        // Should move to (1,2)
        assert_eq!(vm.organelles[0].context_loc, (1, 2));

        // Should have mutated. Since gene has args, it mutates args.
        // We can't predict exact value (RNG), but we can check output log or if value changed.
        // Since RNG is used, let's just check if "MUTATION: ..." is in output.
        // process_signals appends to output.

        let mutation_msg = vm.output.iter().any(|s| s.contains("MUTATION"));
        assert!(mutation_msg, "Phage should trigger mutation on Bang");
    }

    #[test]
    fn test_phage_infection_on_host() {
        let mut vm = make_vm();
        // Strand 0
        let strand = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(99)],
            }],
        };
        vm.dna.helix.strands.push(strand);

        // Phage at (1,1) moving East
        vm.organelles.push(crate::vm::nova::Organelle {
            stack: Vec::new(),
            ip: (0, 0), // Carrying Strand 0
            context_loc: (1, 1),
            call_stack: Vec::new(),
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Phage,
            direction: (0, 1),
            ttl: None,
            name: "ViralPhage".to_string(),
            traits: vec![],
            id: 1,
            tissue_id: None,
            genome_id: 0, energy: 100, experience: 0, stage: 0,
        });

        // Place Host 'H' at (1,2)
        vm.grid[1][2] = Value::Str("H".to_string());

        process_signals(&mut vm);

        // Should move to (1,2)
        assert_eq!(vm.organelles[0].context_loc, (1, 2));

        // Should have cloned strand 0 to strand 1
        assert_eq!(vm.dna.helix.strands.len(), 2);

        let new_strand = &vm.dna.helix.strands[1];
        assert_eq!(new_strand.genes.len(), 1);
        if let Nucleotide::Number(n) = &new_strand.genes[0].args[0] {
            assert_eq!(n, 99);
        } else {
            panic!("Unexpected gene arg");
        }

        let infection_msg = vm.output.iter().any(|s| s.contains("PHAGE: Injected"));
        assert!(infection_msg, "Phage should trigger infection on Host");
    }

    #[test]
    fn test_phage_bounce_wall() {
        let mut vm = make_vm();
        let strand = Strand { genes: vec![] };
        vm.dna.helix.strands.push(strand);

        // Phage at (1,1) moving East
        vm.organelles.push(crate::vm::nova::Organelle {
            stack: Vec::new(),
            ip: (0, 0),
            context_loc: (1, 1),
            call_stack: Vec::new(),
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Phage,
            direction: (0, 1),
            ttl: None,
            name: "BouncyPhage".to_string(),
            traits: vec![],
            id: 1,
            tissue_id: None,
            genome_id: 0, energy: 100, experience: 0, stage: 0,
        });

        // Place Wall '#' at (1,2)
        vm.grid[1][2] = Value::Str("#".to_string());

        process_signals(&mut vm);

        // Should STAY at (1,1) but direction reversed to (0, -1)
        assert_eq!(vm.organelles[0].context_loc, (1, 1));
        assert_eq!(vm.organelles[0].direction, (0, -1));
    }
}
