#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_gravitate() {
        // Setup:
        // Center at (8, 8).
        // Place items at (8, 10) [dist 2] and (8, 11) [dist 3].
        // Gravitate radius 5.
        // Expected:
        // (8, 10) moves to (8, 9).
        // (8, 11) moves to (8, 10).

        let mut vm = ChimeraVM::new(make_dna(vec![
            // Move context to 8,8 (default start is 8,8 so no need to move)

            // Write 100 at 8,10
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            }, // x
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            // Write 200 at 8,11
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(200)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(11)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            }, // x
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            // Gravitate radius 5
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Gravitate,
                args: vec![],
            },
        ]));

        // Run
        while !vm.halted && vm.ip.1 < 10 {
            // 10 instructions
            vm.step();
        }

        // Verify positions
        // 8,10 should be empty (moved)
        // 8,9 should be 100
        // 8,11 should be empty
        // 8,10 should be 200

        // Check 8,9
        if let Value::Int(n) = vm.grid[9][8] {
            // y=9, x=8
            assert_eq!(n, 100, "Item at 8,10 should move to 8,9");
        } else {
            panic!("Expected Int at 8,9");
        }

        // Check 8,10
        if let Value::Int(n) = vm.grid[10][8] {
            // y=10, x=8
            assert_eq!(n, 200, "Item at 8,11 should move to 8,10");
        } else {
            panic!("Expected Int at 8,10");
        }

        // Check 8,11
        if let Value::Int(n) = vm.grid[11][8] {
            assert_eq!(n, 0, "Item at 8,11 should be gone");
        }
    }

    #[test]
    fn test_gravitate_blocked() {
        // Setup:
        // Item at 8,9 (dist 1)
        // Item at 8,10 (dist 2)
        // Gravitate.
        // 8,9 tries to move to 8,8. 8,8 is center? 8,8 is empty?
        // Center is 8,8. Can items move onto center?
        // My implementation says: `if dist_sq == 0 { continue; }` so center itself is skipped as source.
        // But can it be a target?
        // `target_x` calculation: `cx - tx`
        // If at 8,9, `dy = 8 - 9 = -1`. `sy = -1`. `target_y = 9 - 1 = 8`.
        // Target is 8,8.
        // If 8,8 is empty (0), it moves there.

        // Let's block 8,8 with something.

        let mut vm = ChimeraVM::new(make_dna(vec![
            // Write 999 at 8,8 (Center)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(999)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            // Write 100 at 8,9
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(9)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            // Write 200 at 8,10
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(200)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            // Gravitate radius 5
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Gravitate,
                args: vec![],
            },
        ]));

        while !vm.halted && vm.ip.1 < 12 {
            vm.step();
        }

        // 8,8 is occupied. 8,9 cannot move.
        // 8,9 is occupied. 8,10 cannot move.

        assert_eq!(vm.grid[8][8], Value::Int(999));
        assert_eq!(vm.grid[9][8], Value::Int(100));
        assert_eq!(vm.grid[10][8], Value::Int(200));
    }
}
