#[cfg(test)]
mod tests {
    use crate::lexicon::Lexicon;

    #[test]
    #[should_panic]
    fn test_lexicon_havoc() {
        // If we tokenize a string starting with quotes but never ending,
        // will it panic?
        let code = "\"unfinished string literal";
        let _lexicon = Lexicon::new(code);
    }
}
