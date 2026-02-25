use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand, JunctionType};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::{ChimeraVM, Value};

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
fn test_censor_logic() {
    let genes = vec![
        // 1. Enable Censor
        Gene { op: OpCode::Censor, args: vec![] },
        // 2. Try to print "Fail" (should be censored)
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("Fail".to_string())] },
        Gene { op: OpCode::Print, args: vec![] },
        // 3. Disable Censor
        Gene { op: OpCode::Censor, args: vec![] },
        // 4. Print "Success" (should pass)
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("Success".to_string())] },
        Gene { op: OpCode::Print, args: vec![] },
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));

    // Manually add rule to KB: censor("print")
    // Fact format: Junction(Any, ["censor", "print"])
    let rule = Value::Junction(
        JunctionType::Any,
        vec![Value::Str("censor".to_string()), Value::Str("print".to_string())],
    );
    vm.knowledge_base.push(rule);

    // Run until strand 0 ends
    // 6 genes. But step() handles gene execution.
    // We can run until halted or loop limit.
    for _ in 0..20 {
        if vm.ip.0 >= 1 { break; }
        vm.step();
    }

    // Verify output
    println!("VM Output: {:?}", vm.output);

    // "CENSOR: Regulatory System ENABLED"
    // "CENSORED: print"
    // "CENSOR: Regulatory System DISABLED"
    // "Success"

    assert!(vm.output.iter().any(|s| s.contains("CENSORED: print")), "Should log censorship");
    assert!(!vm.output.iter().any(|s| s.contains("Fail")), "Should not print Fail");
    assert!(vm.output.iter().any(|s| s.contains("Success")), "Should print Success");
}
