#![cfg(feature = "nova")]

use chimera_lang::prelude::*;
use chimera_lang::vm::Value;

#[test]
fn test_glossolalia_generate() {
    // Construct a grammar: Seq(Match("Hello"), Match("World"))
    // [ push("World") parser_match() push("Hello") parser_match() parser_seq() generate() ]

    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("World".to_string())],
        },
        Gene {
            op: OpCode::ParserMatch,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("Hello".to_string())],
        },
        Gene {
            op: OpCode::ParserMatch,
            args: vec![],
        },
        Gene {
            op: OpCode::ParserSeq,
            args: vec![],
        },
        Gene {
            op: OpCode::Generate,
            args: vec![],
        },
    ];

    let dna = Dna { evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    // Run until strand 0 completes (ip jumps or halted)
    // Actually we can just run for enough steps.
    for _ in 0..10 {
        if vm.halted {
            break;
        }
        vm.step();
    }

    // Check stack for "WorldHello" (Seq pushes p1 then p2, p1 was World, p2 was Hello)
    if let Some(val) = vm.stack.last() {
        if let Value::Str(s) = val {
            assert_eq!(s, "WorldHello");
        } else {
            panic!("Expected String on stack, got {:?}", val);
        }
    } else {
        panic!("Stack empty");
    }
}

#[test]
fn test_glossolalia_scribe() {
    // [ push("TheTablet") scribe() ]
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("TheTablet".to_string())],
        },
        Gene {
            op: OpCode::Scribe,
            args: vec![],
        },
    ];

    let dna = Dna { evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    for _ in 0..5 {
        if vm.halted {
            break;
        }
        vm.step();
    }

    assert_eq!(vm.tablet.len(), 1);
    assert_eq!(vm.tablet[0], "TheTablet");
}

#[cfg(feature = "oracle")]
#[test]
fn test_oracle_generate() {
    // Logic: generate(match("Oracle"), ?Output).
    // Should bind ?Output = "Oracle".

    use chimera_lang::vm::oracle;
    use std::collections::HashMap;

    let dna = Dna { evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let vm = ChimeraVM::new(dna);

    let grammar = Value::Junction(
        chimera_lang::ast::JunctionType::Any,
        vec![
            Value::Str("Match".to_string()),
            Value::Str("Oracle".to_string()),
        ],
    );

    let goal = Value::Junction(
        chimera_lang::ast::JunctionType::Any,
        vec![
            Value::Str("generate".to_string()),
            grammar,
            Value::Str("?Output".to_string()),
        ],
    );

    let mut solutions = Vec::new();
    oracle::solve(
        &[goal],
        HashMap::new(),
        &vm.knowledge_base,
        &vm,
        &mut solutions,
        0,
    );

    assert!(!solutions.is_empty());
    let sol = &solutions[0];
    if let Some(Value::Str(s)) = sol.get("?Output") {
        assert_eq!(s, "Oracle");
    } else {
        panic!("?Output not bound correctly");
    }
}
