use chimera_lang::prolouge_compiler::compile;

#[test]
fn test_prolouge_compiler_unwrap_orca() {
    let src = "
    orca 10
    "; // Missing y
    let res = compile(src);
    assert!(res.is_err(), "Expected error for invalid orca instruction");
}
