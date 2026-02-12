use chimera_lang::vm::ChimeraVM;
use chimera_lang::ast::{Dna, Helix, Strand, Gene, Nucleotide};
use chimera_lang::opcode::OpCode;

#[test]
fn test_entropy_surge() {
    let genes = vec![
        Gene { op: OpCode::EntropySurge, args: vec![] },
    ];
    let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
    let mut vm = ChimeraVM::new(dna);

    vm.step();

    assert!(vm.glitch_level > 0.0);
    assert!(vm.energy < 50);
    assert!(vm.output.iter().any(|s| s.contains("ENTROPY SURGE")));
}

#[test]
fn test_quantum_tunnel() {
    let genes = vec![
        Gene { op: OpCode::QuantumTunnel, args: vec![] },
        Gene { op: OpCode::Nop, args: vec![] },
        Gene { op: OpCode::Nop, args: vec![] },
    ];
    let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
    let mut vm = ChimeraVM::new(dna);

    vm.step();

    assert!(vm.output.iter().any(|s| s.contains("QUANTUM TUNNEL")));
}
