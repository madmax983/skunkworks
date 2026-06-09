#![cfg(feature = "git")]
use chimera_lang::opcode::OpCode;
use chimera_lang::prologue_esolang_compiler::compile;

#[test]
fn test_git_associates_block_compilation() {
    let source = "
    git_associates {
        history
        diff
        123
        \"hello\"
    }
    ";

    let dna = compile(source).expect("Compilation failed");
    let genes = &dna.helix.strands[0].genes;
    assert_eq!(genes.len(), 4);
    assert_eq!(genes[0].op, OpCode::GitHistory);
    assert_eq!(genes[1].op, OpCode::GitDiffWorkspace);
    assert_eq!(genes[2].op, OpCode::Push);
    assert_eq!(genes[3].op, OpCode::Push);
}
