use chimera_lang::opcode::OpCode;
use chimera_lang::prologue_esolang_compiler::compile;

#[test]
fn test_circuit_sigil_block_compilation() {
    let source = "
    circuit_sigil {
        42
        simulate
    }
    ";

    let dna = compile(source).expect("Compilation failed");
    let genes = &dna.helix.strands[0].genes;
    assert_eq!(genes.len(), 2);
    assert_eq!(genes[0].op, OpCode::Push);
    assert_eq!(genes[0].args[0], chimera_lang::ast::Nucleotide::Number(42));
    assert_eq!(genes[1].op, OpCode::CircuitSigil);
}
