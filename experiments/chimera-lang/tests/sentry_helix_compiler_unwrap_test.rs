use chimera_lang::helix_compiler::compile;

#[test]
fn test_helix_compiler_unwrap_invalid_syntax() {
    let inputs = vec![
        "bang",     // missing arguments
        "bang 8",   // missing one argument
        "bang 8 8", // missing strand separator
        "a(b)",     // missing . in prolog
        ">> +",     // missing << in hyper
    ];

    for src in inputs {
        let res = compile(src);
        assert!(
            res.is_err(),
            "Expected error for invalid helix syntax: {}",
            src
        );
    }
}
