#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Helix, Strand};
    use crate::opcode::OpCode;
    use crate::vm::nova::{Organelle, OrganelleType};
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000;
        vm
    }

    #[test]
    fn test_metazoa_bonding() {
        let mut vm = make_vm();

        // Organelle 1 at (5,5)
        vm.organelle_id_counter += 1;
        let id1 = vm.organelle_id_counter;
        let org1 = Organelle {
            stack: vec![],
            ip: (0, 0),
            context_loc: (5, 5),
            call_stack: vec![],
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Worker,
            direction: (0, 0),
            ttl: None,
            name: "Cell 1".to_string(),
            traits: vec![],
            id: id1,
            tissue_id: None,
            genome_id: 0,
        };
        vm.organelles.push(org1);

        // Organelle 2 at (5,6) (East of 1)
        vm.organelle_id_counter += 1;
        let id2 = vm.organelle_id_counter;
        let org2 = Organelle {
            stack: vec![],
            ip: (0, 0),
            context_loc: (5, 6),
            call_stack: vec![],
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Worker,
            direction: (0, 0),
            ttl: None,
            name: "Cell 2".to_string(),
            traits: vec![],
            id: id2,
            tissue_id: None,
            genome_id: 0,
        };
        vm.organelles.push(org2);

        // Cell 1 Bonds East (1)
        // Manual execution context setup
        vm.context_loc = (5, 5);
        vm.stack.push(Value::Int(1)); // East
        crate::vm::nova_metazoa::exec_bond(&mut vm, OpCode::Bond, &[]);

        // Check bond
        let tissue_id_val = vm.stack.pop().unwrap();
        if let Value::Int(tid) = tissue_id_val {
            assert!(tid > 0, "Should have created a tissue");
            assert!(vm.tissues.contains_key(&(tid as usize)), "Tissue should exist");
            let tissue = &vm.tissues[&(tid as usize)];
            assert!(tissue.members.contains(&id1));
            assert!(tissue.members.contains(&id2));
        } else {
            panic!("Bond failed to return tissue ID");
        }

        // Cell 2 Sends Signal (Signify)
        vm.context_loc = (5, 6);
        vm.stack.push(Value::Str("Hello".to_string()));
        crate::vm::nova_metazoa::exec_signify(&mut vm, OpCode::Signify, &[]);

        // Check Cell 1 stack
        let cell1 = vm.organelles.iter().find(|o| o.id == id1).unwrap();
        assert_eq!(cell1.stack.len(), 1);
        assert_eq!(cell1.stack[0], Value::Str("Hello".to_string()));
    }
}
