#[cfg(feature = "nova")]
use chimera_lang::prelude::*;
#[cfg(feature = "nova")]
use chimera_lang::tui::{run_tui, ViewMode};

#[cfg(feature = "nova")]
fn main() {
    println!("🗣️ Echo's Story Demo");
    if let Err(e) = run_demo() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(not(feature = "nova"))]
fn main() {
    println!("⚠️  This example requires the 'nova' feature.");
    println!("👉  Run with: cargo run --example story_demo --features nova");
}

#[cfg(feature = "nova")]
fn run_demo() -> anyhow::Result<()> {
    // 1. Initialize empty VM
    let dna = Dna { evolution_config: None,
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

    // Launch TUI
    // We start in Grid view to see the story elements we just wrote
    run_tui(vm, Some(ViewMode::Grid), None)?;

    Ok(())
}
