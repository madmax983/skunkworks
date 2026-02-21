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
    fn test_hyphae_connect_transport() {
        // 1. Sprout at (8,8)
        // 2. Move to (8,9)
        // 3. Sprout at (8,9)
        // 4. Connect (8,8) [args: 8, 8]
        // 5. Transport 42 to (8,8) [args: 42, 8, 8]

        let genes = vec![
            // Sprout at start (8,8)
            Gene {
                op: OpCode::Hyphae,
                args: vec![],
            },
            // Move to (8,9)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // dy
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // dx
            Gene {
                op: OpCode::Migrate,
                args: vec![],
            },
            // Sprout at (8,9)
            Gene {
                op: OpCode::Hyphae,
                args: vec![],
            },
            // Connect to (8,8)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            }, // x
            Gene {
                op: OpCode::Connect,
                args: vec![],
            },
            // Transport 42 to (8,8)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(42)],
            }, // val
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            }, // x
            Gene {
                op: OpCode::Transport,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        // Give enough energy
        vm.energy = 1000;

        // Run simulation
        // 1. Hyphae (1 step)
        // 2. Push, Push, Migrate (3 steps)
        // 3. Hyphae (1 step)
        // 4. Push, Push, Connect (3 steps)
        // 5. Push, Push, Push, Transport (4 steps)
        // Total ~12 steps
        for _ in 0..20 {
            if vm.halted {
                break;
            }
            vm.step();
        }

        // Check grid at (8,8)
        assert_eq!(vm.grid[8][8], Value::Int(42));

        // Check Mycelium
        assert!(vm.mycelium.contains_key(&(8, 8)));
        assert!(vm.mycelium.contains_key(&(8, 9)));
        // Check connection
        let node = vm.mycelium.get(&(8, 9)).unwrap();
        assert!(node.connections.contains(&(8, 8)));
    }

    #[test]
    fn test_brainfuck() {
        // Code: ",." -> reads input, writes output
        // Input: "A"
        // Output on stack: "A"

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String(",.".to_string())],
            }, // Code
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("A".to_string())],
            }, // Input
            Gene {
                op: OpCode::Brainfuck,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 100;

        for _ in 0..10 {
            if vm.halted {
                break;
            }
            vm.step();
        }

        // Check stack
        // Stack should contain output string
        let res = vm.stack.pop().unwrap();
        assert_eq!(res, Value::Str("A".to_string()));
    }
}
