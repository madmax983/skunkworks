use chimera_lang::tui::{run_tui, ViewMode};
use chimera_lang::{ChimeraVM, Dna, Gene, Helix, Nucleotide, OpCode, Strand, Value};

fn main() -> anyhow::Result<()> {
    println!("✍️  Writing story elements to Petri Dish...");

    // Let's create a story: "Once upon a time, there were 10 dragons."
    // In Chimera: push(10) print()
    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand {
                genes: vec![
                    Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::Number(3)],
                    },
                    Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::Number(0)],
                    },
                    Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::Number(0)],
                    },
                    Gene {
                        op: OpCode::Incubate,
                        args: vec![],
                    },
                ],
            }],
        },
    };

    let mut vm = ChimeraVM::new(dna);
    vm.grid[0][0] = Value::Str("push".to_string());
    vm.grid[0][1] = Value::Int(10);
    vm.grid[0][2] = Value::Str("print".to_string());

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
