use proptest::prelude::*;
use chimera_lang::prologue_compiler;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn test_prologue_parser_fuzz(input in "\\PC*") {
        let _ = prologue_compiler::compile(&input, None);
    }
}
