#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::vm::paradox::Paradox;

    #[test]
    #[should_panic]
    fn test_havoc_paradox_out_of_bounds() {
        // Trigger: Out-of-bounds access in parse_rule when "do" is at the end of the input string.
        let mut paradox = Paradox::new();
        // This will cause parts[do_idx + 1] to panic because do_idx is the last element
        let _ = paradox.parse_rule("rule name triggers always do");
    }
}
