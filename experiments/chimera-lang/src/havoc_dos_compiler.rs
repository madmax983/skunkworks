#[cfg(test)]
mod tests {
    use crate::compiler::compile;

    #[test]
    fn test_huge_macro_expansion() {
        let mut src = String::new();
        src.push_str("macro M { push(1) push(1) }\n");
        for i in 0..15 { // 2^15 = 32768
            src.push_str(&format!("macro M{} {{ M M }}\n", i));
            src.push_str(&format!("macro M {{ M{} M{} }}\n", i, i));
        }
        src.push_str("strand main { M }\n");

        let res = compile(&src, None);
        assert!(res.is_err());
    }
}
