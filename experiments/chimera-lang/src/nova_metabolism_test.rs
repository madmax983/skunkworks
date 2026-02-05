use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
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
fn test_hibernation() {
    // [ push(0) metabolism() push(1) ]
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Metabolism,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));

    // Step 1: Push 0
    vm.step();
    // Step 2: Metabolism (pops 0). Rate becomes 0.
    vm.step();
    assert_eq!(vm.metabolic_rate, 0);
    assert_eq!(vm.ip.1, 2); // Advanced past metabolism

    // Step 3: Should be Hibernating. IP should NOT advance.
    vm.step();
    assert_eq!(vm.ip.1, 2); // Still at 2
    assert_eq!(vm.stack.len(), 0); // Push(1) should not have happened
}

#[test]
fn test_overclock() {
    // [ push(2) metabolism() push(1) push(2) ]
    // Step 1: Push 2
    // Step 2: Metabolism
    // Step 3: Push 1 AND Push 2
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)],
        },
        Gene {
            op: OpCode::Metabolism,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(3)],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));

    vm.step(); // Push 2
    vm.step(); // Metabolism
    assert_eq!(vm.metabolic_rate, 2);
    assert_eq!(vm.ip.1, 2);

    // Step 3: Should run TWO instructions (Push 1, Push 2)
    vm.step();
    assert_eq!(vm.ip.1, 4); // 2 -> 3 -> 4
    assert_eq!(vm.stack.len(), 2);
    assert_eq!(vm.stack[0], Value::Int(1));
    assert_eq!(vm.stack[1], Value::Int(2));
}

#[test]
fn test_reflex_wake() {
    // [ reflex(0, 1) push(0) metabolism() push(999) ]
    // Fix: Reflex args are on Stack.
    let strand0 = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)], // Strand 1
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // Event 0
            },
            Gene {
                op: OpCode::Reflex,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // Rate 0
            },
            Gene {
                op: OpCode::Metabolism,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(999)],
            },
        ],
    };

    let strand1 = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Metabolism, // Wake up explicitly? No, Reflex wakes up.
                args: vec![],
            },
            Gene {
                op: OpCode::Ret, // Return to strand 0
                args: vec![],
            },
        ],
    };

    let dna = Dna {
        helix: Helix {
            strands: vec![strand0, strand1],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    vm.step(); // Push 1
    vm.step(); // Push 0
    vm.step(); // Reflex
    vm.step(); // Push 0
    vm.step(); // Metabolism
    assert_eq!(vm.metabolic_rate, 0);

    // Hibernate step
    vm.step();
    // IP should be at index 5 (Push 999).
    assert_eq!(vm.ip.0, 0);
    assert_eq!(vm.ip.1, 5);
    assert_eq!(vm.stack.len(), 0);

    // Trigger Reflex 0
    vm.trigger_reflex(0);

    assert_eq!(vm.metabolic_rate, 1, "Reflex should wake organism");
    assert_eq!(vm.ip.0, 1, "Should jump to handler");

    // Execute Handler
    // Handler: Push 1, Metabolism(1), Ret.
    vm.step(); // Push 1
    vm.step(); // Metabolism
    vm.step(); // Ret

    // Returns to Strand 0, Gene 5 (Push 999).
    // IP is restored to (0, 5).
    // But Ret was executed. Next instruction is at (0, 5).

    vm.step(); // Push 999
    assert_eq!(vm.stack.pop(), Some(Value::Int(999)));
}

#[test]
fn test_overclock_boundary_panic() {
    // [ metabolism(2) push(1) ]
    // Length 2.
    // Step 1: Set Rate 2. IP -> 1.
    // Step 2: Overclocked step.
    //   Iter 1: Push(1). IP -> 2. End of strand. IP -> (1, 0).
    //   Iter 2: IP is (1, 0). Helix length is 1. OOB!
    //   Should halt gracefully, not panic.
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)],
        },
        Gene {
            op: OpCode::Metabolism,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));

    vm.step(); // Push 2
    vm.step(); // Metabolism(2). IP=2.

    // Step 3: Run Overclocked.
    // Should run Push(1), then halt.
    vm.step();

    assert!(vm.halted);
    assert_eq!(vm.stack.pop(), Some(Value::Int(1)));
}
