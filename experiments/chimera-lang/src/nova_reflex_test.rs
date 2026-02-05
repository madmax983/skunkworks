#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Topology, Value};

    fn make_dna(strands: Vec<Vec<Gene>>) -> Dna {
        let strands = strands.into_iter().map(|genes| Strand { genes }).collect();
        Dna {
            helix: Helix { strands },
        }
    }

    #[test]
    fn test_reflex_collision() {
        // Strand 0: Register reflex for Collision (0) to Strand 1, then try to migrate into wall
        let strand0 = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // Strand 1
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // Event 0
            Gene {
                op: OpCode::Reflex,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // dy = 0
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // dx = 1 (East)
            Gene {
                op: OpCode::Migrate,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(0)],
            }, // Loop
        ];

        // Strand 1: Handler. Push 999 to indicate success. Return.
        let strand1 = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(999)],
            },
            Gene {
                op: OpCode::Ret,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(vec![strand0, strand1]));

        // Setup: Place at East edge (0, 15) and set Plane topology (walls)
        vm.context_loc = (0, 15);
        vm.topology = Topology::Plane;

        // Step 1: push(1)
        vm.step();
        // Step 2: push(0)
        vm.step();
        // Step 3: Reflex
        vm.step();
        assert!(vm.reflexes.contains_key(&0));

        // Step 4: push(0)
        vm.step();
        // Step 5: push(1)
        vm.step();

        // Step 6: Migrate (Hits wall) -> Trigger Reflex -> Jump to Strand 1
        vm.step();

        assert_eq!(vm.ip.0, 1);

        // Step 7: Execute handler (Push 999)
        vm.step();
        assert_eq!(vm.stack.last(), Some(&Value::Int(999)));

        // Step 8: Ret -> Back to Strand 0
        vm.step();
        assert_eq!(vm.ip.0, 0);
    }

    #[test]
    fn test_reflex_low_energy() {
        // Strand 0: Register reflex for Low Energy (2) to Strand 1. Loop.
        let strand0 = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::Reflex,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(3)],
            }, // Infinite loop to drain energy
        ];

        // Strand 1: Handler. Photosynthesize (+5). Ret.
        let strand1 = vec![
            Gene {
                op: OpCode::Photosynthesize,
                args: vec![],
            },
            Gene {
                op: OpCode::Ret,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(vec![strand0, strand1]));

        // Setup: Set energy high enough to setup
        vm.energy = 20;

        // Step 1: push(1)
        vm.step();
        // Step 2: push(2)
        vm.step();
        // Step 3: Reflex
        vm.step();
        assert!(vm.reflexes.contains_key(&2));

        // Now drain energy manually to 10
        vm.energy = 10;

        // Next step should trigger reflex (energy -> 9 < 10)
        // Step 4: Jump(3) -> Trigger -> Jump to Strand 1
        vm.step();

        assert_eq!(vm.ip.0, 1);

        // Execute handler: Photosynthesize
        vm.step();

        // Energy should be > 9 (9 - 1 + 5 = 13)
        assert!(vm.energy > 9);
    }

    #[test]
    fn test_reflex_mutation() {
        // Strand 0: Register reflex for Mutation (1) to Strand 1.
        let strand0 = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Reflex,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(3)],
            }, // Wait
        ];

        // Strand 1: Handler. Push 777. Ret.
        let strand1 = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(777)],
            },
            Gene {
                op: OpCode::Ret,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(vec![strand0, strand1]));

        // Setup reflex
        vm.step(); // push
        vm.step(); // push
        vm.step(); // reflex

        // Manually trigger mutation until it succeeds
        let old_ip = vm.ip;
        for _ in 0..100 {
            vm.mutate();
            if vm.ip != old_ip {
                break;
            }
        }

        // Should have triggered reflex and jumped to Strand 1
        assert_eq!(vm.ip.0, 1);

        // Run step to execute handler
        vm.step();
        assert_eq!(vm.stack.last(), Some(&Value::Int(777)));

        // Ret
        vm.step();
        assert_eq!(vm.ip.0, 0);
    }
}
