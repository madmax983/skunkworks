#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_rune_inscribe_read() {
        // [ push("F") rune_inscribe() push(0) push(0) rune_read() ]
        // Should inscribe "Rune:F" at (0,0) (default context_loc is (8,8) usually but let's check)
        // context_loc in new() is (8,8).
        // Let's set context_loc or just read from (8,8).

        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("F".to_string())] },
            Gene { op: OpCode::RuneInscribe, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] }, // y
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] }, // x
            Gene { op: OpCode::RuneRead, args: vec![] },
        ];
        let mut vm = make_vm(genes);

        vm.step(); // push
        vm.step(); // inscribe

        if let Value::Str(s) = &vm.grid[8][8] {
            assert_eq!(s, "Rune:F");
        } else {
            panic!("Rune not inscribed");
        }

        vm.step(); // push y
        vm.step(); // push x
        vm.step(); // read

        let val = vm.stack.pop().unwrap();
        if let Value::Str(s) = val {
            assert_eq!(s, "F");
        } else {
            panic!("Rune read failed");
        }
    }

    #[test]
    fn test_rune_invoke_fehu() {
        // [ push("F") rune_inscribe() push("F") rune_invoke() ]
        // Fehu adds 10 energy.
        // Inscribe costs 10. Invoke costs nothing explicit but effect +10.
        // Start 50. Inscribe -> 40. Invoke -> 50.
        // Step costs 1 per step.

        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("F".to_string())] },
            Gene { op: OpCode::RuneInscribe, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("F".to_string())] },
            Gene { op: OpCode::RuneInvoke, args: vec![] },
        ];
        let mut vm = make_vm(genes);
        let _start_energy = vm.energy; // 50

        vm.step(); // push "F"
        vm.step(); // inscribe (cost 10) -> 39 (tick cost 1)

        let energy_after_inscribe = vm.energy;
        // Inscribe cost 10, plus step cost 1.
        // vm.energy should be around 39 or 38 depending on when cost is applied.
        // step() applies energy -= 1 first.

        vm.step(); // push "F"
        vm.step(); // invoke

        // Invoke Fehu adds 10.
        let end_energy = vm.energy;

        assert!(end_energy > energy_after_inscribe);
    }

    #[test]
    fn test_rune_sense() {
        // [ push("F") rune_inscribe() push(1) push(1) migrate() rune_sense() ]
        // Inscribe at (8,8). Move to (9,9). Sense should find F at (-1, -1).

        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("F".to_string())] },
            Gene { op: OpCode::RuneInscribe, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // dy
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // dx
            Gene { op: OpCode::Migrate, args: vec![] },
            Gene { op: OpCode::RuneSense, args: vec![] },
        ];
        let mut vm = make_vm(genes);

        vm.step(); // push
        vm.step(); // inscribe
        vm.step(); // push
        vm.step(); // push
        vm.step(); // migrate -> (9,9)
        vm.step(); // sense

        // Stack: rune, dx, dy (pushed in reverse order: dy, dx, rune)
        // Code: vm.stack.push(dy); vm.stack.push(dx); vm.stack.push(rune);
        // Pop: rune, dx, dy

        let rune = vm.stack.pop().unwrap();
        let dx = vm.stack.pop().unwrap();
        let dy = vm.stack.pop().unwrap();

        assert_eq!(rune, Value::Str("F".to_string()));
        assert_eq!(dx, Value::Int(-1));
        assert_eq!(dy, Value::Int(-1));
    }

    #[test]
    fn test_rune_link() {
        // Link F -> K. Invoke F. Check Light (K effect).
        // [ push("F") rune_inscribe() push("F") push("K") rune_link() push("F") rune_invoke() ]

        // NOTE: K needs to be ON THE GRID to be triggered by link?
        // Logic: "RuneLink: Links two Runes. When rune_a is invoked, rune_b is also invoked."
        // And RuneInvoke: "Invoking ... instances of ..."
        // So yes, K needs to be inscribed somewhere to have an effect.

        // So: Inscribe F at (8,8). Inscribe K at (8,9). Link F->K. Invoke F.

        let genes = vec![
            // Inscribe F
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("F".to_string())] },
            Gene { op: OpCode::RuneInscribe, args: vec![] },

            // Move to (8,9)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Migrate, args: vec![] },

            // Inscribe K
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("K".to_string())] },
            Gene { op: OpCode::RuneInscribe, args: vec![] },

            // Link F -> K
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("F".to_string())] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("K".to_string())] },
            Gene { op: OpCode::RuneLink, args: vec![] },

            // Invoke F
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("F".to_string())] },
            Gene { op: OpCode::RuneInvoke, args: vec![] },
        ];

        let len = genes.len();
        let mut vm = make_vm(genes);

        while !vm.halted && vm.ip.1 < len {
            vm.step();
        }

        // Check Light at (8,9)
        // K adds 100 light.
        assert!(vm.light_grid[8][9] >= 100);
    }
}
