#[cfg(all(feature = "nova", feature = "oracle"))]
mod tests {
    use chimera_lang::vm::{ChimeraVM, Value};
    use chimera_lang::ast::{Dna, Helix, Strand, Gene, JunctionType};
    use chimera_lang::opcode::OpCode;

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_logos_reaction() {
        let mut vm = make_vm();
        vm.logos_mode = true;

        // Setup:
        // Grid[0][0] = 1 (Agent)
        // Grid[0][1] = 2 (Reagent)
        // Rule: reaction(1, 2, 3).

        vm.grid[0][0] = Value::Int(1);
        vm.grid[0][1] = Value::Int(2);

        // Add rule: reaction(1, 2, 3)
        // Fact: any("reaction", 1, 2, 3)
        let rule = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("reaction".to_string()),
                Value::Int(1),
                Value::Int(2),
                Value::Int(3)
            ]
        );
        vm.knowledge_base.push(rule);

        // Run one tick
        vm.step();

        // Expect:
        // Grid[0][0] = 3 (Product)
        // Grid[0][1] = 0 (Consumed)
        assert_eq!(vm.grid[0][0], Value::Int(3), "Agent should transform into Product");
        assert_eq!(vm.grid[0][1], Value::Int(0), "Reagent should be consumed");
    }

    #[test]
    fn test_logos_opcode_toggle() {
        let genes = vec![
            Gene { op: OpCode::Logos, args: vec![] },
            Gene { op: OpCode::Logos, args: vec![] },
        ];
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Initial state
        assert!(!vm.logos_mode);

        // Step 1: Logos toggle ON
        vm.step();
        assert!(vm.logos_mode);

        // Step 2: Logos toggle OFF
        vm.step();
        assert!(!vm.logos_mode);
    }
}
