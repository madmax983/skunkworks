#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
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
    fn test_prophecy_recursion_bomb() {
        // 👺 HAVOC: Triggering Stack Overflow via Recursive Prophecy
        // Strategy: Use Signals to trigger Prophecy.
        // Signals use `execute_gene_inner` which BYPASSES `recursion_depth` check.
        // Prophecy also fails to increment recursion depth.
        // Thus, infinite recursion on the Rust stack.

        let mut genes = Vec::new();

        // 1. Fill stack with 'ticks' for Prophecy (needs to pop 10 each time)
        // We push enough to cause a deep enough recursion to blow the stack.
        // 50000 should definitely blow the stack or OOM.
        for _ in 0..50000 {
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)], // 100 ticks
            });
        }

        // 2. Write "prophecy" to Grid at (8,8)
        // Note: OpCodes are snake_case.
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::String("prophecy".to_string())] });
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] });
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] });
        genes.push(Gene { op: OpCode::GWrite, args: vec![] });

        // 3. Write "*" (Bang) to Grid at (8,7) to trigger signal on (8,8)
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::String("*".to_string())] });
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(7)] }); // y=7
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] }); // x=8
        genes.push(Gene { op: OpCode::GWrite, args: vec![] });

        // 4. Wait loop
        genes.push(Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(genes.len() as i64)] });

        let mut vm = ChimeraVM::new(make_dna(genes));

        // Step will:
        // 1. Execute genes to set up grid and stack.
        // 2. Once setup, the "*" will generate signals.
        // 3. process_signals will see "Prophecy" getting signal.
        // 4. execute_gene_inner(Prophecy) called (No depth inc).
        // 5. exec_prophecy called (Clones VM).
        // 6. sim_vm.step() called.
        // 7. sim_vm.process_signals() called.
        // 8. ... Infinite Recursion ...

        for _ in 0..2100 {
            vm.step();
        }
    }
}
