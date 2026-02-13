use anyhow::Result;
use clap::Parser as ClapParser;
use pest::Parser;
use std::fs;

use chimera_lang::{ast::Dna, compiler, tui::run_tui, vm::ChimeraVM, ChimeraParser, Rule};
use std::path::Path;

#[cfg(feature = "resonance")]
use crossbeam_channel::unbounded;
#[cfg(feature = "resonance")]
use resonance_audio::audio::AudioModel;
#[cfg(feature = "resonance")]
use std::thread;
#[cfg(feature = "resonance")]
use std::time::Duration;

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: String,

    /// Run in headless mode (no TUI)
    #[arg(long)]
    headless: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let unparsed_file = fs::read_to_string(&cli.input)?;

    // Set a panic hook to restore the terminal if we panic
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen);
        let _ = crossterm::execute!(std::io::stdout(), crossterm::cursor::Show);
        original_hook(panic_info);
    }));

    let path = Path::new(&cli.input);
    let extension = path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");

    let dna = if extension == "chs" {
        compiler::compile(&unparsed_file, path.parent())?
    } else {
        let dna_pair = ChimeraParser::parse(Rule::dna, &unparsed_file)?
            .next()
            .ok_or_else(|| anyhow::anyhow!("No DNA found"))?;
        Dna::try_from_pair(dna_pair).map_err(|e| anyhow::anyhow!("DNA parse error: {}", e))?
    };

    let mut vm = ChimeraVM::new(dna);

    #[cfg(feature = "resonance")]
    {
        let (cmd_tx, cmd_rx) = unbounded();
        let (snap_tx, snap_rx) = unbounded();

        vm.set_audio_tx(cmd_tx);
        vm.set_snapshot_rx(snap_rx);

        // Spawn Audio Simulation Thread
        thread::spawn(move || {
            let mut model = AudioModel::new(16, 16, cmd_rx, snap_tx);
            // Simulate 44100Hz audio in chunks
            // Process 735 samples (approx 16.6ms of audio) every ~16ms to keep real-time speed.
            let chunk_size = 735;
            let mut buffer = vec![0.0; chunk_size];
            loop {
                model.process(&mut buffer);
                thread::sleep(Duration::from_millis(16));
            }
        });
    }

    if cli.headless {
        while !vm.halted {
            vm.step();
        }

        println!("Execution complete.");

        let mut table = comfy_table::Table::new();
        table.set_header(vec!["Index", "Type", "Value"]);

        for (i, val) in vm.stack.iter().rev().enumerate() {
            let type_str = match val {
                chimera_lang::vm::Value::Int(_) => "Integer",
                chimera_lang::vm::Value::Str(_) => "String",
                chimera_lang::vm::Value::Junction(_, _) => "Junction",
                chimera_lang::vm::Value::Superposition(_) => "Superposition",
            };

            let val_str = format!("{}", val);
            let mut val_cell = comfy_table::Cell::new(&val_str);

            // Mosaic Philosophy: "Colorize 'True' as Green."
            if val_str == "1" || val_str.to_lowercase() == "true" {
                val_cell = val_cell.fg(comfy_table::Color::Green);
            } else if val_str == "0" || val_str.to_lowercase() == "false" {
                val_cell = val_cell.fg(comfy_table::Color::Red);
            }

            table.add_row(vec![
                comfy_table::Cell::new(i),
                comfy_table::Cell::new(type_str),
                val_cell,
            ]);
        }

        println!("Final Stack (Top -> Bottom):");
        println!("{table}");

        println!("Output Log:");
        for line in vm.output {
            println!("  {}", line);
        }
    } else {
        run_tui(vm)?;
    }

    Ok(())
}
