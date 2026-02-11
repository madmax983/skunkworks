#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix, JunctionType};
    use crate::vm::{ChimeraVM, Value};
    use crate::vm::nova_botany::exec_plant;

    use crate::ast::{Gene, Nucleotide, Strand};
    use crate::opcode::OpCode;

    fn make_vm() -> ChimeraVM {
        // Create a dummy strand to keep VM alive
        let strand = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Jump,
                    args: vec![Nucleotide::Number(0)],
                },
            ],
        };
        let dna = Dna {
            helix: Helix {
                strands: vec![strand],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_bio_architect() {
        let mut vm = make_vm();

        // 1. Setup Stack for plant()
        // Stack: [mapping], rules, axiom

        // Mapping: Junction(Any, [Str("M:F=*"), Str("M:G=A")])
        // F -> * (Bang)
        // G -> A (Add)
        let mapping = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("M:F=*".to_string()),
                Value::Str("M:G=A".to_string()),
            ]
        );

        // Rules: X -> F+G
        let rules = Value::Str("X=F+G".to_string());

        // Axiom: X
        let axiom = Value::Str("X".to_string());

        vm.stack.push(mapping);
        vm.stack.push(rules);
        vm.stack.push(axiom);

        // 2. Exec plant
        // This should spawn a Seed at context_loc (8,8)
        vm.context_loc = (8, 8);
        exec_plant(&mut vm);

        assert_eq!(vm.organelles.len(), 1);
        let seed = &mut vm.organelles[0];
        assert!(seed.traits.contains(&"M:F=*".to_string()));
        assert!(seed.traits.contains(&"M:G=A".to_string()));

        // 3. Step the VM to let Seed grow
        // Seed ticks once per VM step (if time_grid is 1)

        // Tick 1: X expands to F+G. Index stays 0.
        // Tick 2: F (No rule). Interprets as *. Write * at (8,8). Move East to (8,9). Index 1.
        // Tick 3: + (No rule). Turns Right (South). Dir (1,0). Index 2.
        // Tick 4: G (No rule). Interprets as A. Write A at (8,9). Move South to (9,9). Index 3.
        // Tick 5: End. Seed dies/halts.

        for _ in 0..10 {
            // Manually process organelles to avoid main loop noise
            // But vm.step() handles it.
            vm.step();
        }

        // 4. Verify Grid

        // (8,8) should be "*"
        match &vm.grid[8][8] {
            Value::Str(s) => assert_eq!(s, "*"),
            _ => panic!("Expected * at 8,8, got {:?}", vm.grid[8][8]),
        }

        // (8,9) should be "A"
        match &vm.grid[8][9] {
            Value::Str(s) => assert_eq!(s, "A"),
            _ => panic!("Expected A at 8,9, got {:?}", vm.grid[8][9]),
        }
    }
}
