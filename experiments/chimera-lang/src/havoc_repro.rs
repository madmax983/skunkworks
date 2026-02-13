#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_junction_blowup() {
        // Attack: Exponential Junction Blowup via OpCode::Add (Cross Product)
        // 1. Create a Junction of size 2: [1, 1]
        // 2. Loop: Dup, Add.
        // Size sequence: 2 -> 4 -> 16 -> 256 -> 65536 -> 4,294,967,296 (4B) -> OOM

        let genes = vec![
            // Init: Push(1)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            // Make it [1, 1] using Map("[ dup() ]")
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("[ dup() ]".to_string())],
            },
            Gene {
                op: OpCode::Map,
                args: vec![],
            },
            // Iteration 1: 2 -> 4
            Gene {
                op: OpCode::Dup,
                args: vec![],
            },
            Gene {
                op: OpCode::Add,
                args: vec![],
            },
            // Iteration 2: 4 -> 16
            Gene {
                op: OpCode::Dup,
                args: vec![],
            },
            Gene {
                op: OpCode::Add,
                args: vec![],
            },
            // Iteration 3: 16 -> 256
            Gene {
                op: OpCode::Dup,
                args: vec![],
            },
            Gene {
                op: OpCode::Add,
                args: vec![],
            },
            // Iteration 4: 256 -> 65536 (Wait, 65536 is > 1024, so it should be blocked here)
            Gene {
                op: OpCode::Dup,
                args: vec![],
            },
            Gene {
                op: OpCode::Add,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));

        let mut steps = 0;
        while !vm.halted && steps < 1000 {
            vm.step();
            steps += 1;
        }

        // Verify we hit the complexity limit
        let has_error = vm
            .output
            .iter()
            .any(|s| s.contains("complexity limit") || s.contains("limit exceeded"));
        assert!(
            has_error,
            "Expected complexity/size limit error, got: {:?}",
            vm.output
        );
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_hyperbolic_virus_singularity() {
        // 👺 HAVOC: Triggering Hyperbolic Singularity via Virus-injected Geometry
        // 1. Set Topology to Hyperbolic (6).
        // 2. Use Virus to execute Migrate at (15, 15).
        //    (15, 15) maps to (0.95, 0.95) which has norm > 1.3 in Euclidean,
        //    but grid_to_disk allows it (mapping square to "disk" space naively).
        // 3. Migrate(-5, -5) creates displacement a=(-0.5, -0.5).
        //    This aligns with z=(0.95, 0.95) to minimize denominator |1 + a'z|.
        //    1 + (-0.5+0.5i)(0.95+0.95i) = 1 + (-0.95) = 0.05.
        //    This amplifies the result.
        //    If we tune it closer, we might get infinity.

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(6)],
            }, // Hyperbolic
            Gene {
                op: OpCode::Shape,
                args: vec![],
            },
            // Stack for Virus: [ "Migrate", dy, dx ] -> Virus pops y, x.
            // Virus executes "Migrate". Migrate expects [dy, dx].
            // So we need to push [dy, dx] BEFORE calling Virus?
            // No, Virus executes an opcode. Migrate pops from the VM stack.
            // So we need stack to be [..., dy, dx, "Migrate", y, x] (top)
            // Virus pops x, y. Pops "Migrate". Executes it.
            // Migrate pops dx, dy.
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(-5)],
            }, // dy
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(-5)],
            }, // dx
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("Migrate".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(15)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(15)],
            }, // x
            Gene {
                op: OpCode::Virus,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));

        // Run
        let mut steps = 0;
        while !vm.halted && steps < 100 {
            vm.step();
            steps += 1;
        }

        println!("VM Output: {:?}", vm.output);

        // Assert that we didn't crash (if we crashed, test fails with panic, which is also a win)
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_scavenge_path_traversal() {
        // 👺 HAVOC: Path Traversal via Scavenge
        // 1. Target a file known to exist outside the likely sandbox (../../Cargo.toml relative to experiments/chimera-lang)
        // 2. Call Scavenge on it.
        // 3. Assert that access is DENIED (secure).

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("../../Cargo.toml".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            },
            Gene {
                op: OpCode::Scavenge,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.step(); // Push path
        vm.step(); // Push len
        vm.step(); // Scavenge

        // If secure, output should contain "Error".
        // If vulnerable, output should contain "SCAVENGE: Consumed".

        let vulnerable = vm.output.iter().any(|s| s.contains("SCAVENGE: Consumed"));
        if vulnerable {
             println!("VM Output: {:?}", vm.output);
        }
        assert!(!vulnerable, "SECURITY BREACH: Path traversal detected! Able to read ../../Cargo.toml");
    }
}
