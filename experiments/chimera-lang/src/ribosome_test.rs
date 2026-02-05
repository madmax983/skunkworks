use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

fn make_dna(strands: Vec<Strand>) -> Dna {
    Dna {
        helix: Helix {
            strands: strands.into_iter().map(std::rc::Rc::new).collect(),
        },
    }
}

#[test]
fn test_ribosome_execution() {
    // Strand 0: Spawn Ribosome, then jump to Strand 1.
    let strand0 = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(4)],
            },
            Gene {
                op: OpCode::Spawn,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(1)],
            },
        ],
    };

    // Strand 1: Infinite loop to keep VM alive.
    let strand1 = Strand {
        genes: vec![Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(1)],
        }],
    };

    let mut vm = ChimeraVM::new(make_dna(vec![strand0, strand1]));

    // Setup Grid Program
    // (8,8): 42 (Push 42)
    vm.grid[8][8] = Value::Int(42);
    // (8,9): "v" (Turn Down)
    vm.grid[8][9] = Value::Str("v".to_string());
    // (9,9): 99 (Push 99)
    vm.grid[9][9] = Value::Int(99);

    // Step 1: Push 0
    vm.step();
    // Step 2: Push 4
    vm.step();
    // Step 3: Spawn. Ribosome created.
    // Ribosome executes Step 1: Reads 42 at (8,8). Pushes 42. Moves East to (8,9).
    // Main IP -> Jump(1).
    vm.step();

    assert_eq!(vm.organelles.len(), 1);
    assert_eq!(vm.organelles[0].stack.len(), 1);
    assert_eq!(vm.organelles[0].stack[0], Value::Int(42));
    assert_eq!(vm.organelles[0].context_loc, (8, 9));
    assert_eq!(vm.organelles[0].direction, (0, 1));

    // Step 4: Main Jumps to Strand 1.
    // Ribosome executes Step 2: Reads "v" at (8,9). Dir -> South. Moves South to (9,9).
    vm.step();

    assert_eq!(vm.organelles[0].direction, (1, 0));
    assert_eq!(vm.organelles[0].context_loc, (9, 9));

    // Step 5: Main Jumps to Strand 1 (Loop).
    // Ribosome executes Step 3: Reads 99 at (9,9). Pushes 99. Moves South to (10,9).
    vm.step();

    assert_eq!(vm.organelles[0].stack.len(), 2);
    assert_eq!(vm.organelles[0].stack[1], Value::Int(99));
    assert_eq!(vm.organelles[0].context_loc, (10, 9));
}
