#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Helix, JunctionType, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_chemistry_mix_brew_splash() {
        let mut vm = make_vm();

        // Setup Grid
        // Center: 8,8
        vm.context_loc = (8, 8);

        // Place Ingredients around center
        // (8, 9) = Fire
        vm.grid[8][9] = Value::Str("Fire".to_string());
        // (9, 8) = Water
        vm.grid[9][8] = Value::Str("Water".to_string());

        // 1. MIX
        // Stack: [ 1 ] (Radius)
        vm.stack.push(Value::Int(1));
        vm.execute_gene_inner(OpCode::Mix, &[]);

        // Check Mix Result
        let center = vm.grid[8][8].clone();
        if let Value::Junction(JunctionType::Dish, ingredients) = center {
            let names: Vec<String> = ingredients.iter().map(|v| v.to_string()).collect();
            assert!(names.iter().any(|s| s.contains("Fire")));
            assert!(names.iter().any(|s| s.contains("Water")));
        } else {
            panic!("Mix failed: Expected Dish, got {:?}", center);
        }

        // Check neighbors cleared
        assert_eq!(vm.grid[8][9], Value::Int(0));
        assert_eq!(vm.grid[9][8], Value::Int(0));

        // 2. BREW
        // Stack: [ 20 ] (Heat)
        vm.stack.push(Value::Int(20));
        vm.execute_gene_inner(OpCode::Brew, &[]);

        // Check Brew Result -> Acid
        let center = vm.grid[8][8].clone();
        if let Value::Junction(JunctionType::Dish, args) = center {
            assert_eq!(args[0], Value::Str("Solution".to_string()));
            assert_eq!(args[1], Value::Str("Acid".to_string()));
            // Potency: 20 (base) + 10 (recipe) = 30
            assert_eq!(args[2], Value::Int(30));
        } else {
            panic!("Brew failed: Expected Solution, got {:?}", center);
        }

        // 3. SPLASH
        // Target: (8, 10) (dx=2, dy=0 relative to 8,8)
        // Set up target to be destroyed
        vm.grid[8][10] = Value::Str("Wall".to_string());

        // Stack: [ 1, 0, 2 ] -> radius=1, dy=0, dx=2
        vm.stack.push(Value::Int(2)); // dx
        vm.stack.push(Value::Int(0)); // dy
        vm.stack.push(Value::Int(1)); // radius

        vm.execute_gene_inner(OpCode::Splash, &[]);

        // Check Splash Result (Acid destroys)
        assert_eq!(vm.grid[8][10], Value::Int(0));

        // Check Source Empty (Potion consumed)
        assert_eq!(vm.grid[8][8], Value::Int(0));
    }
}
