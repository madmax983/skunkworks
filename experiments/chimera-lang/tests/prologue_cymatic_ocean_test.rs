use chimera_lang::opcode::OpCode;
use chimera_lang::prologue_esolang_compiler::compile;
use chimera_lang::vm::ChimeraVM;

#[test]
fn test_prologue_cymatic_ocean_integration() {
    let source = r#"
    cymatic_ocean {
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
    assert_eq!(genes[1].op, OpCode::CymaticOcean);

    // Verify VM execution
    let mut vm = ChimeraVM::new(dna);

    // Execute Push
    vm.step();
    // Execute CymaticOcean
    vm.step();

    // Check that the output contains the CymaticOcean dispatch message
    assert!(
        vm.output
            .contains(&"Cymatic Ocean simulation triggered.".to_string()),
        "VM output did not contain expected Cymatic Ocean message. Output: {:?}",
        vm.output
    );
}
