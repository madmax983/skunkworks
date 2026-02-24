#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;
    use crate::vm::Value;

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.knowledge_base.clear(); // Clear standard library for tests
        vm
    }

    #[test]
    fn test_rule_duplication() {
        let mut vm = make_vm();

        // rule(head, body)
        // Stack: [Body, Head] -> Rule

        let head = Value::Str("test".to_string());
        let body = Value::Str("true".to_string());

        // Push Body
        vm.stack.push(body.clone());
        // Push Head
        vm.stack.push(head.clone());

        // Execute Rule
        crate::vm::oracle::exec_oracle_op(&mut vm, OpCode::Rule, &[]);

        assert_eq!(vm.knowledge_base.len(), 1);

        // Push Body again
        vm.stack.push(body.clone());
        // Push Head again
        vm.stack.push(head.clone());

        // Execute Rule again
        crate::vm::oracle::exec_oracle_op(&mut vm, OpCode::Rule, &[]);

        // Should still be 1 if duplication is prevented
        assert_eq!(vm.knowledge_base.len(), 1, "Duplicate rule added");
    }

    #[test]
    fn test_savant_logic_loop() {
        let mut vm = make_vm();

        // DNA: Assert action(move(north))
        // Nucleotide::Junction construction

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::Any,
                    vec![
                        Nucleotide::String("action".to_string()),
                        Nucleotide::Junction(
                            JunctionType::Any,
                            vec![
                                Nucleotide::String("move".to_string()),
                                Nucleotide::String("north".to_string()),
                            ],
                        ),
                    ],
                )],
            },
            Gene {
                op: OpCode::Assert,
                args: vec![],
            },
        ];

        let strand = Strand { genes };
        vm.dna.helix.strands.push(strand);

        // Create Savant
        let mut savant = crate::vm::nova::Organelle {
            kind: crate::vm::nova::OrganelleType::Savant,
            ip: (1, 0), // Point to the new strand (index 1, as index 0 is empty from make_vm)
            context_loc: (8, 8),
            stack: vec![],
            call_stack: vec![],
            recursion_depth: 0,
            halted: false,
            direction: (0, 0),
            ttl: None,
            name: "Savant".to_string(),
            traits: vec![],
            id: 1,
            tissue_id: None,
            genome_id: 0,
            energy: 100,
            experience: 0,
            stage: 0,
        };

        // Simulate tick_organelle swapping
        std::mem::swap(&mut vm.stack, &mut savant.stack);
        std::mem::swap(&mut vm.ip, &mut savant.ip);
        std::mem::swap(&mut vm.context_loc, &mut savant.context_loc);

        // Process
        crate::vm::nova_savant::process_savant(&mut vm, &mut savant);

        // Swap back
        std::mem::swap(&mut vm.stack, &mut savant.stack);
        std::mem::swap(&mut vm.ip, &mut savant.ip);
        std::mem::swap(&mut vm.context_loc, &mut savant.context_loc);

        // Check if moved
        // North is (-1, 0). 8,8 -> 7,8
        assert_eq!(savant.context_loc, (7, 8), "Savant failed to move north");
    }
}
