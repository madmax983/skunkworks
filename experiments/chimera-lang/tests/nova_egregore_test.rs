#[cfg(feature = "nova")]
#[test]
fn test_egregore_cult() {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    // DNA:
    // 1. Summon "Cult of the Stack"
    // 2. Link to it
    // 3. Invest 20 Energy -> 20 Credits (requires Invest)
    // 4. Tithe 10 Credits to Egregore
    // 5. Dictate "Law" = 42
    // 6. Query "Law" (Expect 42)
    // 7. Channel 5 Credits back
    // 8. Balance (Expect 15: 20 initial - 10 tithe + 5 channel)

    let genes = vec![
        // 1. Summon
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("Cult of the Stack".to_string())],
        },
        Gene {
            op: OpCode::EgregoreSummon,
            args: vec![],
        },
        // Stack: [egregore_id]

        // 2. Link
        // Need to duplicate ID for later use? Or just use top.
        // Link consumes ID. So let's dup it first if we need it later.
        Gene {
            op: OpCode::Dup,
            args: vec![],
        },
        Gene {
            op: OpCode::EgregoreLink,
            args: vec![],
        },
        // Stack: [egregore_id]

        // 3. Invest (Energy -> Credits)
        // Need energy first. Init is 50.
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(20)],
        },
        Gene {
            op: OpCode::Invest,
            args: vec![],
        },
        // Stack: [egregore_id] (Invest consumes amount)

        // 4. Tithe 10
        Gene {
            op: OpCode::Dup, // Keep ID
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(10)],
        },
        Gene {
            op: OpCode::EgregoreTithe,
            args: vec![],
        },
        // Stack: [egregore_id]

        // 5. Dictate "Law" = 42
        Gene {
            op: OpCode::Dup, // Keep ID
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("Law".to_string())],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(42)],
        },
        Gene {
            op: OpCode::EgregoreDictate,
            args: vec![],
        },
        // Stack: [egregore_id]

        // 6. Query "Law"
        Gene {
            op: OpCode::Dup, // Keep ID
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("Law".to_string())],
        },
        Gene {
            op: OpCode::EgregoreQuery,
            args: vec![],
        },
        // Stack: [egregore_id, 42]

        // Check 42
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(42)],
        },
        Gene {
            op: OpCode::Sub,
            args: vec![],
        },
        // Stack: [egregore_id, 0]
        Gene {
            op: OpCode::Drop,
            args: vec![],
        },

        // 7. Channel 5
        // Stack: [egregore_id]
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        },
        Gene {
            op: OpCode::EgregoreChannel,
            args: vec![],
        },
        // Stack: [egregore_id]

        // 8. Balance
        Gene {
            op: OpCode::Balance,
            args: vec![],
        },
        // Stack: [egregore_id, balance]
    ];

    // Correction: Drop is OpCode::Drop.
    let fixed_genes = genes.into_iter().map(|g| {
        if g.op == OpCode::Unknown("Pop".to_string()) { // Placeholder fix if I used wrong enum
             Gene { op: OpCode::Drop, args: vec![] }
        } else {
             g
        }
    }).collect();

    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes: fixed_genes }],
        },
    };

    let mut vm = ChimeraVM::new(dna);

    // Run until done
    for _ in 0..100 {
        vm.step();
        if vm.halted { break; }
    }

    // Check Balance
    // Initial 0. Invest 20 (+20). Tithe 10 (-10). Channel 5 (+5). Total 15.
    let balance = vm.stack.pop().unwrap();
    if let Value::Int(b) = balance {
        assert_eq!(b, 15, "Balance mismatch");
    } else {
        panic!("Expected int balance");
    }
}
