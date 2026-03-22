#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::vm::paradox::Paradox;

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_havoc_paradox_out_of_bounds() {
        let mut paradox = Paradox::new();
        let _ = paradox.parse_rule("rule Name triggers always do");
    }
}
