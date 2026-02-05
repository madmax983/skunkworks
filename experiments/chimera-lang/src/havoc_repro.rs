#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand, JunctionType};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM};

    fn make_nested_junction_dna(depth: usize) -> Dna {
        let mut nuc = Nucleotide::Number(1);
        for _ in 0..depth {
            nuc = Nucleotide::Junction(JunctionType::Any, vec![nuc]);
        }

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![nuc],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Add,
                args: vec![],
            }
        ];

        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_stack_overflow() {
        // 🧨 Havoc: Trigger recursion bomb via Junctions
        // We use depth 500.
        // - Safe for Rust default stack (Nucleotide Drop won't crash).
        // - Sufficient to trigger our new safety limits (100).
        let depth = 500;
        let mut vm = ChimeraVM::new(make_nested_junction_dna(depth));

        // Push Junction -> Should fail now! (complexity limit in nuc_to_val)
        vm.step();

        // Push 1 -> Stack has [1]
        vm.step();

        // Add -> Stack has [1]. Add needs 2. -> Stack underflow.
        // If Push succeeded (it shouldn't), Add would fail with complexity limit.
        vm.step();

        // Check for ANY of our safety nets
        let has_safety = vm.output.iter().any(|s|
            s.contains("complexity limit") ||
            s.contains("Invalid arg") ||
            s.contains("Stack underflow")
        );

        assert!(has_safety, "Expected VM to safely handle complexity bomb");
    }
}
