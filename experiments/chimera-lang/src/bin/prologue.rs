use anyhow::Result;
use clap::Parser as ClapParser;
use std::fs;
use std::path::Path;

use chimera_lang::{
    ast::Dna,
    ast::Helix,
    compiler,
    tui::{run_tui, ViewMode},
    vm::ChimeraVM,
    ChimeraParser, Rule,
};
use pest::Parser;

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
    input: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set a panic hook to restore the terminal if we panic
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // Attempt to restore terminal state
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen);
        let _ = crossterm::execute!(std::io::stdout(), crossterm::cursor::Show);
        original_hook(panic_info);
    }));

    let dna = if let Some(input_path) = &cli.input {
        let unparsed_file = fs::read_to_string(input_path)?;
        let path = Path::new(input_path);
        let extension = path
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("");

        if extension == "chs" {
            compiler::compile(&unparsed_file, path.parent())?
        } else if extension == "lisp" || extension == "cl" {
            chimera_lang::lisp::compile(&unparsed_file)?
        } else {
            let dna_pair = ChimeraParser::parse(Rule::dna, &unparsed_file)?
                .next()
                .ok_or_else(|| anyhow::anyhow!("No DNA found"))?;
            Dna::try_from_pair(dna_pair).map_err(|e| anyhow::anyhow!("DNA parse error: {}", e))?
        }
    } else {
        // Default empty DNA
        Dna {
            helix: Helix { strands: vec![] },
        }
    };

    let mut vm = ChimeraVM::new(dna);

    // Enable Prologue Mode by default
    #[cfg(feature = "nova")]
    {
        vm.prologue_state.active = true;
    }

    #[cfg(feature = "resonance")]
    {
        let (cmd_tx, cmd_rx) = unbounded();
        let (snap_tx, snap_rx) = unbounded();

        vm.set_audio_tx(cmd_tx);
        vm.set_snapshot_rx(snap_rx);

        thread::spawn(move || {
            let mut model = AudioModel::new(16, 16, cmd_rx, snap_tx);
            let chunk_size = 735;
            let mut buffer = vec![0.0; chunk_size];
            loop {
                model.process(&mut buffer);
                thread::sleep(Duration::from_millis(16));
            }
        });
    }

    #[cfg(feature = "nova")]
    run_tui(vm, Some(ViewMode::Prologue))?;

    #[cfg(not(feature = "nova"))]
    {
        println!("Error: Prologue requires the 'nova' feature enabled.");
        run_tui(vm, None)?;
    }

    Ok(())
}
