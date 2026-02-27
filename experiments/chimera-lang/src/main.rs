use anyhow::Result;
use clap::Parser as ClapParser;
use pest::Parser;
use std::collections::HashMap;
use std::fs;

use chimera_lang::{
    ast::{Dna, Helix, JunctionType},
    compiler, prologue_compiler,
    tui::{run_tui, ViewMode},
    vm::{ChimeraVM, Value},
    ChimeraParser, Rule,
};
use std::path::Path;

#[cfg(feature = "resonance")]
use chimera_lang::audio_source::RodioAudioSource;
#[cfg(feature = "resonance")]
use crossbeam_channel::unbounded;
#[cfg(feature = "resonance")]
use resonance_audio::audio::AudioModel;
#[cfg(feature = "resonance")]
use rodio::OutputStream;
#[cfg(feature = "resonance")]
use std::thread;
#[cfg(feature = "resonance")]
use std::time::Duration;

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: Option<String>,

    /// Run in headless mode (no TUI)
    #[arg(long)]
    headless: bool,

    /// Number of ticks to run in headless mode
    #[arg(long, default_value = "100")]
    ticks: u64,
}

fn format_oracle_result(val: &Value) -> Option<String> {
    if let Value::Junction(JunctionType::All, items) = val {
        // Check if this looks like a list of bindings from Oracle (List of [Key, Value])
        let is_binding_list = !items.is_empty()
            && items.iter().all(|item| {
                if let Value::Junction(JunctionType::All, b_args) = item {
                    b_args.len() == 2 && matches!(b_args[0], Value::Str(_))
                } else {
                    false
                }
            });

        if is_binding_list {
            let mut table = comfy_table::Table::new();
            table.load_preset(comfy_table::presets::UTF8_NO_BORDERS);
            table.set_header(vec!["Key", "Value"]);

            for item in items {
                if let Value::Junction(_, args) = item {
                    if let Value::Str(k) = &args[0] {
                        let v = &args[1];
                        let v_str = format!("{}", v);
                        let mut v_cell = comfy_table::Cell::new(&v_str);

                        // Colorize
                        if v_str == "1" || v_str.eq_ignore_ascii_case("true") {
                            v_cell = v_cell.fg(comfy_table::Color::Green);
                        } else if v_str == "0" || v_str.eq_ignore_ascii_case("false") {
                            v_cell = v_cell.fg(comfy_table::Color::Red);
                        } else if matches!(v, Value::Str(_)) {
                            v_cell = v_cell.fg(comfy_table::Color::Cyan);
                        }

                        table.add_row(vec![
                            comfy_table::Cell::new(k).fg(comfy_table::Color::Yellow),
                            v_cell,
                        ]);
                    }
                }
            }
            return Some(table.to_string());
        }
    }
    None
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set a panic hook to restore the terminal if we panic
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen);
        let _ = crossterm::execute!(std::io::stdout(), crossterm::cursor::Show);
        original_hook(panic_info);
    }));

    let (dna, grid, orca_mode, custom_runes) = if let Some(input_path) = &cli.input {
        let unparsed_file = fs::read_to_string(input_path)?;
        let path = Path::new(input_path);
        let extension = path
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("");

        if extension == "pro" {
            prologue_compiler::compile(&unparsed_file, path.parent())?
        } else if extension == "score" {
            #[cfg(feature = "resonance")]
            {
                (
                    chimera_lang::acoustic_compiler::compile(&unparsed_file)?,
                    None,
                    None,
                    HashMap::new(),
                )
            }
            #[cfg(not(feature = "resonance"))]
            {
                return Err(anyhow::anyhow!(
                    "Resonance feature disabled. Cannot compile score."
                ));
            }
        } else if extension == "chs" {
            (
                compiler::compile(&unparsed_file, path.parent())?,
                None,
                None,
                HashMap::new(),
            )
        } else if extension == "lisp" || extension == "cl" {
            (
                chimera_lang::lisp::compile(&unparsed_file)?,
                None,
                None,
                HashMap::new(),
            )
        } else {
            let dna_pair = ChimeraParser::parse(Rule::dna, &unparsed_file)?
                .next()
                .ok_or_else(|| anyhow::anyhow!("No DNA found"))?;
            (
                Dna::try_from_pair(dna_pair)
                    .map_err(|e| anyhow::anyhow!("DNA parse error: {}", e))?,
                None,
                None,
                HashMap::new(),
            )
        }
    } else {
        // Default empty DNA
        (
            Dna {
                evolution_config: None,
                helix: Helix { strands: vec![] },
            },
            None,
            None,
            HashMap::new(),
        )
    };

    let mut vm = ChimeraVM::new(dna);

    if let Some(g) = grid {
        vm.grid = g;
    }

    // Enable Prologue Mode by default if Nova is active
    #[cfg(feature = "nova")]
    {
        vm.prologue_state.active = true;
        if let Some(mode) = orca_mode {
            vm.prologue_state.orca_mode = mode;
        }
        vm.prologue_state.custom_runes = custom_runes;
    }

    #[cfg(feature = "resonance")]
    // Create stream handle outside to keep it alive. We assume default device exists.
    // If not, this might panic, which is acceptable for an experimental feature.
    let (_stream, stream_handle) = match OutputStream::try_default() {
        Ok(s) => (Some(s.0), Some(s.1)),
        Err(e) => {
            eprintln!("Warning: Failed to initialize audio output: {}", e);
            (None, None)
        }
    };

    #[cfg(feature = "resonance")]
    let _sink = if let Some(handle) = stream_handle.as_ref() {
        let (cmd_tx, cmd_rx) = unbounded();
        let (snap_tx, snap_rx) = unbounded();

        vm.set_audio_tx(cmd_tx);
        vm.set_snapshot_rx(snap_rx);

        // Initialize Audio Model with recording_tx = None
        // Grid size matches VM (16x16)
        let model = AudioModel::new(16, 16, cmd_rx, snap_tx, None);
        let source = RodioAudioSource::new(model);

        match rodio::Sink::try_new(handle) {
            Ok(sink) => {
                sink.append(source);
                sink.play();
                Some(sink)
            }
            Err(e) => {
                eprintln!("Warning: Failed to create audio sink: {}", e);
                None
            }
        }
    } else {
        None
    };

    if cli.headless {
        for _ in 0..cli.ticks {
            if vm.halted {
                break;
            }
            vm.step();
        }

        println!("Execution complete.");

        let mut table = comfy_table::Table::new();
        table
            .load_preset(comfy_table::presets::UTF8_FULL)
            .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
            .set_header(vec!["Index", "Type", "Value"]);

        for (i, val) in vm.stack.iter().rev().enumerate() {
            let (type_str, type_color) = match val {
                Value::Int(_) => ("Integer", comfy_table::Color::Blue),
                Value::Str(_) => ("String", comfy_table::Color::Cyan),
                Value::Junction(_, _) => ("Junction", comfy_table::Color::Magenta),
                Value::Superposition(_) => ("Superposition", comfy_table::Color::Yellow),
                Value::Symbol(_) => ("Symbol", comfy_table::Color::Magenta),
                Value::Color(_, _, _) => ("Color", comfy_table::Color::Green),
            };

            let val_str = format_oracle_result(val).unwrap_or_else(|| format!("{}", val));

            let mut val_cell = comfy_table::Cell::new(&val_str);

            if val_str == "1" || val_str.to_lowercase() == "true" {
                val_cell = val_cell.fg(comfy_table::Color::Green);
            } else if val_str == "0" || val_str.to_lowercase() == "false" {
                val_cell = val_cell.fg(comfy_table::Color::Red);
            } else if matches!(val, Value::Str(_)) {
                val_cell = val_cell.fg(comfy_table::Color::Cyan);
            }

            table.add_row(vec![
                comfy_table::Cell::new(i),
                comfy_table::Cell::new(type_str).fg(type_color),
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
        let input_path = cli.input.as_ref().map(|s| Path::new(s).to_path_buf());

        #[cfg(feature = "nova")]
        run_tui(vm, Some(ViewMode::Prologue), input_path)?;

        #[cfg(not(feature = "nova"))]
        run_tui(vm, None, input_path)?;
    }

    Ok(())
}
