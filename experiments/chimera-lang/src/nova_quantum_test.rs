#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_entanglement() {
        // 1. Entangle (8,8) [context default] with (5,5)
        // 2. Write 42 to (8,8)
        // 3. Read (5,5) -> Should be 42
        let genes = vec![
            // Entangle context (8,8) with (5,5)
            // stack: y, x -> 5, 5
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                name: "entangle".to_string(),
                args: vec![],
            },
            // Write 42 to (8,8)
            // stack: val, y, x -> 42, 8, 8
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(42)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                name: "g_write".to_string(),
                args: vec![],
            },
            // Read (5,5)
            // stack: y, x -> 5, 5
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                name: "g_read".to_string(),
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        // Expect stack top to be 42
        if let Some(val) = vm.stack.pop() {
            match val {
                Value::Int(n) => assert_eq!(
                    n, 42,
                    "Entanglement failed: Expected 42 at (5,5), got {}",
                    n
                ),
                _ => panic!("Expected Int(42), got {:?}", val),
            }
        } else {
            panic!("Stack underflow: g_read failed");
        }
    }

    #[test]
    fn test_decoherence() {
        // 1. Entangle (8,8) with (5,5)
        // 2. Decohere (8,8)
        // 3. Write 99 to (8,8)
        // 4. Read (5,5) -> Should be 0 (default)
        let genes = vec![
            // Entangle
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                name: "entangle".to_string(),
                args: vec![],
            },
            // Decohere
            Gene {
                name: "decohere".to_string(),
                args: vec![],
            },
            // Write 99 to (8,8)
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(99)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                name: "g_write".to_string(),
                args: vec![],
            },
            // Read (5,5)
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                name: "g_read".to_string(),
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        if let Some(val) = vm.stack.pop() {
            match val {
                Value::Int(n) => {
                    assert_eq!(n, 0, "Decoherence failed: Expected 0 at (5,5), got {}", n)
                }
                _ => panic!("Expected Int(0), got {:?}", val),
            }
        }
    }
}
