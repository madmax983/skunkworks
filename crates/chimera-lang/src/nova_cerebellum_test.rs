#![cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(strands: Vec<Vec<Gene>>) -> Dna {
        let strands = strands.into_iter().map(|genes| Strand { genes }).collect();
        Dna { evolution_config: None,
            helix: Helix { strands },
        }
    }

    fn gene(name: &str, arg: Option<i64>) -> Gene {
        Gene {
            op: name.parse().unwrap(),
            args: if let Some(n) = arg {
                vec![Nucleotide::Number(n)]
            } else {
                vec![]
            },
        }
    }

    #[test]
    fn test_call_ret() {
        // Strand 0: [ push(100), call(1), push(300) ]
        // Strand 1: [ push(200), ret() ]

        let s0 = vec![
            gene("push", Some(100)),
            gene("call", Some(1)),
            gene("push", Some(300)),
        ];

        let s1 = vec![gene("push", Some(200)), gene("ret", None)];

        let dna = make_dna(vec![s0, s1]);
        let mut vm = ChimeraVM::new(dna);

        // Step 1: push(100)
        vm.step();
        assert_eq!(vm.stack.last(), Some(&Value::Int(100)));

        // Step 2: call(1)
        vm.step();
        assert_eq!(vm.ip, (1, 0));
        assert_eq!(vm.call_stack.len(), 1);
        assert_eq!(vm.call_stack[0], (0, 2)); // Return to strand 0, index 2 (push 300)

        // Step 3: push(200)
        vm.step();
        assert_eq!(vm.stack.last(), Some(&Value::Int(200)));

        // Step 4: ret()
        vm.step();
        assert_eq!(vm.ip, (0, 2)); // Back to caller
        assert_eq!(vm.call_stack.len(), 0);

        // Step 5: push(300)
        vm.step();
        assert_eq!(vm.stack.last(), Some(&Value::Int(300)));
    }

    #[test]
    fn test_bind_interrupt() {
        // Bind 'a' (97) to Strand 1.
        // Strand 0: [ push(97), push(1), bind(), push(0), jump(3) ]
        // Strand 1: [ push(100), ret() ]

        // 'a' is 97 in ASCII.
        let s0 = vec![
            gene("push", Some(97)), // 0
            gene("push", Some(1)),  // 1
            gene("bind", None),     // 2
            gene("push", Some(0)),  // 3
            gene("jump", Some(3)),  // 4
        ];

        let s1 = vec![gene("push", Some(100)), gene("ret", None)];

        let dna = make_dna(vec![s0, s1]);
        let mut vm = ChimeraVM::new(dna);

        // Step 1: push(97)
        vm.step();
        // Step 2: push(1)
        vm.step();
        // Step 3: bind
        vm.step();
        assert!(vm.receptors.contains_key(&'a'));

        // Step 4: push(0)
        vm.step();

        // Inject Input
        vm.handle_input('a');
        assert!(!vm.input_buffer.is_empty());

        // Step 5: Should process input buffer, interrupt, and jump to Strand 1
        // Current IP is (0, 4) [jump(3)].
        // Interrupt pushes (0, 4) to call stack.
        // Sets IP to (1, 0).
        // THEN it executes the instruction at (1, 0) immediately!
        // Instruction at (1, 0) is push(100).
        vm.step();

        // IP should have advanced past push(100) -> (1, 1)
        assert_eq!(vm.ip, (1, 1));
        assert_eq!(vm.call_stack.len(), 1);
        assert_eq!(vm.call_stack[0], (0, 4));
        assert_eq!(vm.stack.last(), Some(&Value::Int(100)));

        // Step 6: ret()
        vm.step();
        assert_eq!(vm.ip, (0, 4)); // Back to caller
    }
}
