use chimera_lang::compiler::compile;
use chimera_lang::vm::ChimeraVM;
use chimera_lang::value::Value;

#[test]
fn test_ouroboros_protocol() {
    let src = r#"
    chimera OuroborosAgent {
        grammar: Regex(".*")
        boot: {
            ouroboros
        }
    }
    "#;

    let dna = compile(src, None).unwrap();
    let mut vm = ChimeraVM::new(dna);

    // Initial state: Strand 0 is OuroborosAgent_DNA
    assert_eq!(vm.dna.helix.strands.len(), 1);
    assert_eq!(vm.ip, (0, 0));

    // Step 1: Compiler inserted `push(grammar)` at start
    vm.step();
    // Verify stack has grammar
    match vm.stack.last() {
        Some(Value::Junction(_, args)) => {
            // Should be Regex(".*")
             if let Value::Str(t) = &args[0] {
                 assert_eq!(t, "Regex");
             } else {
                 panic!("Expected Regex grammar");
             }
        },
        _ => panic!("Expected grammar on stack"),
    }

    // Step 2: ouroboros
    vm.step();

    // Check if new strand created
    assert_eq!(vm.dna.helix.strands.len(), 2, "Ouroboros should create a new strand");

    // Check if IP jumped to new strand
    assert_eq!(vm.ip.0, 1, "IP should jump to new strand");
    assert_eq!(vm.ip.1, 0);
}
