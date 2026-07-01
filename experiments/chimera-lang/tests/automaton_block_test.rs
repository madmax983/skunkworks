use chimera_lang::opcode::OpCode;
use chimera_lang::prologue_esolang_compiler::compile;

#[test]
fn test_automaton_block_compilation() {
    let source = "
    automaton {
        simulate
    }
    ";

    let dna = compile(source).expect("Compilation failed");
    let genes = &dna.helix.strands[0].genes;
    assert_eq!(genes.len(), 1);
    assert_eq!(genes[0].op, OpCode::Automaton);
}
