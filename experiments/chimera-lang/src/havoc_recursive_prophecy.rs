#[cfg(test)]
mod tests {
    use crate::compiler::compile;
    #[test]
    fn test_deeply_nested_macro_blowup() {
        // Recursive macros are a classic vector.
        // Or just huge macro expansions. Let's see if we can trigger a blowup.
        let mut src = String::new();
        for i in 0..100 {
            src.push_str(&format!("macro M{} {{ M{} M{} }}\n", i, i + 1, i + 1));
        }
        src.push_str("macro M100 { push(1) }\n");
        src.push_str("strand main { M0 }\n");

        let res = compile(&src, None);
        // Should not panic, should return error
        assert!(res.is_err());
    }
}
