#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value, GRID_SIZE};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_red_tape() {
        // [ push(50) red_tape() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(50)],
            },
            Gene {
                op: OpCode::RedTape,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Initial state: 0
        // Default context_loc is (8, 8)
        assert_eq!(vm.bureaucracy_grid[8][8], 0);

        vm.step(); // push(50)
        vm.step(); // red_tape()

        assert_eq!(vm.bureaucracy_grid[8][8], 50);
    }

    #[test]
    fn test_bureaucracy_drain() {
        // [ push(100) red_tape() jump(2) ]
        // Should drain energy faster.
        // red_tape() increases grid to 100.
        // process_red_tape() drains 5 energy per tick if level > 50.
        // jump(2) loops to itself.

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            },
            Gene {
                op: OpCode::RedTape,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(2)], // Jump to self (index 2)
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        vm.step(); // push(100). Energy: 50 -> 49
        vm.step(); // red_tape(). Energy: 49 -> 48. Grid[8][8] = 100.

        let energy_before_jump = vm.energy;
        vm.step(); // jump(2). Energy should drop by 1 (base) + 5 (drain) = 6.
        let energy_after_jump = vm.energy;

        assert_eq!(
            energy_before_jump - energy_after_jump,
            6,
            "Expected drain of 6 (1 base + 5 red tape)"
        );
    }

    #[test]
    fn test_form_sign_permit_immunity() {
        // [ push(100) red_tape() push(10) form() sign() permit() jump(6) ]
        // 0: push(100)
        // 1: red_tape() -> grid=100
        // 2: push(10)
        // 3: form() -> grid=90, stack=["Form:10"] (consumes 5 energy)
        // 4: sign() -> stack=["Permit:10"] (consumes 2 energy)
        // 5: permit() -> Adds "Bureaucrat" buff (100 ticks)
        // 6: jump(6) -> Loops. Immune to drain.

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            },
            Gene {
                op: OpCode::RedTape,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Form,
                args: vec![],
            },
            Gene {
                op: OpCode::Sign,
                args: vec![],
            },
            Gene {
                op: OpCode::Permit,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(6)],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));

        // Give enough energy to survive the setup costs
        vm.energy = 1000;

        for _ in 0..6 {
            vm.step();
        }

        // After permit(), we should have immunity
        assert!(vm.buffs.contains_key("Bureaucrat"), "Buff should be active");

        // Grid is still 90, so drain WOULD be 5.
        assert_eq!(vm.bureaucracy_grid[8][8], 90);

        let energy_before_loop = vm.energy;
        vm.step(); // jump(6)
        let energy_after_loop = vm.energy;

        // Should only lose 1 energy (base cost) because of immunity
        assert_eq!(
            energy_before_loop - energy_after_loop,
            1,
            "Expected only base energy cost (1) due to immunity"
        );
    }
}
