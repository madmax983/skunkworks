#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
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
    fn test_retrograde() {
        // [ push(10) push(0) push(0) g_write() ] -> write 10 at 0,0
        // [ push(20) push(0) push(0) g_write() ] -> write 20 at 0,0
        // [ push(2) retrograde() ] -> revert to 10

        let genes = vec![
            // 0: Push 10
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            // 1: Push 0 (y)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            // 2: Push 0 (x)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            // 3: GWrite (Grid -> 10)
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            // 4: Push 20
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(20)],
            },
            // 5: Push 0
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            // 6: Push 0
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            // 7: GWrite (Grid -> 20)
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            // 8: Push 5 (Go back 5 ticks to be safe? Or calculate exact)
            // Ticks passed:
            // 0, 1, 2, 3 (Write 10)
            // 4, 5, 6, 7 (Write 20)
            // 8 (Push N)
            // 9 (Retrograde)
            // Total 10 ticks.
            // We want state after tick 3 (before tick 7).
            // Tick 7 wrote 20. Tick 8 start pushed history (10).
            // Tick 9 start pushed history (20).
            // We want history from Tick 8 start (10).
            // Current is Tick 10 start (executing Retrograde).
            // History:
            // Len-1: Tick 10 start (20)
            // Len-2: Tick 9 start (20)
            // Len-3: Tick 8 start (10, because tick 7 finished writing 20? No, tick 7 START pushed history of tick 6 end).

            // Let's trace carefully.
            // Tick 1 (Exec Gene 0): Pushes grid (init 0). Grid -> 0.
            // ...
            // Tick 4 (Exec Gene 3 GWrite): Pushes grid (0). Executes GWrite. Grid -> 10.
            // Tick 5 (Exec Gene 4): Pushes grid (10). Grid -> 10.
            // Tick 6 (Exec Gene 5): Pushes grid (10). Grid -> 10.
            // Tick 7 (Exec Gene 6): Pushes grid (10). Grid -> 10.
            // Tick 8 (Exec Gene 7 GWrite): Pushes grid (10). Executes GWrite. Grid -> 20.
            // Tick 9 (Exec Gene 8 Push N): Pushes grid (20). Grid -> 20.
            // Tick 10 (Exec Gene 9 Retro): Pushes grid (20). Executes Retro.

            // We want state 10. That is available at Tick 5, 6, 7, 8 pushes.
            // Tick 8 push is at `len - (10-8) - 1` = `len - 3`?
            // History has 10 items.
            // Idx 0: Tick 1 push.
            // Idx 7: Tick 8 push (10).
            // Idx 8: Tick 9 push (20).
            // Idx 9: Tick 10 push (20).

            // Retrograde(N) reads `len - 1 - N`.
            // N=2 => `9 - 2` = 7. Value at 7 is 10.
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::Retrograde,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));

        // Execute all
        for _ in 0..10 {
            vm.step();
        }

        // Should be 10 again
        assert_eq!(vm.grid[0][0], Value::Int(10));
    }
}
