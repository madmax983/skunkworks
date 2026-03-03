#[cfg(test)]
mod tests {
    use crate::lisp::parse;

    #[test]
    fn test_tokenize_bomb() {
        let mut bomb = String::new();
        for _ in 0..100000 {
            bomb.push_str(" ");
        }
        bomb.push_str("\""); // open string, never close
        parse(&bomb).unwrap_err();
    }
}
