#[cfg(test)]
mod tests {
    use crate::prologue_esolang_compiler::compile;
    use crate::vm::ChimeraVM;

    #[test]
    fn test_soroban_specter_execution() {
        // Compile the SorobanSpecter block logic.
        let code = r#"
        soroban_specter {
            calculate
        }
        "#;

        let dna = compile(code).expect("Should compile soroban_specter block");
        let mut vm = ChimeraVM::new(dna);

        // Run until completion or max steps
        for _ in 0..10 {
            if vm.halted {
                break;
            }
            vm.step();
        }

        // Output should contain the expected reconstructed string log.
        let has_log = vm
            .output
            .iter()
            .any(|line| line.contains("🧮 Soroban Specter calculation simulated."));
        assert!(
            has_log,
            "Expected '🧮 Soroban Specter calculation simulated.' in output. Found: {:?}",
            vm.output
        );
    }
}
