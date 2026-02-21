#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::nova_arcana::Arcana;
    use crate::vm::ChimeraVM;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_draw() {
        let genes = vec![Gene {
            op: OpCode::Draw,
            args: vec![],
        }];
        let mut vm = ChimeraVM::new(make_dna(genes));

        vm.step(); // Execute Draw

        assert!(vm.fate.active_arcana.is_some());
        assert_eq!(vm.fate.duration, 100);
        assert_eq!(vm.stack.len(), 1); // Pushes card ID
    }

    #[test]
    fn test_shuffle() {
        let genes = vec![
            Gene {
                op: OpCode::Draw,
                args: vec![],
            },
            Gene {
                op: OpCode::Draw, // Moves first card to discard
                args: vec![],
            },
            Gene {
                op: OpCode::Shuffle,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        vm.step(); // Draw 1
        vm.step(); // Draw 2 (1 goes to discard)

        assert!(!vm.fate.discard.is_empty());

        vm.step(); // Shuffle

        assert!(vm.fate.discard.is_empty()); // Discard should be empty
    }

    #[test]
    fn test_magician_effect() {
        // Magician adds energy
        // We force set the fate to Magician
        let genes = vec![Gene {
            op: OpCode::Push, // Does nothing but tick time
            args: vec![Nucleotide::Number(0)],
        }];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Inject Magician
        vm.fate.active_arcana = Some(Arcana::TheMagician);
        vm.fate.duration = 10;

        let initial_energy = vm.energy;
        vm.step(); // Push(0). Costs 1 base. Magician adds 2. Net +1.

        assert!(vm.energy > initial_energy);
    }

}
