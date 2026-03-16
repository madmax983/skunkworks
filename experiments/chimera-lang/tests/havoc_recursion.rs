#[cfg(test)]
mod tests {
    use chimera_lang::compiler::compile;

    #[test]
    fn test_deeply_nested_junction_crash() {
        let depth = 20000;
        let mut deep_nesting = String::with_capacity(depth * 5);
        for _ in 0..depth {
            deep_nesting.push_str("any(");
        }
        deep_nesting.push('1');
        for _ in 0..depth {
            deep_nesting.push(')');
        }

        let src = format!(
            r#"
            strand main {{
                {}
            }}
            "#,
            deep_nesting
        );

        println!("Compiling deeply nested junction of depth {}", depth);
        let result = compile(&src, None);
        assert!(
            result.is_err(),
            "Expected compilation error due to recursion depth"
        );
        let err = result.unwrap_err();
        assert!(
            err.to_string().contains("Recursion depth exceeded"),
            "Expected specific recursion error, got: {}",
            err
        );
    }
}
