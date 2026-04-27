use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;

#[test]
fn test_quipu_execution() {
    let genes = vec![
        Gene::new(OpCode::Push, vec![Nucleotide::Number(42)]),
        Gene::new(OpCode::Quipu, vec![]),
        Gene::new(
            OpCode::Push,
            vec![Nucleotide::String("knot_info".to_string())],
        ),
        Gene::new(OpCode::Quipu, vec![]),
        Gene::new(OpCode::Quipu, vec![]), // Should cause underflow
    ];
    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    vm.step(); // push 42
    vm.step(); // Quipu logic
    vm.step(); // push knot_info
    vm.step(); // Quipu logic
    vm.step(); // underflow Quipu logic

    assert!(vm
        .output
        .iter()
        .any(|s| s.contains("🧶 Quipu logic tied: Int(42)")));
    assert!(vm
        .output
        .iter()
        .any(|s| s.contains("🧶 Quipu logic tied: Str(\"knot_info\")")));
    assert!(vm
        .output
        .iter()
        .any(|s| s.contains("🧶 Quipu logic failed: stack underflow")));
}
