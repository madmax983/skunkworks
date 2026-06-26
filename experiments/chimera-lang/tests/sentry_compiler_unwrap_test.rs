use chimera_lang::compiler::compile;

#[test]
fn test_compiler_map_empty_content_unwrap() {
    let src = "
    map {}
    ";
    let res = compile(src, None);
    assert!(res.is_err(), "Expected error for invalid map block");
}

#[test]
fn test_compiler_strand_unwrap() {
    let src = "
    strand {}
    ";
    let res = compile(src, None);
    assert!(res.is_err(), "Expected error for invalid strand block");
}

#[test]
fn test_compiler_evolution_unwrap() {
    // Actually an empty evolution block might be valid syntax wise (just no properties).
    // Let's test a property that is missing parts
    let src = "
    evolution foo {
        fitness {
    }
    ";
    let res = compile(src, None);
    assert!(res.is_err(), "Expected error for invalid evolution block");
}

#[test]
fn test_compiler_grammar_unwrap() {
    let src = "
    grammar foo {
        rule = {}
    }
    ";
    let res = compile(src, None);
    assert!(res.is_err(), "Expected error for invalid grammar block");
}
