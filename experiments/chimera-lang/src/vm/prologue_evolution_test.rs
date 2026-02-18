#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix, Strand, Gene, Nucleotide};
    use crate::opcode::OpCode;
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value};

    fn setup_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        // Push a dummy strand at index 0 because '!' rune treats Int(0) as empty signal.
        vm.dna.helix.strands.push(Strand { genes: vec![] });
        vm
    }

    #[test]
    fn test_evolution_length() {
        let mut vm = setup_vm();
        // Setup: Strand 1 has 2 genes.
        // 1 -> ! -> l

        let strand = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![] },
                Gene { op: OpCode::Add, args: vec![] },
            ],
        };
        vm.dna.helix.strands.push(strand); // Index 1

        vm.grid[5][4] = Value::Int(1);
        vm.grid[6][4] = Value::Str("!".to_string());
        vm.grid[6][5] = Value::Str("l".to_string());

        exec_prologue_tick(&mut vm);

        // Check l output at 6,5
        assert_eq!(vm.prologue_state.signal_grid[6][5], Some(Value::Int(2)));
    }

    #[test]
    fn test_evolution_nucleotide() {
        let mut vm = setup_vm();
        // Setup: Strand 1. Gene 1 is Add.
        // West (ID): 1. North (Idx): 1.
        // 1 -> ! -> n <- ! <- 1
        // n at 6,5.

        let strand = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![] },
                Gene { op: OpCode::Add, args: vec![] },
            ],
        };
        vm.dna.helix.strands.push(strand); // Index 1

        vm.grid[5][4] = Value::Int(1);
        vm.grid[6][4] = Value::Str("!".to_string()); // Source 1 at 6,4 (West)

        vm.grid[4][5] = Value::Int(1);
        vm.grid[5][5] = Value::Str("!".to_string()); // Source 1 at 5,5 (North)

        vm.grid[6][5] = Value::Str("n".to_string());

        exec_prologue_tick(&mut vm);

        // Check n output at 6,5. Should be "Add".
        if let Some(Value::Str(s)) = &vm.prologue_state.signal_grid[6][5] {
            assert_eq!(s, "Add");
        } else {
            panic!("Nucleotide failed, got {:?}", vm.prologue_state.signal_grid[6][5]);
        }
    }

    #[test]
    fn test_evolution_evolve() {
        let mut vm = setup_vm();
        // Setup: Strand 1.
        // 1 -> ! -> e
        // e at 6,5.

        let strand = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
            ],
        };
        vm.dna.helix.strands.push(strand); // Index 1

        vm.grid[5][4] = Value::Int(1);
        vm.grid[6][4] = Value::Str("!".to_string());
        vm.grid[6][5] = Value::Str("e".to_string());

        exec_prologue_tick(&mut vm);

        // e should output 1 (signal) to current grid, and New Index (2) to Delayed grid.
        assert_eq!(vm.prologue_state.signal_grid[6][5], Some(Value::Int(1)));
        assert_eq!(vm.prologue_state.delayed_signals[6][5], Some(Value::Int(2)));

        // DNA should have 3 strands (0=dummy, 1=original, 2=mutated)
        assert_eq!(vm.dna.helix.strands.len(), 3);
    }

    #[test]
    fn test_evolution_breed() {
        let mut vm = setup_vm();
        // Setup: Strand 1, Strand 2.
        // 1 -> ! -> b <- ! <- 2
        // b at 6,5.

        let strand0 = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            ],
        };
        let strand1 = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            ],
        };
        vm.dna.helix.strands.push(strand0); // Index 1
        vm.dna.helix.strands.push(strand1); // Index 2

        vm.grid[5][4] = Value::Int(1);
        vm.grid[6][4] = Value::Str("!".to_string()); // Source 1

        vm.grid[5][6] = Value::Int(2);
        vm.grid[6][6] = Value::Str("!".to_string()); // Source 2

        vm.grid[6][5] = Value::Str("b".to_string());

        exec_prologue_tick(&mut vm);

        // b should output New Index (3) to Delayed.
        assert_eq!(vm.prologue_state.delayed_signals[6][5], Some(Value::Int(3)));
        assert_eq!(vm.dna.helix.strands.len(), 4); // 0, 1, 2, 3

        // Crossover logic: 50/50 split.
        let child = &vm.dna.helix.strands[3];
        assert_eq!(child.genes.len(), 2);

        // First gene from Strand 1 (Push 0)
        assert_eq!(child.genes[0].args[0], Nucleotide::Number(0));
        // Second gene from Strand 2 (Push 1)
        assert_eq!(child.genes[1].args[0], Nucleotide::Number(1));
    }
}
