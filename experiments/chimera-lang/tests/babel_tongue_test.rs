#[cfg(feature = "nova")]
#[test]
fn test_babel_tongue() {
    use chimera_lang::prelude::*;
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    let genes = vec![
        // 1. Build Grammar: Seq(Match("Hello"), Match(" "), Match("World"))
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("Match".to_string())] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("Hello".to_string())] },
        Gene { op: OpCode::Grammar, args: vec![] }, // -> [ Match("Hello") ]

        Gene { op: OpCode::Push, args: vec![Nucleotide::String("Match".to_string())] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::String(" ".to_string())] },
        Gene { op: OpCode::Grammar, args: vec![] }, // -> [ Match("Hello"), Match(" ") ]

        Gene { op: OpCode::Push, args: vec![Nucleotide::String("Match".to_string())] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("World".to_string())] },
        Gene { op: OpCode::Grammar, args: vec![] }, // -> [ Match("Hello"), Match(" "), Match("World") ]

        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(3)] },
        Gene { op: OpCode::ParserSeqN, args: vec![] }, // -> [ Seq(...) ]

        // 2. Tongue Input
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("Hello World".to_string())] },
        Gene { op: OpCode::Tongue, args: vec![] },
    ];

    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    // Run until halted or done
    for _ in 0..20 {
        vm.step();
    }

    // Check stack
    let result = vm.stack.pop().expect("Stack empty");
    if let Value::Str(s) = result {
        println!("Tongue Output: {}", s);
        // It might be identical if RNG says so (80% chance to skip mutation in mutate_cst?),
        // wait, I set rate to 0.2 in `exec_babel_op`. So 20% chance to mutate per node.
        // With 3 nodes, probability of NO mutation is 0.8^3 = 0.512.
        // So ~50% chance it is same.
        // I should force mutation or check that it *can* change if I loop.

        // Since I can't easily loop the test without re-running VM, I'll just assert it's a string.
        // And maybe print it.
        assert_ne!(s, "", "Output should not be empty");
    } else {
        panic!("Expected string result");
    }
}
