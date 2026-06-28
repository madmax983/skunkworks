use chimera_lang::prologue_esolang_compiler::compile;
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;

#[test]
fn test_chromatic_code_integration() {
    let source = "
chromatic_code {
    steg
}
    ";

    let dna = compile(source).expect("Failed to compile chromatic_code block");

    // Check compilation
    let genes = &dna.helix.strands[0].genes;
    assert_eq!(genes.len(), 1);
    assert_eq!(genes[0].op, OpCode::ChromaticCode);

    // Check execution
    let mut vm = ChimeraVM::new(dna);
    vm.step();

    // Verify side effect
    assert!(
        vm.output.contains(&"CHROMATIC_CODE: Carrier Signal Detected 🌈\u{1f510}".to_string())
    );
    assert_eq!(
        vm.grid[0][0],
        chimera_lang::vm::Value::Str("HIDDEN_DATA".to_string())
    );
}
