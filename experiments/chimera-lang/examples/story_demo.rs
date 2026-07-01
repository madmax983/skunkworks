use chimera_lang::prelude::*;
use chimera_lang::tui::{run_tui, ViewMode};

fn main() -> anyhow::Result<()> {
    // 1. Initialize empty VM
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);

    // 2. Write "Story Elements" to the Petri Dish using NarrativeBuilder
    println!("✍️  Writing story elements to Petri Dish...");

    // Let's create a story: "Once upon a time, there were 10 dragons."
    // In Chimera: push(10) print()
    // Using direct grid manipulation:
    vm.grid[0][0] = Value::Str("push".to_string());
    vm.grid[0][1] = Value::Int(10);
    vm.grid[0][2] = Value::Str("print".to_string());

    let reader_strand = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(3)],
            }, // len
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // x
            Gene {
                op: OpCode::Incubate,
                args: vec![],
            },
        ],
    };
    vm.dna.helix.strands.push(reader_strand);

    println!("🧪 Incubating narrative... Launching TUI.");
    println!("(Press Space to Step, Q to Quit)");

    // run_tui starts a TUI and blocks execution
    run_tui(vm, Some(ViewMode::Grid), None)?;

    Ok(())
}
