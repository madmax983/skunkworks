#[cfg(test)]
use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
#[cfg(test)]
use crate::opcode::OpCode;
#[cfg(test)]
use crate::vm::{ChimeraVM, Value};

#[cfg(test)]
fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
fn test_vaccinate_verify() {
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Vaccinate,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Verify,
            args: vec![],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));

    vm.step(); // push 0
    vm.step(); // vaccinate 0
    vm.step(); // push 0
    vm.step(); // verify 0

    assert_eq!(vm.stack.len(), 1);
    if let Value::Int(result) = vm.stack[0] {
        assert_eq!(result, 1, "Expected verified (1) after vaccination");
    } else {
        panic!("Expected integer result");
    }
}

#[test]
fn test_audit() {
    // Strand 0: [ push(0) vaccinate() audit() ]
    // Strand 1: [ push(1) ] (Untrusted)
    let strand0 = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Vaccinate,
                args: vec![],
            },
            Gene {
                op: OpCode::Audit,
                args: vec![],
            },
        ],
    };
    let strand1 = Strand {
        genes: vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        }],
    };

    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![strand0, strand1],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    vm.step(); // push 0
    vm.step(); // vaccinate 0
    vm.step(); // audit

    let result = vm.stack.pop().unwrap();
    if let Value::Junction(t, vals) = result {
        assert_eq!(t, JunctionType::All);
        assert_eq!(vals.len(), 1);
        if let Value::Int(idx) = vals[0] {
            assert_eq!(idx, 1, "Expected strand 1 to be untrusted");
        } else {
            panic!("Expected integer index");
        }
    } else {
        panic!("Expected Junction, got {:?}", result);
    }
}

#[test]
fn test_mutation_breaks_immunity() {
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Vaccinate,
            args: vec![],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));

    vm.step(); // push 0
    vm.step(); // vaccinate 0

    // Manually mutate the strand
    vm.dna.helix.strands[0].genes[0].op = OpCode::Drop;

    // Verify manually
    // We inject verify logic via a new strand or just calling exec_security_op if accessible.
    // Easier: Add a second strand that verifies strand 0.

    // Actually, simpler:
    let verify_genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Verify,
            args: vec![],
        },
    ];

    vm.dna.helix.strands.push(Strand {
        genes: verify_genes,
    });

    // Jump to verification strand
    vm.ip = (1, 0);
    vm.step(); // push 0
    vm.step(); // verify 0

    assert_eq!(vm.stack.len(), 1);
    if let Value::Int(result) = vm.stack[0] {
        assert_eq!(result, 0, "Expected verify (0) after mutation");
    } else {
        panic!("Expected integer result");
    }
}
