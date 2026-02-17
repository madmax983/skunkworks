#[cfg(test)]
mod tests {
    use chimera_lang::compiler::compile;

    #[test]
    #[ignore = "HAVOC: Causes stack overflow due to recursive parsing"]
    fn test_deeply_nested_junction_crash() {
        let depth = 20000;
        let mut deep_nesting = String::with_capacity(depth * 5);
        for _ in 0..depth {
            deep_nesting.push_str("any(");
        }
        deep_nesting.push_str("1");
        for _ in 0..depth {
            deep_nesting.push_str(")");
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
        // This should crash the process with a stack overflow
        let _ = compile(&src, None);
    }
}
