#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_guild_create() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("Create".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("Builders".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Guild,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 200; // Need 100 to create

        while !vm.halted && vm.ip.1 < 4 {
            vm.step();
        }

        assert!(vm.guilds.contains_key("Builders"));
        assert_eq!(vm.strand_guild_map.get(&0), Some(&"Builders".to_string()));
        // Energy: 200 - 4 steps - 100 cost = 96
        assert_eq!(vm.energy, 96);
    }

    #[test]
    fn test_guild_join() {
        // First create guild manually
        let mut vm = ChimeraVM::new(make_dna(vec![]));
        vm.energy = 200;

        use crate::vm::nova_guild::GuildState;
        let guild = GuildState::new("Mages".to_string(), 1); // Founder is 1
        vm.guilds.insert("Mages".to_string(), guild);

        // Strand 0 tries to join
        // Stack: [ "Join", "Mages", 0 ]
        vm.stack.push(Value::Str("Join".to_string()));
        vm.stack.push(Value::Str("Mages".to_string()));
        vm.stack.push(Value::Int(0));

        // Mock execute OpCode::Guild logic via helper if possible,
        // or just rely on manual execution if we could expose it,
        // but it's private to module usually? No, it's pub in nova_guild.
        crate::vm::nova_guild::exec_guild(&mut vm);

        assert!(vm.guilds.get("Mages").unwrap().members.contains(&0));
        assert_eq!(vm.strand_guild_map.get(&0), Some(&"Mages".to_string()));
        assert_eq!(vm.energy, 190); // Cost 10
    }

    #[test]
    fn test_guild_deposit() {
        // Strand 0 in "Bankers"
        let mut vm = ChimeraVM::new(make_dna(vec![]));
        vm.energy = 100;

        use crate::vm::nova_guild::GuildState;
        let mut guild = GuildState::new("Bankers".to_string(), 0);
        guild.members.insert(0);
        vm.guilds.insert("Bankers".to_string(), guild);
        vm.strand_guild_map.insert(0, "Bankers".to_string());

        // Deposit 50
        vm.stack.push(Value::Str("Deposit".to_string()));
        vm.stack.push(Value::Str("Bankers".to_string()));
        vm.stack.push(Value::Int(50));

        crate::vm::nova_guild::exec_guild(&mut vm);

        assert_eq!(vm.energy, 50);
        assert_eq!(vm.guilds.get("Bankers").unwrap().treasury, 50);
    }

    #[test]
    fn test_charter_kick() {
        let mut vm = ChimeraVM::new(make_dna(vec![]));
        vm.energy = 100;

        use crate::vm::nova_guild::GuildState;
        let mut guild = GuildState::new("Thieves".to_string(), 0); // Founder 0
        guild.members.insert(1); // Member 1
        vm.guilds.insert("Thieves".to_string(), guild);
        vm.strand_guild_map.insert(0, "Thieves".to_string());
        vm.strand_guild_map.insert(1, "Thieves".to_string());

        // Kick member 1
        // Expected Stack order for pop:
        // Pop 1: Guild Name ("Thieves")
        // Pop 2: Arg (1)
        // Pop 3: Action ("Kick")
        // So Stack should be [Action, Arg, Name] -> Push Action, Push Arg, Push Name.

        vm.stack.push(Value::Str("Kick".to_string()));
        vm.stack.push(Value::Int(1));
        vm.stack.push(Value::Str("Thieves".to_string()));

        crate::vm::nova_guild::exec_charter(&mut vm);

        assert!(!vm.guilds.get("Thieves").unwrap().members.contains(&1));
        assert!(vm.strand_guild_map.get(&1).is_none());
    }
}
