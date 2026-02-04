use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

fn make_dna(strands: Vec<Strand>) -> Dna {
    Dna {
        helix: Helix { strands },
    }
}

#[test]
fn test_conjugation_incubation_cycle() {
    // Strand 0: The Conjugator
    // [ push(1) push(5) push(5) push(0) conjugate() ]
    // Writes Strand 1 to grid at (5,5) direction Right (0)
    let conjugator = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)], // Target Strand Index (Template)
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)], // Y
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)], // X
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // Direction Right
            },
            Gene {
                op: OpCode::Conjugate,
                args: vec![],
            },
        ],
    };

    // Strand 1: The Template
    // [ s_index() ]
    // Pushes its own index.
    let template = Strand {
        genes: vec![Gene {
            op: OpCode::SIndex,
            args: vec![],
        }],
    };

    // Strand 2: The Incubator
    // [ push(1) push(5) push(5) incubate() ]
    // Reads 1 cell from (5,5).
    // Template has: s_index() -> 1 cell ("s_index")
    let incubator = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)], // Length
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)], // Y
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)], // X
            },
            Gene {
                op: OpCode::Incubate,
                args: vec![],
            },
        ],
    };

    let dna = make_dna(vec![conjugator, template, incubator]);
    let mut vm = ChimeraVM::new(dna);

    // Run enough steps for everyone to finish once.
    // S0 (5 genes), S1 (1 gene), S2 (4 genes). Total ~10 steps.
    // Plus some buffer.

    for _ in 0..50 {
        vm.step();
    }

    // Check Grid
    // (5,5) should be "s_index"
    assert_eq!(vm.grid[5][5], Value::Str("s_index".to_string()));

    // Check if new strand was created (Strand 3)
    assert_eq!(vm.dna.helix.strands.len(), 4);

    // Check content of Strand 3
    let new_strand = &vm.dna.helix.strands[3];
    assert_eq!(new_strand.genes.len(), 1);
    assert_eq!(new_strand.genes[0].op, OpCode::SIndex);

    // S3 should have run by now (it follows S2).
    // Stack expectation:
    // S1 ran: pushed 1.
    // S3 ran: pushed 3.
    // Stack should contain [1, 3] (possibly others from args if not popped? No, args are popped).
    // Wait, conjugator pops args. incubator pops args.
    // So stack should be clean before S1 runs.

    // S0 pops 4. Stack empty.
    // S1 pushes 1. Stack: [1].
    // S2 pops 3. Stack: [1].
    // S3 pushes 3. Stack: [1, 3].

    assert_eq!(vm.stack.len(), 2);
    assert_eq!(vm.stack[0], Value::Int(1));
    assert_eq!(vm.stack[1], Value::Int(3));
}
