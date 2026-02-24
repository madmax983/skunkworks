use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
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
fn test_heatmap_tracking() {
    // [ push(1) push(1) add() jump(0) ]
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Add,
            args: vec![],
        },
        Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(0)],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));

    // Run 10 steps
    for _ in 0..10 {
        vm.step();
    }

    // Check counts
    // Strand 0, Gene 0 (Push) should be executed
    let count_0 = *vm
        .gene_execution_counts
        .get(0)
        .and_then(|s| s.get(0))
        .unwrap_or(&0);
    assert!(count_0 > 0, "Gene 0 should have been executed");

    // Gene 3 (Jump) should be executed
    let count_3 = *vm
        .gene_execution_counts
        .get(0)
        .and_then(|s| s.get(3))
        .unwrap_or(&0);
    assert!(count_3 > 0, "Gene 3 should have been executed");
}

#[test]
fn test_viral_injection() {
    // [ push(10) ]
    let genes = vec![Gene {
        op: OpCode::Push,
        args: vec![Nucleotide::Number(10)],
    }];
    let mut vm = ChimeraVM::new(make_dna(genes));

    // Step once to execute push(10)
    vm.step();
    assert_eq!(vm.stack.last(), Some(&Value::Int(10)));

    // Inject [ push(20) add() ]
    let injected_genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(20)],
        },
        Gene {
            op: OpCode::Add,
            args: vec![],
        },
    ];

    vm.inject_genes(injected_genes);

    // Verify DNA modification
    // Original: push(10) (executed, IP is now (0, 1))
    // Injected at (0, 1): push(20), add()
    // Resulting Strand: push(10), push(20), add()

    assert_eq!(vm.dna.helix.strands[0].genes.len(), 3);
    assert_eq!(vm.dna.helix.strands[0].genes[1].op, OpCode::Push);
    assert_eq!(vm.dna.helix.strands[0].genes[2].op, OpCode::Add);

    // Step to execute injected code
    vm.step(); // executes push(20)
    assert_eq!(vm.stack.last(), Some(&Value::Int(20)));

    vm.step(); // executes add() -> 10 + 20 = 30
    assert_eq!(vm.stack.last(), Some(&Value::Int(30)));

    // Check injection counts
    let count_injected = *vm
        .gene_execution_counts
        .get(0)
        .and_then(|s| s.get(1))
        .unwrap_or(&0);
    assert_eq!(count_injected, 1, "Injected gene should execute once");
}
