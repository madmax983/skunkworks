#[cfg(test)]
mod tests {
    use crate::prologue_esolang_compiler::compile;
    use crate::vm::ChimeraVM;

    #[test]
    fn test_spqr_rsa_execution() {
        // Compile the SpqrRsa block logic.
        let code = r#"
        spqr_rsa {
            encrypt
        }
        "#;

        let dna = compile(code).expect("Should compile spqr_rsa block");
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
            .any(|line| line.contains("🏛️ SPQR RSA cryptography simulated."));
        assert!(
            has_log,
            "Expected '🏛️ SPQR RSA cryptography simulated.' in output. Found: {:?}",
            vm.output
        );
    }
}
