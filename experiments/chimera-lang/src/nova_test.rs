#![cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::vm::ChimeraVM;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_epigenetics() {
        // [ push(0) push(3) methylate() push(100) ]
        // 0: push(0) - strand idx
        // 1: push(3) - gene idx (target)
        // 2: methylate()
        // 3: push(100) - this should be skipped
        let genes = vec![
            Gene { name: "push".to_string(), args: vec![Nucleotide::Number(0)] },
            Gene { name: "push".to_string(), args: vec![Nucleotide::Number(3)] },
            Gene { name: "methylate".to_string(), args: vec![] },
            Gene { name: "push".to_string(), args: vec![Nucleotide::Number(100)] },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Step 1: push(0)
        vm.step();
        // Step 2: push(3)
        vm.step();
        // Step 3: methylate
        vm.step();

        // Check if methylated
        // Note: vm.epigenome field access requires it to be pub
        assert!(vm.epigenome.contains(&(0, 3)));

        // Step 4: execute gene 3 (should be skipped)
        vm.step();

        // Stack should NOT contain 100.
        // Stack should be empty (0 and 3 were popped by methylate).
        assert!(vm.stack.is_empty(), "Stack should be empty, but has {:?}", vm.stack);

        // Test Demethylate
        // [ push(0) push(3) demethylate() ]
        // We need to add these genes or reset VM. Let's just manually append to the current DNA or rely on manual execution?
        // Easier to check if we can run the skipped gene now? No, we passed it.
        // Let's verify demethylation works by calling it.

        // Manually reset IP to 3 and demethylate
        vm.ip = (0, 3);

        // We need to execute demethylate.
        // Let's make a new DNA for demethylation test.

        let genes2 = vec![
            Gene { name: "push".to_string(), args: vec![Nucleotide::Number(0)] },
            Gene { name: "push".to_string(), args: vec![Nucleotide::Number(5)] },
            Gene { name: "demethylate".to_string(), args: vec![] },
        ];
        let mut vm2 = ChimeraVM::new(make_dna(genes2));

        // Manually methylate (0, 0) - The first push(0)
        // Wait, if I methylate (0, 0), it will be skipped.
        // Let's methylate a hypothetical gene (0, 5) that doesn't exist or is later.
        // Or better:
        // Gene 0: push(0)
        // Gene 1: push(5)
        // Gene 2: demethylate

        // Let's verify we can remove (0, 5) from epigenome.
        vm2.epigenome.insert((0, 5));

        vm2.step(); // push(0)
        vm2.step(); // push(5)
        vm2.step(); // demethylate removes (0, 5)

        assert!(!vm2.epigenome.contains(&(0, 5)));
    }
}
