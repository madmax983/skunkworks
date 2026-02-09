use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::{ChimeraVM, Value};

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
fn test_extract_and_inject() {
    let genes0 = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
    ];
    let genes1 = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
        Gene { op: OpCode::Extract, args: vec![] },
        Gene { op: OpCode::Inject, args: vec![] },
    ];

    let dna = Dna {
        helix: Helix {
            strands: vec![
                Strand { genes: genes0 },
                Strand { genes: genes1 },
            ],
        },
    };

    let mut vm = ChimeraVM::new(dna);
    // Execute Strand 1
    vm.ip = (1, 0);

    vm.step(); // push(0)
    vm.step(); // extract(0)

    if let Some(Value::Plasmid(genes)) = vm.stack.last() {
        assert_eq!(genes.len(), 1);
        assert_eq!(genes[0].op, OpCode::Push);
    } else {
        panic!("Expected Plasmid on stack");
    }

    vm.step(); // inject

    assert_eq!(vm.dna.helix.strands.len(), 3);
    assert_eq!(vm.dna.helix.strands[2].genes.len(), 1);
    assert_eq!(vm.dna.helix.strands[2].genes[0].op, OpCode::Push);
}

#[test]
fn test_sample_enzyme() {
    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(99)] }, // 0
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },  // 1: len
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },  // 2: start
        Gene { op: OpCode::Sample, args: vec![] },                     // 3
        Gene { op: OpCode::Enzyme, args: vec![] },                     // 4
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));

    vm.step(); // push(99)
    vm.step(); // push(1)
    vm.step(); // push(0)
    vm.step(); // sample

    if let Some(Value::Plasmid(_)) = vm.stack.last() {
        // ok
    } else {
        panic!("Expected Plasmid");
    }

    vm.step(); // enzyme

    if vm.stack.len() != 2 {
        println!("Stack: {:?}", vm.stack);
        println!("Output: {:?}", vm.output);
    }

    // Stack should be [99, 99]
    assert_eq!(vm.stack.len(), 2);
    assert_eq!(vm.stack[0], Value::Int(99));
    assert_eq!(vm.stack[1], Value::Int(99));
}

#[test]
fn test_cut_paste() {
    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // 0
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // 1: target to cut
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // 2: len
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // 3: start
        Gene { op: OpCode::Cut, args: vec![] },                       // 4
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // 5: insert index
        Gene { op: OpCode::Paste, args: vec![] },                     // 6
    ];
    // Ops:
    // 0: push(1)
    // 1: push(1) (This one is cut)
    // 2: push(1) (Stack: len)
    // 3: push(1) (Stack: start)
    // 4: Cut

    let mut vm = ChimeraVM::new(make_dna(genes));

    // Step manually to debug if needed
    for _ in 0..100 {
        if vm.halted || vm.ip.0 != 0 || vm.ip.1 >= vm.dna.helix.strands[0].genes.len() {
            break;
        }
        vm.step();
    }

    // Check genome
    // Original: 7 genes.
    // Cut 1. Len 6.
    // Paste 1. Len 7.
    // Result should have 7 genes.

    if vm.dna.helix.strands[0].genes.len() != 7 {
        println!("Genome: {:?}", vm.dna.helix.strands[0].genes);
        println!("Output: {:?}", vm.output);
    }

    let genes = &vm.dna.helix.strands[0].genes;
    assert_eq!(genes.len(), 7);
    assert_eq!(genes[0].op, OpCode::Push);
    assert_eq!(genes[0].args[0], Nucleotide::Number(1));

    // The pasted one is at 0.
    // The original at 0 (push(1)) is now at 1.
    assert_eq!(genes[1].op, OpCode::Push);
    assert_eq!(genes[1].args[0], Nucleotide::Number(1));
}
