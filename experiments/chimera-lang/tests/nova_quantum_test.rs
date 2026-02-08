use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::{ChimeraVM, Value};

#[test]
fn test_quantum_jump() {
    // Setup:
    // Strand 0: [ Push(1), Push(0), Entangle(), QuantumJump() ]
    // Strand 1: [ Push(999) ]

    let strand0 = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)], // Partner
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // Self
            },
            Gene {
                op: OpCode::Entangle,
                args: vec![],
            },
            Gene {
                op: OpCode::QuantumJump,
                args: vec![],
            },
        ],
    };

    let strand1 = Strand {
        genes: vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(999)],
        }],
    };

    let dna = Dna {
        helix: Helix {
            strands: vec![strand0, strand1],
        },
    };

    let mut vm = ChimeraVM::new(dna);

    // Step 1: Push(1)
    vm.step();
    // Step 2: Push(0)
    vm.step();
    // Step 3: Entangle
    vm.step();

    // IP should be (0, 3) - pointing to QuantumJump
    assert_eq!(vm.ip, (0, 3));
    assert!(vm.entangled_pairs.contains_key(&0));
    assert_eq!(vm.entangled_pairs[&0], 1);

    // Step 4: QuantumJump
    vm.step();

    // Logic:
    // QuantumJump returns Some((1, target)).
    // target calculation: gene_idx = 3. p_len = 1.
    // 3 < 1 is false. target = 0 (saturated sub).
    // New IP = (1, 0).
    assert_eq!(vm.ip, (1, 0));

    // Step 5: Execute Strand 1 (Push 999)
    vm.step();
    assert_eq!(vm.stack.last(), Some(&Value::Int(999)));
}

#[test]
fn test_spirit_message() {
    // Strand 0: [ Push("Hello"), Spirit() ]
    // Note: Spirit uses stack for message?
    // Let's check implementation:
    /*
        OpCode::Spirit => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(msg) = val {
                    vm.spirit_message = Some(msg);
                } else {
                    vm.stack.push(val);
                    vm.spirit_message = None;
                }
            }
            vm.spirit_request = true;
            None
        }
    */
    // Yes, it pops stack.

    let strand0 = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("Hello Spirit".to_string())],
            },
            Gene {
                op: OpCode::Spirit,
                args: vec![],
            },
        ],
    };

    let dna = Dna {
        helix: Helix {
            strands: vec![strand0],
        },
    };

    let mut vm = ChimeraVM::new(dna);

    // Push
    vm.step();

    // Spirit
    vm.step();

    assert!(vm.spirit_request);
    assert_eq!(vm.spirit_message, Some("Hello Spirit".to_string()));

    // Mock user input
    vm.spirit_value = Some(Value::Int(42));

    // Step (Resume)
    vm.step();

    assert!(!vm.spirit_request);
    assert_eq!(vm.stack.last(), Some(&Value::Int(42)));
}
