#![cfg(all(test, feature = "nova"))]

use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::ChimeraVM;

fn make_dna(strands: Vec<Strand>) -> Dna {
    Dna {
        helix: Helix { strands },
    }
}

fn make_strand(genes: Vec<Gene>) -> Strand {
    Strand { genes }
}

fn make_gene(op: OpCode, args: Vec<Nucleotide>) -> Gene {
    Gene { op, args }
}

#[test]
fn test_retroscribe_basic() {
    // Strand 0: Target [ Push(1) Push(2) Add ]
    // Strand 1: Pattern [ Push(2) ]
    // Strand 2: Replacement [ Push(3) Push(3) ]
    // Strand 3: Main [ Retroscribe(2, 1, 0) ]

    let target = make_strand(vec![
        make_gene(OpCode::Push, vec![Nucleotide::Number(1)]),
        make_gene(OpCode::Push, vec![Nucleotide::Number(2)]),
        make_gene(OpCode::Add, vec![]),
    ]);

    let pattern = make_strand(vec![
        make_gene(OpCode::Push, vec![Nucleotide::Number(2)]),
    ]);

    let replacement = make_strand(vec![
        make_gene(OpCode::Push, vec![Nucleotide::Number(3)]),
        make_gene(OpCode::Push, vec![Nucleotide::Number(3)]),
    ]);

    let main = make_strand(vec![
        make_gene(OpCode::Push, vec![Nucleotide::Number(2)]), // Replacement Idx
        make_gene(OpCode::Push, vec![Nucleotide::Number(1)]), // Pattern Idx
        make_gene(OpCode::Push, vec![Nucleotide::Number(0)]), // Target Idx
        make_gene(OpCode::Retroscribe, vec![]),
    ]);

    let dna = make_dna(vec![target, pattern, replacement, main]);
    let mut vm = ChimeraVM::new(dna);

    // Jump to main strand (3)
    vm.ip = (3, 0);

    // Execute Main Strand
    // Push(2), Push(1), Push(0), Retroscribe
    for _ in 0..4 {
        vm.step();
    }

    // Verify Target Strand (0)
    // Should be: [ Push(1) Push(3) Push(3) Add ]
    let genes = &vm.dna.helix.strands[0].genes;
    assert_eq!(genes.len(), 4);
    assert_eq!(genes[0].op, OpCode::Push);
    assert_eq!(genes[0].args[0], Nucleotide::Number(1));
    assert_eq!(genes[1].op, OpCode::Push);
    assert_eq!(genes[1].args[0], Nucleotide::Number(3));
    assert_eq!(genes[2].op, OpCode::Push);
    assert_eq!(genes[2].args[0], Nucleotide::Number(3));
    assert_eq!(genes[3].op, OpCode::Add);
}
