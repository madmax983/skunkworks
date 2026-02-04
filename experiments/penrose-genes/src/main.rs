use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use pest_derive::Parser;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

mod ast;
mod penrose;
mod tui;
mod vm;

use ast::{Dna, Gene, Helix, Nucleotide, Strand};
use vm::ChimeraVM;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ChimeraParser;

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Set panic hook to restore terminal
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));

    // Genes: A simple "Painter" that colors tiles around the center
    // It writes 1 (Red), 2 (Green), 3 (Yellow) to (0,0), (10,10), (-10,-10)
    let genes = vec![
        Gene {
            name: "push".to_string(),
            args: vec![Nucleotide::Number(1)],
        }, // Color Red
        Gene {
            name: "push".to_string(),
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            name: "push".to_string(),
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            name: "g_write".to_string(),
            args: vec![],
        },
        Gene {
            name: "push".to_string(),
            args: vec![Nucleotide::Number(2)],
        }, // Color Green
        Gene {
            name: "push".to_string(),
            args: vec![Nucleotide::Number(10)],
        },
        Gene {
            name: "push".to_string(),
            args: vec![Nucleotide::Number(10)],
        },
        Gene {
            name: "g_write".to_string(),
            args: vec![],
        },
        Gene {
            name: "push".to_string(),
            args: vec![Nucleotide::Number(3)],
        }, // Color Yellow
        Gene {
            name: "push".to_string(),
            args: vec![Nucleotide::Number(-10)],
        },
        Gene {
            name: "push".to_string(),
            args: vec![Nucleotide::Number(-10)],
        },
        Gene {
            name: "g_write".to_string(),
            args: vec![],
        },
        Gene {
            name: "photosynthesize".to_string(),
            args: vec![],
        },
        Gene {
            name: "jump".to_string(),
            args: vec![Nucleotide::Number(12)],
        }, // Loop
    ];

    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };

    let mut vm = ChimeraVM::new(dna);
    vm.energy = 1000;

    // Run Loop
    loop {
        terminal.draw(|f| {
            tui::ui(f, &vm.tiling, &vm.stack, &vm.output, vm.energy);
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        vm.step();
        if vm.halted {
            // keep running visualization but stop stepping
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
