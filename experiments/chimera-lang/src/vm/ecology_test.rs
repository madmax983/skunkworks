#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::vm::ChimeraVM;
    use crate::ast::{Dna, Helix, Strand};
    use crate::vm::nova_ecology;

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_spawn_ecology() {
        let mut vm = make_vm();
        nova_ecology::spawn_random_ecology(&mut vm, 10);
        assert_eq!(vm.organelles.len(), 10);
        assert_eq!(vm.dna.helix.strands.len(), 10);
    }

    #[test]
    fn test_spawn_food() {
        let mut vm = make_vm();
        nova_ecology::spawn_food(&mut vm);
        // We can't easily assert where it spawned, but we can check if grid changed
        // But grid is random spawn.
        // Let's spawn 100 food, statistically some should appear.
        for _ in 0..100 {
            nova_ecology::spawn_food(&mut vm);
        }

        let mut food_count = 0;
        for row in &vm.grid {
            for cell in row {
                if let crate::vm::Value::Int(n) = cell {
                    if *n > 0 {
                        food_count += 1;
                    }
                }
            }
        }
        assert!(food_count > 0);
    }

    #[test]
    fn test_scavenger_mechanics() {
        let mut vm = make_vm();
        use crate::vm::nova::{Organelle, OrganelleType};
        use crate::value::Value;

        // Create Scavenger
        let scavenger = Organelle {
            stack: vec![],
            ip: (0, 0),
            context_loc: (5, 5),
            call_stack: vec![],
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Worker,
            direction: (0, 0),
            ttl: None,
            name: "Scavenger".to_string(),
            traits: vec!["Scavenger".to_string()],
            id: 1,
            tissue_id: None,
            genome_id: 0,
            energy: 10, experience: 0, stage: 0,
        };
        vm.organelles.push(scavenger);

        // Place Toxic Waste
        vm.grid[5][5] = Value::Int(-50);

        // Tick
        nova_ecology::process_ecology_tick(&mut vm);

        // Verify Energy Gain: 10 (base) - 1 (metabolism) + 50 (waste) = 59
        // Grid should be 0
        assert_eq!(vm.organelles[0].energy, 59);
        assert_eq!(vm.grid[5][5], Value::Int(0));
    }

    #[test]
    fn test_radioactive_death() {
        let mut vm = make_vm();
        use crate::vm::nova::{Organelle, OrganelleType};
        use crate::value::Value;

        // Create Radioactive Organism with 0 energy (dying)
        // Note: process_ecology_tick subtracts 1 energy at start. So 1 -> 0 -> Death.
        let mutant = Organelle {
            stack: vec![],
            ip: (0, 0),
            context_loc: (5, 5),
            call_stack: vec![],
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Worker,
            direction: (0, 0),
            ttl: None,
            name: "Mutant".to_string(),
            traits: vec!["Radioactive".to_string()],
            id: 1,
            tissue_id: None,
            genome_id: 0,
            energy: 1, experience: 0, stage: 0, // Will drop to 0 in metabolism
        };
        vm.organelles.push(mutant);

        // Tick
        nova_ecology::process_ecology_tick(&mut vm);

        // Verify Death and Waste
        assert!(vm.organelles.is_empty()); // Cleaned up
        assert_eq!(vm.grid[5][5], Value::Int(-50));
    }

    #[test]
    fn test_viral_transfer() {
        let mut vm = make_vm();
        use crate::vm::nova::{Organelle, OrganelleType};
        use crate::ast::{Strand, Gene, Nucleotide};
        use crate::opcode::OpCode;

        // Winner Strand (0)
        vm.dna.helix.strands.push(Strand { genes: vec![] });
        // Loser Strand (1) - Has a gene
        vm.dna.helix.strands.push(Strand { genes: vec![Gene { op: OpCode::Push, args: vec![Nucleotide::Number(99)] }] });

        let winner = Organelle {
            stack: vec![],
            ip: (0, 0),
            context_loc: (5, 5),
            call_stack: vec![],
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Worker,
            direction: (0, 0),
            ttl: None,
            name: "Winner".to_string(),
            traits: vec![],
            id: 1,
            tissue_id: None,
            genome_id: 0,
            energy: 100, experience: 0, stage: 0,
        };

        let loser_viral = Organelle {
            stack: vec![],
            ip: (1, 0),
            context_loc: (5, 5),
            call_stack: vec![],
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Worker,
            direction: (0, 0),
            ttl: None,
            name: "Virus".to_string(),
            traits: vec!["Viral".to_string()],
            id: 2,
            tissue_id: None,
            genome_id: 0,
            energy: 10, experience: 0, stage: 0,
        };

        vm.organelles.push(winner);
        vm.organelles.push(loser_viral);

        // Tick
        nova_ecology::process_ecology_tick(&mut vm);

        // Verify Winner survived and got gene
        assert_eq!(vm.organelles.len(), 1);
        let survivor = &vm.organelles[0];
        assert_eq!(survivor.name, "Winner");

        // Winner's strand (0) should now have the gene from strand 1
        let winner_genes = &vm.dna.helix.strands[0].genes;
        assert_eq!(winner_genes.len(), 1);
        assert_eq!(winner_genes[0].op, OpCode::Push);
        if let Nucleotide::Number(n) = winner_genes[0].args[0] {
            assert_eq!(n, 99);
        } else {
            panic!("Wrong gene arg");
        }
    }
}
