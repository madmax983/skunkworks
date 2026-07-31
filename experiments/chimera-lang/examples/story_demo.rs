use chimera_lang::tui::{run_tui, ViewMode};
use chimera_lang::NarrativeBuilder;

fn main() -> anyhow::Result<()> {
    println!("✍️  Writing story elements to Petri Dish...");

    // Let's create a story: "Once upon a time, there were 10 dragons."
    // In Chimera: push(10) print()
    // Using the high-level NarrativeBuilder API:
    let mut vm = NarrativeBuilder::new()
        .push_str("push")
        .push_int(10)
        .push_str("print")
        .add_reader_strand()
        .build();

    if std::env::args().any(|arg| arg == "--headless") {
        println!("🧪 Incubating narrative headlessly...");
        // In headless mode, we can just run the VM for a few steps
        // The Incubate op reads from the grid, creating a new strand.
        // Then we can step the new strand.
        for _ in 0..10 {
            if vm.halted {
                break;
            }
            vm.step();
        }
        for line in &vm.output {
            println!("{}", line);
        }
        return Ok(());
    }

    println!("🧪 Incubating narrative... Launching TUI.");
    println!("(Press Space to Step, Q to Quit)");

    // run_tui starts a TUI and blocks execution
    run_tui(vm, Some(ViewMode::Grid), None)?;

    Ok(())
}
