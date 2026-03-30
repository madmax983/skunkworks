#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::vm::paradox::Paradox;

    #[test]
    fn test_havoc_paradox_out_of_bounds() {
        let mut paradox = Paradox::new();
        let res = paradox.parse_rule("rule Name triggers always do");
        assert!(res.is_err());
    }
}
