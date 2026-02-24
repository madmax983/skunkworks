#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Helix, JunctionType, Strand};
    use crate::vm::nova_pocket;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let genes = vec![];
        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_pocket_unpocket() {
        let mut vm = make_vm();

        // Setup grid
        // Center at 8,8
        vm.context_loc = (8, 8);
        vm.grid[8][8] = Value::Int(42);
        vm.grid[8][9] = Value::Int(10);
        vm.grid[7][8] = Value::Int(20);

        // Pocket radius 1 (3x3 area)
        vm.stack.push(Value::Int(1));
        nova_pocket::exec_pocket(&mut vm);

        // Verify stack has pocket
        assert_eq!(vm.stack.len(), 1);
        let pocket = vm.stack.pop().unwrap();

        if let Value::Junction(JunctionType::All, items) = &pocket {
            assert_eq!(items[0], Value::Str("POCKET".to_string()));
            assert_eq!(items[1], Value::Int(1));
            // Size 3x3 = 9 + 2 header = 11
            assert_eq!(items.len(), 11);
        } else {
            panic!("Expected Pocket Junction, got {:?}", pocket);
        }

        // Verify grid cleared
        assert_eq!(vm.grid[8][8], Value::Int(0));
        assert_eq!(vm.grid[8][9], Value::Int(0));
        assert_eq!(vm.grid[7][8], Value::Int(0));

        // Unpocket
        vm.stack.push(pocket);
        nova_pocket::exec_unpocket(&mut vm);

        // Verify grid restored
        assert_eq!(vm.grid[8][8], Value::Int(42));
        assert_eq!(vm.grid[8][9], Value::Int(10));
        assert_eq!(vm.grid[7][8], Value::Int(20));
    }

    #[test]
    fn test_pocket_bounds() {
        let mut vm = make_vm();
        // Corner at 0,0
        vm.context_loc = (0, 0);
        vm.grid[0][0] = Value::Int(99);

        // Pocket radius 1 (should clip edges by filling with 0s)
        vm.stack.push(Value::Int(1));
        nova_pocket::exec_pocket(&mut vm);

        let pocket = vm.stack.pop().unwrap();
        if let Value::Junction(_, items) = &pocket {
            // Should still be 11 items
            assert_eq!(items.len(), 11);
        }

        // Grid cleared
        assert_eq!(vm.grid[0][0], Value::Int(0));

        // Unpocket at different location (5,5)
        vm.context_loc = (5, 5);
        vm.stack.push(pocket);
        nova_pocket::exec_unpocket(&mut vm);

        // Check center (offset 0,0 relative to 5,5 is 5,5)
        // Original was at 0,0.
        // Wait, pocket captures relative to context.
        // At (0,0), radius 1 captures (-1,-1) to (1,1).
        // (0,0) is at index corresponding to dy=0, dx=0.
        // The value 99 was at (0,0) relative to context (0,0).
        // So 99 is at the center of the pocket data.

        // Unpocketing at (5,5):
        // Center (5,5) gets the center data (99).
        assert_eq!(vm.grid[5][5], Value::Int(99));
    }

    #[test]
    fn test_recursive_pocket() {
        let mut vm = make_vm();
        vm.context_loc = (8, 8);
        vm.grid[8][8] = Value::Int(100);

        // Pocket radius 0 (1x1)
        vm.stack.push(Value::Int(0));
        nova_pocket::exec_pocket(&mut vm);

        let inner_pocket = vm.stack.pop().unwrap();

        // Place inner pocket on grid
        vm.grid[9][9] = inner_pocket;

        // Move to 9,9
        vm.context_loc = (9, 9);

        // Pocket radius 1 (captures 9,9 which contains inner pocket)
        vm.stack.push(Value::Int(1));
        nova_pocket::exec_pocket(&mut vm);

        let outer_pocket = vm.stack.pop().unwrap();

        // Unpocket outer at 5,5
        vm.context_loc = (5, 5);
        vm.stack.push(outer_pocket);
        nova_pocket::exec_unpocket(&mut vm);

        // 5,5 should contain inner pocket
        if let Value::Junction(JunctionType::All, items) = &vm.grid[5][5] {
            assert_eq!(items[0], Value::Str("POCKET".to_string()));
            assert_eq!(items[1], Value::Int(0)); // Inner radius 0
        } else {
            panic!("Expected inner pocket at 5,5");
        }

        // Unpocket inner at 5,5
        let inner = vm.grid[5][5].clone();
        vm.stack.push(inner);
        nova_pocket::exec_unpocket(&mut vm);

        // 5,5 should be 100
        assert_eq!(vm.grid[5][5], Value::Int(100));
    }
}
