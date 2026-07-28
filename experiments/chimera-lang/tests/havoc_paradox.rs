use chimera_lang::Paradox;

#[test]
#[ignore = "👺 HAVOC: Intended Panic"]
#[should_panic(expected = "index out of bounds: the len is 5 but the index is 5")]
fn test_havoc_paradox_triggers_and_do_at_end() {
    let mut p = Paradox::new();
    // In `parse_rule`, `do_idx` is checked.
    // If `do_idx` is the last index (e.g. len 5, do_idx is 4), `parts[do_idx + 1]` panics!
    let _ = p.parse_rule("rule test triggers always do");
}
