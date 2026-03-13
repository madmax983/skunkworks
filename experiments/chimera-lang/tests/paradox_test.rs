#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Helix};
    use chimera_lang::vm::paradox::{Action, Paradox, Trigger};
    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_empty_vm() -> ChimeraVM {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_paradox_rule_parsing() {
        let mut paradox = Paradox::new();
        let rule_str = "rule Test triggers always do log Hello";
        assert!(paradox.parse_rule(rule_str).is_ok());

        let rule = &paradox.rules[0];
        assert_eq!(rule.name, "Test");
        match &rule.trigger {
            Trigger::Always => {}
            _ => panic!("Expected Trigger::Always"),
        }
        match &rule.actions[0] {
            Action::Log(msg) => assert_eq!(msg, "Hello"),
            _ => panic!("Expected Action::Log"),
        }
    }

    #[test]
    fn test_paradox_execution() {
        let mut vm = make_empty_vm();

        // Add a rule to set grid value
        let rule_str = "rule Setter triggers always do set 0 0 42";
        vm.paradox.parse_rule(rule_str).unwrap();

        // Run tick
        vm.step();

        // Check grid at context_loc (8,8)
        let (cy, cx) = vm.context_loc;
        match vm.grid[cy][cx] {
            Value::Int(42) => {}
            _ => panic!("Expected 42 at context_loc"),
        }
    }
}
