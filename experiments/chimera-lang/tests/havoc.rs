use chimera_lang::ast::{Dna, Helix, JunctionType, Strand};
use chimera_lang::value::Value;
use chimera_lang::vm::nova::{Organelle, OrganelleType};
use chimera_lang::vm::ChimeraVM;
use proptest::prelude::*;

proptest! {
    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_nova_botany_nth_unwrap_crash(
        len_diff in 1..10usize
    ) {
        let mut vm = ChimeraVM::new(Dna { evolution_config: None, helix: Helix { strands: vec![Strand { genes: vec![] }] } });

        vm.stack.push(Value::Junction(JunctionType::Any, vec![])); // rules
        vm.stack.push(Value::Junction(JunctionType::Any, vec![])); // mapping
        vm.stack.push(Value::Str("🍄🍄".to_string())); // string length is 8 bytes, but 2 chars.
        // We know length in bytes is 8, char count is 2. We pass index (char_count + len_diff) to bypass byte len check but exceed char count
        vm.stack.push(Value::Int(2 + len_diff as i64)); // e.g. index 3 < 8 bytes, but >= 2 chars. `nth(3)` will return None!
        vm.stack.push(Value::Junction(JunctionType::All, vec![])); // turtle_stack

        let mut organelle = Organelle {
            stack: vec![],
            ip: (0, 0),
            context_loc: (0, 0),
            call_stack: vec![],
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Seed,
            direction: (0, 0),
            ttl: None,
            name: "test".to_string(),
            traits: vec![],
            id: 0,
            tissue_id: None,
            genome_id: 0,
            energy: 10,
            experience: 0,
            stage: 0,
        };

        chimera_lang::vm::nova_botany::tick_seed(&mut vm, &mut organelle);
    }
}
