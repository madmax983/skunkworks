#[cfg(test)]
mod tests {
    use crate::prologue_esolang_compiler::compile;
    use crate::vm::ChimeraVM;

    #[test]
    fn test_genetic_luthier_execution() {
        // Compile the GeneticLuthier block logic.
        let code = r#"
        genetic_luthier {
            pluck
        }
        "#;

        let dna = compile(code).expect("Should compile genetic_luthier block");
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
            .any(|line| line.contains("🎻 Genetic Luthier string \"pluck\"."));
        assert!(
            has_log,
            "Expected '🎻 Genetic Luthier string pluck.' in output. Found: {:?}",
            vm.output
        );
    }
}
