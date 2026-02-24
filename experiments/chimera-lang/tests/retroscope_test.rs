#[cfg(feature = "nova")]
#[test]
fn test_retroscope() {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    // DNA Sequence:
    // 1. push(10) push(0) push(0) g_write()  -> Write 10 to (0,0)
    // 2. step (Implicitly handled by execution flow, but we are running step() manually)
    // We will construct DNA that just does ONE operation per tick if we are careful,
    // but typically `step()` runs until it yields or hits instruction limit.
    // By default `step()` executes 1 instruction/gene (unless buffs).

    // We will use a sequence of instructions and verify state manually.

    let genes = vec![
        // Tick 1: Push 10
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(10)],
        },
        // Tick 2: Push 0
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        // Tick 3: Push 0
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        // Tick 4: GWrite (10 to 0,0)
        Gene {
            op: OpCode::GWrite,
            args: vec![],
        },
        // Tick 5: Push 20
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(20)],
        },
        // Tick 6: Push 0
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        // Tick 7: Push 0
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        // Tick 8: GWrite (20 to 0,0)
        Gene {
            op: OpCode::GWrite,
            args: vec![],
        },
        // Tick 9: Retroscope(4, 0, 0)
        // Current Grid is 20.
        // We want to see 10.
        // GWrite(10) happened at Tick 4.
        // GWrite(20) happened at Tick 8.
        // At start of Tick 9, history contains state after Tick 8.
        // History: [State0, State1, ..., State8].
        // State8 has Grid(20). State4 has Grid(10).
        // Retroscope(ticks) -> History[len - 1 - ticks].
        // History len = 9 (0 to 8).
        // Ticks = 4. Index = 9 - 1 - 4 = 4.
        // State4 is AFTER Tick 4 executed. So it has 10.
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(4)],
        }, // ticks
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }, // y
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }, // x
        Gene {
            op: OpCode::Retroscope,
            args: vec![],
        },
    ];

    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    // Run 9 ticks to reach Retroscope
    // Ticks 1-4: Write 10.
    // Ticks 5-8: Write 20.
    // Tick 9: Retroscope.

    for _ in 0..12 {
        vm.step();
    }

    // Check stack
    // Should have 10 (from retroscope)

    // We expect the stack to contain the retroscope result.
    // The previous ops consumed the stack.

    // Stack should have 1 item: 10.
    assert_eq!(vm.stack.len(), 1, "Stack should have 1 item");
    assert_eq!(vm.stack[0], Value::Int(10), "Retroscope should return 10");
}
