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
    fn test_thermodynamics_opcodes() {
        // [ exothermic(100, 1) thermometer() endothermic(50, 1) thermometer() state() ]
        let genes = vec![
            // Exothermic(100, 1) -> Stack: [100, 1]
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] }, // amount
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // radius
            Gene { op: OpCode::Exothermic, args: vec![] },

            // Thermometer
            Gene { op: OpCode::Thermometer, args: vec![] },

            // Endothermic(50, 1) -> Stack: [50, 1]
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(50)] }, // amount
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // radius
            Gene { op: OpCode::Endothermic, args: vec![] },

            // Thermometer
            Gene { op: OpCode::Thermometer, args: vec![] },

            // State
            Gene { op: OpCode::State, args: vec![] },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 1000; // Give plenty of energy

        // We run enough steps to execute all genes.
        for _ in 0..20 {
            if vm.halted { break; }
            vm.step();
        }

        assert_eq!(vm.stack.len(), 3, "Stack: {:?}", vm.stack);

        let t1 = match vm.stack[0] { Value::Int(v) => v, _ => 0 };
        let t2 = match vm.stack[1] { Value::Int(v) => v, _ => 0 };
        let state = match vm.stack[2] { Value::Int(v) => v, _ => -1 };

        assert!(t1 > 50, "Expected T1 > 50, got {}", t1);
        assert!(t2 < t1, "Expected T2 < T1, got {} < {}", t2, t1);
        // Liquid state (1) is 0 < T <= 100.
        // If T2 is around 70, state should be 1.
        assert_eq!(state, 1, "Expected Liquid state for T={}", t2);
    }

    #[test]
    fn test_phase_change_boiling() {
        // Boil water -> Steam
        let genes = vec![
            // Heat it up massively!
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1000)] }, // amount
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // radius
            Gene { op: OpCode::Exothermic, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] }, // Loop
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 1000;
        let (cy, cx) = vm.context_loc;
        vm.moisture_grid[cy][cx] = 100;

        for _ in 0..5 {
            vm.step();
        }

        assert!(vm.temperature_grid[cy][cx] > 100, "Temp was {}", vm.temperature_grid[cy][cx]);
        match &vm.grid[cy][cx] {
            Value::Str(s) => assert_eq!(s, "Steam"),
            _ => panic!("Expected Steam string, got {:?}", vm.grid[cy][cx]),
        }
        assert!(vm.moisture_grid[cy][cx] < 100);
    }

    #[test]
    fn test_phase_change_freezing() {
        // Freeze water -> Ice
        let genes = vec![
            // Cool it down significantly to fight ambient warming
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(500)] }, // amount
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // radius
            Gene { op: OpCode::Endothermic, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] }, // Loop
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 1000;
        let (cy, cx) = vm.context_loc;
        vm.moisture_grid[cy][cx] = 50;
        vm.grid[cy][cx] = Value::Str("Water".to_string());

        for _ in 0..5 {
            vm.step();
        }

        assert!(vm.temperature_grid[cy][cx] < 0, "Temp was {}", vm.temperature_grid[cy][cx]);
        match &vm.grid[cy][cx] {
            Value::Str(s) => assert_eq!(s, "Ice"),
            _ => panic!("Expected Ice string, got {:?}", vm.grid[cy][cx]),
        }
    }
}
