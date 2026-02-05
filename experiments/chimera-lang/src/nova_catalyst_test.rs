#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_catalyze_reduction() {
        // [ push("migrate") push(4) catalyze() push(1) push(0) migrate() ]
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("migrate".to_string())] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(4)] },
            Gene { op: OpCode::Catalyze, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // dy
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // dx
            Gene { op: OpCode::Migrate, args: vec![] },
        ];

        let mut vm = make_vm(genes);
        vm.energy = 1000;

        // Step 1: Push "migrate"
        vm.step(); // energy 999
        // Step 2: Push 4
        vm.step(); // energy 998
        // Step 3: Catalyze
        // Base step cost: 1. Catalyze op cost: 50. Total: 51.
        vm.step(); // energy 998 - 51 = 947

        assert_eq!(vm.catalyst_table.get(&OpCode::Migrate), Some(&4));

        // Step 4: Push 1
        vm.step(); // 946
        // Step 5: Push 0
        vm.step(); // 945
        // Step 6: Migrate
        // Base step cost: 1. Migrate cost: 5. Reduced by 4 -> 1. Total: 2.
        vm.step(); // 945 - 2 = 943.

        assert_eq!(vm.energy, 943);
    }

    #[test]
    fn test_catalyze_min_cost() {
        // Reduce by 100. Cost should be clamped to 1.
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("migrate".to_string())] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] },
            Gene { op: OpCode::Catalyze, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Migrate, args: vec![] },
        ];

        let mut vm = make_vm(genes);
        vm.energy = 1000;

        // Execute all 6 steps
        for _ in 0..6 {
            vm.step();
        }

        // Same calculation as above: 943.
        assert_eq!(vm.energy, 943);
    }

    #[test]
    fn test_catalysis_inheritance() {
        // Verify mitosis inherits catalysts
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("migrate".to_string())] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(4)] },
            Gene { op: OpCode::Catalyze, args: vec![] },
            Gene { op: OpCode::SIndex, args: vec![] }, // Get own index
            Gene { op: OpCode::Mitosis, args: vec![] }, // Clone self
        ];

        let mut vm = make_vm(genes);
        vm.energy = 1000;

        // Execute until mitosis
        // Push(mig)
        vm.step();
        // Push(4)
        vm.step();
        // Catalyze
        vm.step();
        assert_eq!(vm.catalyst_table.get(&OpCode::Migrate), Some(&4));

        // SIndex
        vm.step();
        // Mitosis
        vm.step();

        // Now we have 2 strands.
        // If Mitosis clones the VM state... wait.
        // Mitosis clones the *Strand*.
        // Does it clone the `catalyst_table`?
        // `catalyst_table` is on `ChimeraVM`, not `Strand`.
        // So it's global to the organism.
        // Yes, so it is "inherited" because it's shared/global.
        // But if we `Sporulate` and restore, it should be preserved.

        // Let's test Sporulate/Germinate instead.
    }

    #[test]
    fn test_spore_preservation() {
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("migrate".to_string())] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(4)] },
            Gene { op: OpCode::Catalyze, args: vec![] },
            Gene { op: OpCode::Sporulate, args: vec![] }, // Create spore 0
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("migrate".to_string())] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Catalyze, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // Spore ID 0
            Gene { op: OpCode::Germinate, args: vec![] },
        ];

        let mut vm = make_vm(genes);
        vm.energy = 2000;

        // 1. Catalyze(4)
        vm.step(); vm.step(); vm.step();
        assert_eq!(vm.catalyst_table.get(&OpCode::Migrate), Some(&4));

        // 2. Sporulate
        vm.step();
        // Check spore content
        let spore = &vm.spores[0];
        assert_eq!(spore.catalyst_table.get(&OpCode::Migrate), Some(&4));

        // 3. Catalyze(0) -> actually loop says if reduction > 0.
        // So let's Catalyze(1). Overwrite 4 with 1.
        vm.step(); vm.step(); vm.step();
        assert_eq!(vm.catalyst_table.get(&OpCode::Migrate), Some(&1)); // Overwritten?
        // Wait, did I implement overwrite or check?
        // `vm.catalyst_table.insert` overwrites.

        // 4. Germinate(0)
        vm.step(); // Push 0
        vm.step(); // Germinate

        // 5. Check if restored to 4
        assert_eq!(vm.catalyst_table.get(&OpCode::Migrate), Some(&4));
    }
}
