use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
fn test_push_junction() {
    // [ push(any(1, 2)) ]
    let genes = vec![Gene {
        op: OpCode::Push,
        args: vec![Nucleotide::Junction(
            JunctionType::Any,
            vec![Nucleotide::Number(1), Nucleotide::Number(2)],
        )],
    }];
    let mut vm = ChimeraVM::new(make_dna(genes));
    vm.step();
    assert_eq!(vm.stack.len(), 1);
    if let Value::Junction(t, vals) = &vm.stack[0] {
        assert_eq!(*t, JunctionType::Any);
        assert_eq!(vals.len(), 2);
        assert_eq!(vals[0], Value::Int(1));
        assert_eq!(vals[1], Value::Int(2));
    } else {
        panic!("Expected Junction");
    }
}

#[test]
fn test_autothreading_add() {
    // [ push(any(1, 2)) push(10) add() ]
    // Expected: any(11, 12)
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Junction(
                JunctionType::Any,
                vec![Nucleotide::Number(1), Nucleotide::Number(2)],
            )],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(10)],
        },
        Gene {
            op: OpCode::Add,
            args: vec![],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));
    while !vm.halted {
        vm.step();
    }

    assert_eq!(vm.stack.len(), 1);
    if let Value::Junction(t, vals) = &vm.stack[0] {
        assert_eq!(*t, JunctionType::Any);
        assert_eq!(vals.len(), 2);
        assert_eq!(vals[0], Value::Int(11));
        assert_eq!(vals[1], Value::Int(12));
    } else {
        panic!("Expected Junction");
    }
}

#[test]
fn test_brz_any() {
    // [ push(any(0, 5)) brz(1) push(99) ]
    // 0 is in the set, so it should branch to strand 1.
    let strand0 = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::Any,
                    vec![Nucleotide::Number(0), Nucleotide::Number(5)],
                )],
            },
            Gene {
                op: OpCode::Brz,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(99)],
            },
        ],
    };
    let strand1 = Strand {
        genes: vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(200)],
        }],
    };
    let dna = Dna {
        helix: Helix {
            strands: vec![strand0, strand1],
        },
    };

    let mut vm = ChimeraVM::new(dna);
    vm.step(); // push
    vm.step(); // brz
    assert_eq!(vm.ip, (1, 0));
}

#[test]
fn test_brz_all() {
    // [ push(all(0, 5)) brz(1) push(99) ]
    // Not all are 0, so should NOT branch.
    let strand0 = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::All,
                    vec![Nucleotide::Number(0), Nucleotide::Number(5)],
                )],
            },
            Gene {
                op: OpCode::Brz,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(99)],
            },
        ],
    };
    let strand1 = Strand {
        genes: vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(200)],
        }],
    };
    let dna = Dna {
        helix: Helix {
            strands: vec![strand0, strand1],
        },
    };

    let mut vm = ChimeraVM::new(dna);
    vm.step(); // push
    vm.step(); // brz
    assert_eq!(vm.ip, (0, 2)); // Advanced to next gene in strand 0
}

#[test]
fn test_brz_all_success() {
    // [ push(all(0, 0)) brz(1) ]
    // All are 0, so branch.
    let strand0 = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::All,
                    vec![Nucleotide::Number(0), Nucleotide::Number(0)],
                )],
            },
            Gene {
                op: OpCode::Brz,
                args: vec![Nucleotide::Number(1)],
            },
        ],
    };
    let strand1 = Strand { genes: vec![] };
    let dna = Dna {
        helix: Helix {
            strands: vec![strand0, strand1],
        },
    };

    let mut vm = ChimeraVM::new(dna);
    vm.step(); // push
    vm.step(); // brz
    assert_eq!(vm.ip, (1, 0));
}
