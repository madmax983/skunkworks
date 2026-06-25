#[cfg(test)]
mod tests {
    use crate::prologue_esolang_compiler::compile;
    use crate::vm::ChimeraVM;

    #[test]
    fn test_hologram_text_execution() {
        // Compile the HologramText block logic.
        let code = r#"
        hologram_text {
            "Secret Data"
            reconstruct
        }
        "#;

        let dna = compile(code).expect("Should compile hologram_text block");
        let mut vm = ChimeraVM::new(dna);

        // Run until completion or max steps
        for _ in 0..10 {
            if vm.halted {
                break;
            }
            vm.step();
        }

        // Output should contain the expected reconstructed string log.
        let has_log = vm.output.iter().any(|line| line.contains("🌟 Hologram Text reconstructed: Str(\"Secret Data\")"));
        assert!(
            has_log,
            "Expected '🌟 Hologram Text reconstructed: Str(\"Secret Data\")' in output. Found: {:?}",
            vm.output
        );
    }
}
