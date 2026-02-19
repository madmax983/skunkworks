#[cfg(test)]
mod tests {
    use chimera_lang::compiler::compile;
    use chimera_lang::vm::{ChimeraVM, Value};

    #[test]
    fn test_grid_map_compilation() {
        // Corrected: 'circuit' is an identifier, not a string
        let src = r#"
        map circuit {
            ! ~ ?
        }
        strand main {
            apply_map("circuit", 5, 5)
        }
        "#;
        let dna = compile(src, None).expect("Compilation failed");
        let mut vm = ChimeraVM::new(dna);

        // Execute the 'main' strand which contains the map application logic
        // Step until finished
        for _ in 0..20 {
            vm.step();
            if vm.halted || vm.ip.0 >= vm.dna.helix.strands.len() {
                break;
            }
        }

        // Verify grid state
        assert_eq!(vm.grid[5][5], Value::Str("!".to_string()), "Expected '!' at (5,5)");
        assert_eq!(vm.grid[5][7], Value::Str("~".to_string()), "Expected '~' at (7,5)");
        assert_eq!(vm.grid[5][9], Value::Str("?".to_string()), "Expected '?' at (9,5)");
    }

    #[test]
    fn test_multiline_map() {
        // Corrected: 'box' is an identifier
        let src = r#"
        map box {
            +-+
            | |
            +-+
        }
        strand main {
            apply_map("box", 0, 0)
        }
        "#;
        let dna = compile(src, None).expect("Compilation failed");
        let mut vm = ChimeraVM::new(dna);

        for _ in 0..100 {
            vm.step();
        }

        // Top row (y=0)
        assert_eq!(vm.grid[0][0], Value::Str("+".to_string()));
        assert_eq!(vm.grid[0][1], Value::Str("-".to_string()));
        assert_eq!(vm.grid[0][2], Value::Str("+".to_string()));

        // Middle row (y=1)
        assert_eq!(vm.grid[1][0], Value::Str("|".to_string()));
        assert_eq!(vm.grid[1][2], Value::Str("|".to_string()));

        // Bottom row (y=2)
        assert_eq!(vm.grid[2][0], Value::Str("+".to_string()));
        assert_eq!(vm.grid[2][1], Value::Str("-".to_string()));
        assert_eq!(vm.grid[2][2], Value::Str("+".to_string()));
    }
}
