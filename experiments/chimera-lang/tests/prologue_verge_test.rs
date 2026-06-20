use chimera_lang::opcode::OpCode;
use chimera_lang::prologue_esolang_compiler::compile;
use chimera_lang::vm::ChimeraVM;

#[test]
fn test_prologue_verge_integration() {
    let source = r#"
    verge {
        42
        simulate
    }
    "#;

    let dna = compile(source).expect("Compilation failed");

    // Verify AST correctness
    let genes = &dna.helix.strands[0].genes;
    assert_eq!(genes.len(), 2);
    assert_eq!(genes[0].op, OpCode::Push);
    assert_eq!(genes[0].args[0], chimera_lang::ast::Nucleotide::Number(42));
    assert_eq!(genes[1].op, OpCode::Verge);

    // Verify VM execution
    let mut vm = ChimeraVM::new(dna);

    // Execute Push
    vm.step();
    // Execute Verge
    vm.step();

    // Check that the output contains the Verge dispatch message
    assert!(
        vm.output
            .contains(&"Verge Computer logic triggered.".to_string()),
        "VM output did not contain expected Verge message. Output: {:?}",
        vm.output
    );
}
