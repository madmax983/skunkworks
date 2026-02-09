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

    #[test]
    fn test_emperor_effect() {
        // Emperor reduces Red Tape
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::RedTape,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Inject Emperor
        vm.fate.active_arcana = Some(Arcana::TheEmperor);
        vm.fate.duration = 10;

        vm.step(); // Push 10
        vm.step(); // RedTape -> Grid[8][8] += 10.

        // vm.step() order: Fate (passive) runs BEFORE Gene Execution?
        // I put nova_arcana::process_fate(self) BEFORE gene execution loop.
        // So:
        // Tick 1 (Push 10): Fate runs (Grid=0). Execute Push.
        // Tick 2 (RedTape): Fate runs (Grid=0). Execute RedTape (Grid=10).
        // Tick 3 (Push 0): Fate runs (Grid=10 -> 5). Execute Push.

        // We need to verify that grid is reduced on Tick 3.

        // Run until tick 2 completed.
        // Initial energy = 50.
        // Tick 1: E=49.
        // Tick 2: E=48.

        assert_eq!(vm.bureaucracy_grid[8][8], 10);

        vm.step(); // Tick 3.

        assert_eq!(vm.bureaucracy_grid[8][8], 5);
    }
}
