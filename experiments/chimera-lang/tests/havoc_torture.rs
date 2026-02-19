#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::ChimeraVM;
    use strum::IntoEnumIterator;
    use proptest::prelude::*;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    // Helper to convert index to OpCode
    fn index_to_opcode(i: usize) -> OpCode {
        let opcodes: Vec<OpCode> = OpCode::iter().collect();
        opcodes[i % opcodes.len()].clone()
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]
        #[test]
        fn test_vm_resilience(
            // Generate a vector of (op_index, arg1, arg2, arg_type_choice)
            instructions in prop::collection::vec((any::<usize>(), any::<i64>(), any::<String>(), any::<bool>()), 1..100)
        ) {
            let mut genes = Vec::new();
            for (op_idx, int_arg, str_arg, use_int) in instructions {
                let op = index_to_opcode(op_idx);
                let args = if use_int {
                    vec![Nucleotide::Number(int_arg)]
                } else {
                    vec![Nucleotide::String(str_arg)]
                };
                genes.push(Gene { op, args });
            }

            let mut vm = ChimeraVM::new(make_dna(genes));
            // Enable Havoc
            vm.havoc.rate = 0.1;
            vm.havoc.scope = 7;

            let mut steps = 0;
            while !vm.halted && steps < 100 {
                vm.step();
                steps += 1;
            }
            // If we reach here without panic, success.
        }
    }

    #[test]
    fn test_organelle_flood() {
        // 👺 HAVOC: Attempt to flood organelles beyond limit
        let genes = vec![
            Gene { op: OpCode::Spawn, args: vec![Nucleotide::Number(1), Nucleotide::Number(0)] },
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
        ];

        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Give infinite energy
        vm.energy = 1_000_000;

        let mut steps = 0;
        while !vm.halted && steps < 1000 {
            vm.step();
            steps += 1;
        }

        // Check if we crashed or just capped
        assert!(vm.organelles.len() <= chimera_lang::vm::MAX_ORGANELLES, "Organelle overflow!");
    }

    #[test]
    #[cfg(feature = "hive")]
    fn test_hive_flood() {
        use std::net::UdpSocket;
        use std::thread;

        // 👺 HAVOC: Flood Hive with packets
        // Find a free port
        let port = {
            let s = UdpSocket::bind("0.0.0.0:0").unwrap();
            s.local_addr().unwrap().port()
        };
        // s is dropped here, port is released. Small race condition possible but acceptable for chaos test.

        // VM Code: Bind port, Recv loop
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(port as i64)] },
            Gene { op: OpCode::HiveBind, args: vec![] },
            // Loop
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(port as i64)] },
            Gene { op: OpCode::HiveRecv, args: vec![] },
            Gene { op: OpCode::Drop, args: vec![] }, // Discard result
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(2)] },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));

        // Spawn flooder thread
        let flooder = thread::spawn(move || {
            // Wait slightly for VM to bind
            thread::sleep(std::time::Duration::from_millis(50));
            let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
            let target = format!("127.0.0.1:{}", port);
            for _ in 0..1000 {
                let _ = socket.send_to(b"{\"havoc\": true, \"nested\": [[[]]]}", &target);
                // Also send garbage
                let _ = socket.send_to(b"GARBAGE_DATA_NOT_JSON", &target);
            }
        });

        let mut steps = 0;
        while !vm.halted && steps < 2000 {
            vm.step();
            steps += 1;
        }

        flooder.join().unwrap();
    }
}
