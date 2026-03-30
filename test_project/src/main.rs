use chimera_lang::prelude::*;
use chimera_lang::tui::{run_tui, ViewMode};

fn main() -> anyhow::Result<()> {
    // 1. Initialize empty VM
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);

    // 2. Write "Story Elements" to the Petri Dish
    // We'll put them in a row at y=0
    println!("✍️  Writing story elements to Petri Dish...");
    // Let's create a story: "Once upon a time, there were 10 dragons."
    // In Chimera: push(10) print()
    vm.grid[0][0] = Value::Str("push".to_string());
    vm.grid[0][1] = Value::Int(10); // Argument for push
    vm.grid[0][2] = Value::Str("print".to_string());

    // 3. Create a "Reader" strand that incubates the story
    // incubate(len, y, x) -> creates new strand from grid cells
    // We push args in reverse order because stack: len, y, x (top)
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
    // To ensure our test finishes, we won't call it in the automated script
    // run_tui(vm, Some(ViewMode::Grid), None)?;

    Ok(())
}
