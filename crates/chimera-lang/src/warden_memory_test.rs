#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::value::Value;
    use crate::vm::ChimeraVM;

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_dos_incubate_memory() {
        // [ push(1000000) push(0) push(0) incubate() ]
        // Should cap length to avoid OOM
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1_000_000)],
            }, // length
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // x
            Gene {
                op: OpCode::Incubate,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);

        // We need to verify that the created strand is NOT 1,000,000 in length.
        // Step execution
        vm.step(); // push
        vm.step(); // push
        vm.step(); // push
        vm.step(); // incubate

        let new_strand_idx = vm.dna.helix.strands.len().saturating_sub(1);
        if new_strand_idx == 0 {
            // Failed to incubate (maybe due to stack underflow if setup wrong? No, args correct)
            // Or maybe it failed silently.
            // But if it succeeded, it should be a new strand.
        }

        if new_strand_idx > 0 {
            let new_strand = &vm.dna.helix.strands[new_strand_idx];
            assert!(
                new_strand.genes.len() < 10_000,
                "Incubate length should be capped"
            );
        }
    }

    #[test]
    fn test_dos_methylate_epigenome_size() {
        // [ push(0) push(0) methylate() jump(0) ]
        // Loop: push(random/incrementing) push(0) methylate

        // Simplified: Just call execute_gene_inner in a loop
        let mut vm = make_vm(vec![]);

        for i in 0..10_000 {
            vm.stack.push(Value::Int(0)); // strand
            vm.stack.push(Value::Int(i)); // gene
            vm.execute_gene_inner(OpCode::Methylate, &[]);
        }

        assert!(vm.epigenome.len() < 5000, "Epigenome size should be capped");
    }

    #[test]
    fn test_stack_overflow_nucleotide_formatting() {
        // Create a deeply nested Junction in stack/grid and try to Decompile it.
        // Decompile calls format_nucleotide which recurses.

        let mut vm = make_vm(vec![]);

        // Manually construct deep nucleotide
        let mut nuc = Nucleotide::Number(1);
        for _ in 0..2000 {
            nuc = Nucleotide::Junction(JunctionType::All, vec![nuc]);
        }

        // We need to put this into a strand to Decompile it.
        let gene = Gene {
            op: OpCode::Push,
            args: vec![nuc],
        };
        vm.dna.helix.strands[0].genes.push(gene);

        // push(0) decompile()
        vm.stack.push(Value::Int(0));
        vm.execute_gene_inner(OpCode::Decompile, &[]);

        // If vulnerable, this crashes with stack overflow (in test runner, often segfault or status 101).
        // If secure, it should handle it gracefully.
    }
}
