#![cfg(feature = "git")]
use chimera_lang::opcode::OpCode;
use chimera_lang::prologue_esolang_compiler::compile;

#[test]
fn test_git_block_compilation() {
    let source = "
    git {
        ancestry
        \"hello\"
        excavate
        123
        evolution
    }
    ";

    let dna = compile(source).expect("Compilation failed");
    let genes = &dna.helix.strands[0].genes;
    assert_eq!(genes.len(), 5);
    assert_eq!(genes[0].op, OpCode::Ancestry);
    assert_eq!(genes[1].op, OpCode::Push);
    assert_eq!(genes[2].op, OpCode::Excavate);
    assert_eq!(genes[3].op, OpCode::Push);
    assert_eq!(genes[4].op, OpCode::Evolution);
}
