use chimera_lang::opcode::OpCode;
use chimera_lang::prologue_esolang_compiler::compile;
use chimera_lang::ast::Nucleotide;

#[test]
fn test_tui_mod_grammar_compilation() {
    let source = r#"
    tui_mod {
        50
        glitch
        10
        shake
        "CHAOS"
        message
    }
    "#;

    let dna = compile(source).expect("Compilation failed");
    let genes = &dna.helix.strands[0].genes;

    assert_eq!(genes.len(), 9);

    assert_eq!(genes[0].op, OpCode::Push);
    assert_eq!(genes[0].args[0], Nucleotide::Number(50));

    assert_eq!(genes[1].op, OpCode::Push);
    assert_eq!(genes[1].args[0], Nucleotide::Number(0));

    assert_eq!(genes[2].op, OpCode::TuiMod);

    assert_eq!(genes[3].op, OpCode::Push);
    assert_eq!(genes[3].args[0], Nucleotide::Number(10));

    assert_eq!(genes[4].op, OpCode::Push);
    assert_eq!(genes[4].args[0], Nucleotide::Number(1));

    assert_eq!(genes[5].op, OpCode::TuiMod);

    assert_eq!(genes[6].op, OpCode::Push);
    assert_eq!(genes[6].args[0], Nucleotide::String("CHAOS".to_string()));

    assert_eq!(genes[7].op, OpCode::Push);
    assert_eq!(genes[7].args[0], Nucleotide::Number(2));

    assert_eq!(genes[8].op, OpCode::TuiMod);
}
