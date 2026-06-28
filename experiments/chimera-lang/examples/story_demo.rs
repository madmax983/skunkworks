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
    let mut builder = NarrativeBuilder::new(&mut vm, 0, 0);

    // In Chimera: push(10) print()
    // Using builder abstractions:
    builder
        .write_instruction("push")
        .write_value(10)
        .write_instruction("print");

    builder.incubate();

    println!("🧪 Incubating narrative... Launching TUI.");
    println!("(Press Space to Step, Q to Quit)");

    // run_tui starts a TUI and blocks execution
    run_tui(vm, Some(ViewMode::Grid), None)?;

    Ok(())
}
