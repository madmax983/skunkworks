#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_sow_and_evolve() {
        let mut vm = make_vm();

        // 1. Sow a rule for Species 2: B3/S23 (Life)
        vm.stack.push(Value::Str("B3/S23".to_string()));
        vm.stack.push(Value::Int(2));
        crate::vm::nova_garden::exec_sow(&mut vm);

        // Check if rule is registered
        assert!(vm.garden.rules.contains_key(&2));

        // 2. Plant a Blinker (Species 2)
        // Center at 8,8
        vm.grid[8][8] = Value::Int(2);
        vm.grid[8][9] = Value::Int(2);
        vm.grid[8][10] = Value::Int(2);

        // 3. Evolve (Step 1) -> Should become vertical
        crate::vm::nova_garden::exec_evolve(&mut vm);

        assert_eq!(vm.grid[7][9], Value::Int(2)); // Born
        assert_eq!(vm.grid[8][9], Value::Int(2)); // Survive
        assert_eq!(vm.grid[9][9], Value::Int(2)); // Born
        assert_eq!(vm.grid[8][8], Value::Int(0)); // Died
        assert_eq!(vm.grid[8][10], Value::Int(0)); // Died

        // 4. Evolve (Step 2) -> Should become horizontal again
        crate::vm::nova_garden::exec_evolve(&mut vm);

        assert_eq!(vm.grid[8][8], Value::Int(2));
        assert_eq!(vm.grid[8][9], Value::Int(2));
        assert_eq!(vm.grid[8][10], Value::Int(2));
    }

    #[test]
    fn test_harvest() {
        let mut vm = make_vm();
        vm.grid[5][5] = Value::Int(1);
        vm.grid[5][6] = Value::Int(2);
        vm.context_loc = (5, 5);

        vm.stack.push(Value::Int(1)); // Radius 1
        crate::vm::nova_garden::exec_harvest(&mut vm);

        let res = vm.stack.pop().unwrap();
        if let Value::Str(s) = res {
            // Radius 1 around 5,5 -> 5 cells (center + 4 neighbors)
            // (5,5), (5,6), (6,5), (5,4), (4,5) (order depends on get_circular_coords)
            assert!(s.contains("1,"));
            assert!(s.contains("2,"));
        } else {
            panic!("Expected string");
        }
    }
}
