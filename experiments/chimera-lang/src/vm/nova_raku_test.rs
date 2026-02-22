use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::value::Value;
use crate::vm::ChimeraVM;

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
fn test_hyper_add() {
    // [ push([1, 2, 3]), push([10, 20, 30]), hyper_add() ]
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Junction(
                JunctionType::All,
                vec![
                    Nucleotide::Number(1),
                    Nucleotide::Number(2),
                    Nucleotide::Number(3),
                ],
            )],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Junction(
                JunctionType::All,
                vec![
                    Nucleotide::Number(10),
                    Nucleotide::Number(20),
                    Nucleotide::Number(30),
                ],
            )],
        },
        Gene {
            op: OpCode::HyperAdd,
            args: vec![],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));
    vm.step(); // push
    vm.step(); // push
    vm.step(); // hyper_add

    let res = vm.stack.pop().unwrap();
    if let Value::Junction(_, list) = res {
        assert_eq!(list.len(), 3);
        assert_eq!(list[0], Value::Int(11));
        assert_eq!(list[1], Value::Int(22));
        assert_eq!(list[2], Value::Int(33));
    } else {
        panic!("Expected Junction, got {:?}", res);
    }
}

#[test]
fn test_reduce_add() {
    // [ push([1, 2, 3, 4]), push("+"), reduce() ]
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Junction(
                JunctionType::All,
                vec![
                    Nucleotide::Number(1),
                    Nucleotide::Number(2),
                    Nucleotide::Number(3),
                    Nucleotide::Number(4),
                ],
            )],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("+".to_string())],
        },
        Gene {
            op: OpCode::Reduce,
            args: vec![],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));
    vm.step(); // push
    vm.step(); // push
    vm.step(); // reduce

    let res = vm.stack.pop().unwrap();
    assert_eq!(res, Value::Int(10));
}

#[test]
fn test_cross_mul() {
    // [ push([1, 2]), push([10, 20]), push("*"), cross() ]
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Junction(
                JunctionType::All,
                vec![Nucleotide::Number(1), Nucleotide::Number(2)],
            )],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Junction(
                JunctionType::All,
                vec![Nucleotide::Number(10), Nucleotide::Number(20)],
            )],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("*".to_string())],
        },
        Gene {
            op: OpCode::Cross,
            args: vec![],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));
    vm.step(); // push
    vm.step(); // push
    vm.step(); // push
    vm.step(); // cross

    let res = vm.stack.pop().unwrap();
    if let Value::Junction(_, list) = res {
        assert_eq!(list.len(), 4);
        assert_eq!(list[0], Value::Int(10)); // 1*10
        assert_eq!(list[1], Value::Int(20)); // 1*20
        assert_eq!(list[2], Value::Int(20)); // 2*10
        assert_eq!(list[3], Value::Int(40)); // 2*20
    } else {
        panic!("Expected Junction, got {:?}", res);
    }
}

#[test]
fn test_zip_with_add() {
    // [ push([1, 2, 3]), push([10, 20]), push("+"), zip_with() ]
    // Should result in [11, 22] (truncated)
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Junction(
                JunctionType::All,
                vec![
                    Nucleotide::Number(1),
                    Nucleotide::Number(2),
                    Nucleotide::Number(3),
                ],
            )],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Junction(
                JunctionType::All,
                vec![Nucleotide::Number(10), Nucleotide::Number(20)],
            )],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("+".to_string())],
        },
        Gene {
            op: OpCode::ZipWith,
            args: vec![],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));
    vm.step(); // push
    vm.step(); // push
    vm.step(); // push
    vm.step(); // zip_with

    let res = vm.stack.pop().unwrap();
    if let Value::Junction(_, list) = res {
        assert_eq!(list.len(), 2);
        assert_eq!(list[0], Value::Int(11));
        assert_eq!(list[1], Value::Int(22));
    } else {
        panic!("Expected Junction, got {:?}", res);
    }
}
