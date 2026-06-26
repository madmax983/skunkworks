#[cfg(test)]
mod tests {
    use crate::prologue_esolang_compiler::compile;
    use crate::vm::ChimeraVM;

    #[test]
    fn test_entropic_rain_execution() {
        // Compile the EntropicRain block logic.
        let code = r#"
        entropic_rain {
            simulate
        }
        "#;

        let dna = compile(code).expect("Should compile entropic_rain block");
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
            .any(|line| line.contains("🌧️ Entropic Rain simulation complete."));
        assert!(
            has_log,
            "Expected '🌧️ Entropic Rain simulation complete.' in output. Found: {:?}",
            vm.output
        );
    }
}
