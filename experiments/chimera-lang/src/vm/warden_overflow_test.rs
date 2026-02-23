#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix, JunctionType};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn setup_vm() -> ChimeraVM {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.context_loc = (8, 8);
        vm
    }

    #[test]
    fn test_brew_overflow() {
        let mut vm = setup_vm();

        // Setup ingredients for Acid: Water + Fire
        let ingredients = vec![
            Value::Str("Water".to_string()),
            Value::Str("Fire".to_string()),
        ];
        vm.grid[8][8] = Value::Junction(JunctionType::Dish, ingredients);

        // Heat = i64::MAX - 5
        // Acid recipe adds +10 to potency (heat)
        // (MAX - 5) + 10 = Overflow -> Saturates to MAX
        vm.stack.push(Value::Int(i64::MAX - 5));

        crate::vm::nova_chemistry::exec_brew(&mut vm);

        // Verify result is saturated, not wrapped
        if let Value::Junction(JunctionType::Dish, args) = &vm.grid[8][8] {
            assert_eq!(args[0], Value::Str("Solution".to_string()));
            assert_eq!(args[1], Value::Str("Acid".to_string()));
            assert_eq!(args[2], Value::Int(i64::MAX)); // Saturated
        } else {
            panic!("Expected Solution Dish");
        }
    }
}
