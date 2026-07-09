use chimera_lang::prelude::*;
use chimera_lang::tui::{run_tui, ViewMode};

/// 🎨 Mosaic: A high-level builder API for constructing Narrative experiments.
pub struct NarrativeBuilder {
    pub vm: ChimeraVM,
    current_y: usize,
    current_x: usize,
}

impl NarrativeBuilder {
    pub fn new() -> Self {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        Self {
            vm: ChimeraVM::new(dna),
            current_y: 0,
            current_x: 0,
        }
    }

    /// Appends a word/string element to the grid at the current line.
    pub fn write_word(mut self, word: &str) -> Self {
        self.vm.grid[self.current_y][self.current_x] = Value::Str(word.to_string());
        self.current_x += 1;
        self
    }

    /// Appends an integer element to the grid at the current line.
    pub fn write_int(mut self, value: i64) -> Self {
        self.vm.grid[self.current_y][self.current_x] = Value::Int(value);
        self.current_x += 1;
        self
    }

    /// Moves the cursor to the next line.
    pub fn next_line(mut self) -> Self {
        self.current_y += 1;
        self.current_x = 0;
        self
    }

    /// Adds a basic reader strand that executes a line of length `len` starting at `(y, x)`.
    pub fn add_reader_strand(mut self, y: i64, x: i64, len: i64) -> Self {
        let reader_strand = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(len)],
                }, // len
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(y)],
                }, // y
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(x)],
                }, // x
                Gene {
                    op: OpCode::Incubate,
                    args: vec![],
                },
            ],
        };
        self.vm.dna.helix.strands.push(reader_strand);
        self
    }

    /// Consumes the builder and returns the configured ChimeraVM.
    pub fn build(self) -> ChimeraVM {
        self.vm
    }
}

impl Default for NarrativeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn main() -> anyhow::Result<()> {
    // 1. Initialize VM using the new high-level NarrativeBuilder API
    println!("✍️  Writing story elements to Petri Dish...");

    let mut vm = NarrativeBuilder::new()
        // Let's create a story: "Once upon a time, there were 10 dragons."
        // In Chimera: push(10) print()
        .write_word("push")
        .write_int(10)
        .write_word("print")
        .add_reader_strand(0, 0, 3) // Reads 3 elements starting at (0, 0)
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
