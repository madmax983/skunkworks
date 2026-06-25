use chimera_lang::prologue_esolang_compiler::compile;

#[test]
fn test_large_number_panic() {
    let source = "hologram_text { 999999999999999999999999999999 }";
    let _ = compile(source);
}
