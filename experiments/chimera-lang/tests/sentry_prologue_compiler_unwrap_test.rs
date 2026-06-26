use chimera_lang::prologue_compiler::compile;

#[test]
fn test_prologue_compiler_unwrap() {
    let src = "
    grid {
        \"unterminated
        \"\\
    }
    config {
        mode Orca
    }
    definitions {
        a = {}
    }
    alchemy {
        \"A\" + \"B\" = \"C\"
    }
    ";
    let res = compile(src, None);
    assert!(res.is_err(), "Expected error for invalid prologue blocks");
}
