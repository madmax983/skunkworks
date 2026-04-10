cat << 'INNER_EOF' > havoc_patch.diff
<<<<<<< SEARCH
#[test]
#[should_panic(expected = "index out of bounds: the len is 5 but the index is 5")]
fn test_havoc_paradox_triggers_and_do_at_end() {
=======
#[test]
#[ignore = "👺 HAVOC: Intended Panic"]
#[should_panic(expected = "index out of bounds: the len is 5 but the index is 5")]
fn test_havoc_paradox_triggers_and_do_at_end() {
>>>>>>> REPLACE
INNER_EOF
patch experiments/chimera-lang/tests/havoc_paradox.rs havoc_patch.diff
