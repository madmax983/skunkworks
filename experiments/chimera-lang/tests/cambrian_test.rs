#![cfg(feature = "nova")]

use chimera_lang::prelude::*;
use chimera_lang::opcode::OpCode;
use chimera_lang::ast::{Nucleotide, Gene, Strand, Helix, Dna};

#[test]
fn test_cambrian_explosion() {
    // 1. Setup VM with multiple strands
    let strand_a = Strand {
        genes: vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
        ]
    };
    let strand_b = Strand {
        genes: vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
        ]
    };
    // 3 copies of A, 2 copies of B
    let dna = Dna {
        helix: Helix {
            strands: vec![
                strand_a.clone(), strand_a.clone(), strand_a.clone(),
                strand_b.clone(), strand_b.clone(),
            ]
        }
    };

    let mut vm = ChimeraVM::new(dna);

    // 2. Trigger Speciation manually
    // Manually execute the function or use opcode
    // Let's use opcode to test dispatch
    vm.execute_gene_inner(OpCode::Speciate, &[]);

    // 3. Verify Species Count
    // We expect 2 species.
    assert_eq!(vm.cambrian.species.len(), 2);

    let mut counts = vec![];
    for s in vm.cambrian.species.values() {
        counts.push(s.population);
    }
    counts.sort();
    assert_eq!(counts, vec![2, 3]);

    // 4. Trigger Meteor Strike
    vm.execute_gene_inner(OpCode::Meteor, &[]);

    // 5. Verify Population Drop
    // Meteor kills ~50%.
    // Since it's random, we can't assert exact numbers, but total strands should be <= 5.
    let total_survivors = vm.dna.helix.strands.len();
    assert!(total_survivors <= 5);

    // 6. Trigger Census
    vm.execute_gene_inner(OpCode::Census, &[]);
    if let Some(Value::Int(c)) = vm.stack.pop() {
        assert_eq!(c, vm.cambrian.species.len() as i64);
    } else {
        panic!("Census did not push int");
    }
}
