#[cfg(test)]
mod tests {
    use chimera_lang::compiler::compile;

    #[test]
    fn test_evolution_parsing() {
        let src = r#"
        evolution MyEvo {
            population: 100
            mutation_rate: 0.05
        }
        strand main {
            push(1)
        }
        "#;
        let result = compile(src, None);
        assert!(
            result.is_ok(),
            "Failed to compile evolution block: {:?}",
            result.err()
        );
    }
}
