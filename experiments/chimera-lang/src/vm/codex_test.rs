use crate::prelude::*;
use crate::vm::codex;

#[test]
fn test_codex_void_bind() {
    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
        Gene { op: OpCode::Print, args: vec![] },
    ];
    let dna = Dna {
        helix: Helix { strands: vec![Strand { genes }] },
        evolution_config: None,
    };
    let mut vm = ChimeraVM::new(dna);

    // Initial state
    assert_eq!(vm.dna.helix.strands[0].genes.len(), 2);

    // Find Void Bind spell
    let spell = vm.codex.spells.iter().find(|s| s.name == "Void Bind").expect("Void Bind spell not found").clone();

    // Cast it
    // Ensure enough energy
    vm.energy = 100;
    codex::exec_spell(&mut vm, &spell);

    // Verifygenes cleared
    assert_eq!(vm.dna.helix.strands[0].genes.len(), 0);
    assert!(vm.output.iter().any(|s| s.contains("Strand bound to Void")));
}

#[test]
fn test_codex_opcode() {
    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // Spell ID 0 (Void Bind)
        Gene { op: OpCode::Codex, args: vec![] },
    ];
    let dna = Dna {
        helix: Helix { strands: vec![Strand { genes }] },
        evolution_config: None,
    };
    let mut vm = ChimeraVM::new(dna);
    vm.energy = 100;

    vm.step(); // Push 0
    vm.step(); // Codex

    // Verify genes cleared
    assert_eq!(vm.dna.helix.strands[0].genes.len(), 0);
}
