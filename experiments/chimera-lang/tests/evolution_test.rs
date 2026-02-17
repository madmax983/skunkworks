#[cfg(feature = "nova")]
#[cfg(test)]
mod evolution_tests {
    use chimera_lang::ast::{Dna, Helix, JunctionType, Nucleotide};
    use chimera_lang::vm::nova::{Organelle, OrganelleType};
    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    #[ignore]
    fn test_savant_die() {
        let mut vm = make_vm();
        vm.knowledge_base.clear();

        let mut savant = Organelle {
            stack: Vec::new(),
            ip: (0, 0),
            context_loc: (5, 5),
            call_stack: Vec::new(),
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Savant,
            direction: (0, 0),
            ttl: None,
            name: "Suicidal".to_string(),
            traits: vec![],
            id: 1,
            tissue_id: None,
            genome_id: 0,
            energy: 100,
            experience: 0,
            stage: 0,
        };
        vm.organelles.push(savant);

        // action("die")
        let fact = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("action".to_string()),
                Value::Str("die".to_string()),
            ],
        );
        vm.knowledge_base.push(fact);

        vm.step();

        // Should be dead and removed
        assert!(vm.organelles.is_empty(), "Savant should have died");
    }

    #[test]
    #[ignore]
    fn test_savant_orca_write() {
        let mut vm = make_vm();
        vm.knowledge_base.clear();

        let mut savant = Organelle {
            stack: Vec::new(),
            ip: (0, 0),
            context_loc: (5, 5),
            call_stack: Vec::new(),
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Savant,
            direction: (0, 0),
            ttl: None,
            name: "Architect".to_string(),
            traits: vec![],
            id: 1,
            tissue_id: None,
            genome_id: 0,
            energy: 100,
            experience: 0,
            stage: 0,
        };
        vm.organelles.push(savant);

        let action = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("orca_write".to_string()),
                Value::Int(6),
                Value::Int(6),
                Value::Str("Test".to_string()),
            ],
        );

        let fact = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("action".to_string()), action],
        );

        vm.knowledge_base.push(fact);

        vm.step();

        println!("Output: {:?}", vm.output);
        assert_eq!(vm.grid[6][6], Value::Str("Test".to_string()));
    }

    #[test]
    #[ignore]
    fn test_savant_circuit_place() {
        let mut vm = make_vm();
        vm.knowledge_base.clear();

        let mut savant = Organelle {
            stack: Vec::new(),
            ip: (0, 0),
            context_loc: (5, 5),
            call_stack: Vec::new(),
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Savant,
            direction: (0, 0),
            ttl: None,
            name: "Engineer".to_string(),
            traits: vec![],
            id: 1,
            tissue_id: None,
            genome_id: 0,
            energy: 100,
            experience: 0,
            stage: 0,
        };
        vm.organelles.push(savant);

        // action(circuit_place("wire", 7, 7))
        let action = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("circuit_place".to_string()),
                Value::Str("wire".to_string()),
                Value::Int(7),
                Value::Int(7),
            ],
        );
        let fact = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("action".to_string()), action],
        );
        vm.knowledge_base.push(fact);

        vm.step();

        println!("Output: {:?}", vm.output);
        assert_eq!(vm.grid[7][7], Value::Int(1)); // Wire is 1
    }

    #[test]
    fn test_orca_babel_trigger() {
        let mut vm = make_vm();
        vm.grid[5][5] = Value::Str("Input".to_string());
        vm.grid[6][5] = Value::Str("B".to_string());
        vm.signal_grid[6][5] = 1;
        vm.grid[6][6] = Value::Str(".".to_string());
        vm.step();
        assert!(
            !vm.stack.is_empty(),
            "Stack should have result from BabelLive"
        );
    }
}
