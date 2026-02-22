#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix, Strand};
    use crate::vm::nova_chaos::{check_local_transmutation, AlchemyRecipe};
    use crate::vm::{ChimeraVM, Value};

    fn create_vm() -> ChimeraVM {
        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.chaos_cartridge.active = true;
        vm
    }

    #[test]
    fn test_simple_transmutation() {
        let mut vm = create_vm();

        // Define Recipe: Fire + Water -> Steam (Prob 1.0)
        let recipe = AlchemyRecipe {
            inputs: vec!["Fire".to_string(), "Water".to_string()],
            output: "Steam".to_string(),
            probability: 1.0,
        };
        vm.chaos_cartridge.recipes.push(recipe);

        // Place ingredients around (5, 5)
        // N=(4,5), S=(6,5), E=(5,6), W=(5,4)
        vm.grid[4][5] = Value::Str("Fire".to_string());
        vm.grid[5][6] = Value::Str("Water".to_string());

        // Check (5,5)
        let result = check_local_transmutation(&vm, 5, 5);
        assert!(result.is_some());
        let (output, consumed) = result.unwrap();
        assert_eq!(output, "Steam");
        assert_eq!(consumed.len(), 2);
        assert!(consumed.contains(&(4, 5)));
        assert!(consumed.contains(&(5, 6)));
    }

    #[test]
    fn test_salt_preservation() {
        let mut vm = create_vm();

        let recipe = AlchemyRecipe {
            inputs: vec!["A".to_string(), "B".to_string()],
            output: "Gold".to_string(),
            probability: 1.0,
        };
        vm.chaos_cartridge.recipes.push(recipe);

        vm.grid[4][5] = Value::Str("A".to_string());
        vm.grid[5][6] = Value::Str("B".to_string());
        vm.grid[6][5] = Value::Str("Salt".to_string()); // Catalyst S

        let result = check_local_transmutation(&vm, 5, 5);
        assert!(result.is_some());
        let (output, consumed) = result.unwrap();
        assert_eq!(output, "Gold");
        // Salt prevents consumption
        assert!(consumed.is_empty());
    }

    #[test]
    fn test_sulfur_catalysis() {
        let mut vm = create_vm();

        // Low probability
        let recipe = AlchemyRecipe {
            inputs: vec!["A".to_string()],
            output: "Boom".to_string(),
            probability: 0.0, // Should never happen normally
        };
        vm.chaos_cartridge.recipes.push(recipe);

        vm.grid[4][5] = Value::Str("A".to_string());
        vm.grid[5][6] = Value::Str("Sulfur".to_string()); // Catalyst &

        // Try multiple times to ensure it's not random luck (though prob is 0.0)
        let result = check_local_transmutation(&vm, 5, 5);
        assert!(result.is_some());
        let (output, _) = result.unwrap();
        assert_eq!(output, "Boom");
    }

    #[test]
    fn test_symbol_catalysts() {
        let mut vm = create_vm();

        let recipe = AlchemyRecipe {
            inputs: vec!["X".to_string()],
            output: "Y".to_string(),
            probability: 1.0,
        };
        vm.chaos_cartridge.recipes.push(recipe);

        vm.grid[4][5] = Value::Str("X".to_string());
        vm.grid[5][6] = Value::Str("$".to_string()); // Salt symbol

        let result = check_local_transmutation(&vm, 5, 5);
        assert!(result.is_some());
        let (_, consumed) = result.unwrap();
        assert!(consumed.is_empty()); // Preserved
    }
}
