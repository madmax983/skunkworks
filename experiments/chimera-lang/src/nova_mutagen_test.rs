#[cfg(test)]
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
    fn test_irradiate() {
        // [ push(100) push(2) irradiate() sense_mutagen() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            }, // Amount
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            }, // Radius
            Gene {
                op: OpCode::Irradiate,
                args: vec![],
            },
            Gene {
                op: OpCode::SenseMutagen,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Step 1: Push 100
        vm.step();
        // Step 2: Push 2
        vm.step();
        // Step 3: Irradiate
        vm.step();

        // Center should have 100
        let val = vm.mutagen_grid[8][8];
        println!("Mutagen at center after irradiate: {}", val);
        assert_eq!(val, 100, "Expected 100 mutagen, got {}", val);

        // Step 4: Sense Mutagen
        // Note: process_environment runs BEFORE instruction, so mutagen will decay/diffuse.
        vm.step();

        if let Some(Value::Int(level)) = vm.stack.pop() {
            println!("Sensed mutagen level: {}", level);
            assert!(level < 100, "Level should decay");
            assert!(level > 50, "Level should remain high");
        } else {
            panic!("Expected mutagen level on stack");
        }
    }

    #[test]
    fn test_devour() {
        // [ push(100) push(1) irradiate() devour() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Irradiate,
                args: vec![],
            },
            Gene {
                op: OpCode::Devour,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        let _start_energy = vm.energy;

        // Execute irradiate (costs energy)
        vm.step();
        vm.step();
        vm.step();

        // Check radiation
        let val = vm.mutagen_grid[8][8];
        assert_eq!(val, 100, "Mutagen check failed");

        let energy_after_irradiate = vm.energy;

        // Execute devour
        // process_env runs before devour.
        // 100 decays to ~90.
        // Devour consumes ~90. Energy gain ~45.
        vm.step();

        // Mutagen should be gone
        assert_eq!(vm.mutagen_grid[8][8], 0);

        // Energy should increase
        assert!(vm.energy > energy_after_irradiate, "Energy should increase");
    }
}
