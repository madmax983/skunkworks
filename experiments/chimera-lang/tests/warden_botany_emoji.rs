use chimera_lang::prelude::{Dna, Value};
use chimera_lang::vm::{nova::Organelle, ChimeraVM};

#[test]
fn test_botany_emoji_crash_prevented() {
    let mut vm = ChimeraVM::new(Dna::from_genes(vec![]));
    let mut organelle = Organelle {
        stack: vec![],
        ip: (0, 0),
        context_loc: (0, 0),
        call_stack: vec![],
        recursion_depth: 0,
        halted: false,
        kind: chimera_lang::vm::nova::OrganelleType::Seed,
        direction: (0, 1),
        ttl: Some(1000),
        name: "Test".to_string(),
        traits: vec![],
        id: 1,
        tissue_id: None,
        genome_id: 0,
        energy: 50,
        experience: 0,
        stage: 0,
    };

    vm.stack = vec![
        Value::Junction(chimera_lang::ast::JunctionType::All, vec![]), // Rules
        Value::Junction(chimera_lang::ast::JunctionType::All, vec![]), // Mapping
        Value::Str("🌱".to_string()),                                  // String
        Value::Int(1), // Index out of bounds in terms of chars, but within string bytes
        Value::Junction(chimera_lang::ast::JunctionType::All, vec![]), // Turtle Stack
    ];

    // Trigger tick
    chimera_lang::vm::nova_botany::tick_seed(&mut vm, &mut organelle);

    // Test passes if it does not panic.
}
