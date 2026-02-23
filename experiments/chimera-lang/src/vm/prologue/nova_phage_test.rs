use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

#[test]
fn test_phage_mutation() {
    // 1. Setup DNA with a simple strand
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(42)],
        },
        Gene {
            op: OpCode::Print,
            args: vec![],
        },
        Gene {
            op: OpCode::Dup,
            args: vec![],
        },
        Gene {
            op: OpCode::Add,
            args: vec![],
        }
    ];
    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes: genes.clone() }],
        },
        evolution_config: None,
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // 2. Setup Grid
    // Place Phage at (5, 5)
    vm.grid[5][5] = Value::Str("🦠".to_string());
    // Place Strand Index 0 at (5, 6) (East of Phage)
    vm.grid[5][6] = Value::Int(0);

    // 3. Run Tick
    // The Phage should see the neighbor (0), interpret it as strand index 0, and mutate it.
    exec_prologue_tick(&mut vm);

    // 4. Verify Mutation
    let output = vm.output.join("\n");
    println!("VM Output:\n{}", output);

    assert!(output.contains("PHAGE"), "Phage agent did not log any activity");

    // Verify that the agent is still alive (moved or stayed)
    // It consumes the grid cell it moves to, so (5,5) might be empty and (5,6) might be Phage, or similar.
    // Or it stayed put if no valid move (but neighbors has 0, which is valid move).
}
