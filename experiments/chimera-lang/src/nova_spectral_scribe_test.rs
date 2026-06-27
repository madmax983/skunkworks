#[cfg(test)]
mod tests {
    use crate::prologue_esolang_compiler::compile;
    use crate::vm::ChimeraVM;

    #[test]
    fn test_spectral_scribe_execution() {
        // Compile the SpectralScribe block logic.
        let code = r#"
        spectral_scribe {
            encode
        }
        "#;

        let dna = compile(code).expect("Should compile spectral_scribe block");
        let mut vm = ChimeraVM::new(dna);

        // Run until completion or max steps
        for _ in 0..10 {
            if vm.halted {
                break;
            }
            vm.step();
        }

        // Output should contain the expected log.
        let has_log = vm
            .output
            .iter()
            .any(|line| line.contains("📻 Spectral Scribe encoding complete."));
        assert!(
            has_log,
            "Expected '📻 Spectral Scribe encoding complete.' in output. Found: {:?}",
            vm.output
        );
    }
}
