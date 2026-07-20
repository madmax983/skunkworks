use anyhow::Result;
use clap::Parser as ClapParser;
use pest::Parser;
use std::collections::HashMap;
use std::fs;

use chimera_lang::{
    ast::{Dna, Helix, JunctionType},
    compiler, prologue_compiler, prologue_esolang_compiler,
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
use resonance_audio::AudioModel;
#[cfg(feature = "resonance")]
use rodio::OutputStream;

#[derive(ClapParser)]
#[command(
    name = "Chimera",
    author = "The Mad Scientist",
    version,
    about = "🧬 Chimera VM - A Biological Runtime & Visual Logic Language",
    long_about = "Chimera is a grid-based biological Virtual Machine and visual logic language.\n\nRun genetic code (.chs) or prologue simulations (.prl) in a rich Terminal User Interface, or execute headlessly for CI and automation."
)]
struct Cli {
    /// Path to the DNA (.chs) or Prologue (.prl) file to execute
    #[arg(short, long)]
    input: Option<String>,

    /// Run in headless mode (no TUI)
    #[arg(long)]
    headless: bool,

    /// Output raw JSON instead of formatted text (implies headless)
    #[arg(long)]
    json: bool,

    /// Number of ticks to run in headless mode
    #[arg(long, default_value = "100")]
    ticks: u64,
}

fn format_oracle_result(val: &Value) -> String {
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
            table.load_preset(comfy_table::presets::UTF8_FULL);
            table.apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS);
            table.set_header(vec!["Key", "Value"]);

            for item in items {
                if let Value::Junction(_, args) = item {
                    if let Value::Str(k) = &args[0] {
                        let v = &args[1];
                        let v_str = format!("{}", v);
                        let mut v_cell = comfy_table::Cell::new(&v_str);

                        // Colorize
                        // 🎨 Mosaic: Replaced Red/Green with high contrast Yellow/Green (True -> Green, False -> Yellow)
                        if v_str == "1" || v_str.eq_ignore_ascii_case("true") {
                            v_cell = v_cell.fg(comfy_table::Color::Green);
                        } else if v_str == "0" || v_str.eq_ignore_ascii_case("false") {
                            v_cell = v_cell.fg(comfy_table::Color::Yellow);
                        } else if matches!(v, Value::Str(_)) {
                            v_cell = v_cell.fg(comfy_table::Color::Magenta);
                        }

                        table.add_row(vec![
                            comfy_table::Cell::new(k).fg(comfy_table::Color::Yellow),
                            v_cell,
                        ]);
                    }
                }
            }
            return format!("{}", table);
        }
    }
    format!("{}", val)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // 🎨 Mosaic: Clearer onboarding when no arguments are provided
    if std::env::args().len() <= 1 {
        use crossterm::style::{Color, Stylize};
        println!(
            "
{}",
            "🧬 Welcome to Chimera Lang!".with(Color::Green).bold()
        );
        println!(
            "{}
",
            "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".with(Color::DarkGrey)
        );
        println!(
            "{} {}",
            "ℹ️".with(Color::Cyan),
            "No input file provided. Booting up an empty Petri dish...".with(Color::Grey)
        );
        println!("{} Tip: Run `chimera-lang --help` to see available options, or provide an input file.\n", "💡".with(Color::Yellow));
        std::thread::sleep(std::time::Duration::from_millis(1500));
    }

    // Set a panic hook to restore the terminal if we panic
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen);
        let _ = crossterm::execute!(std::io::stdout(), crossterm::cursor::Show);
        original_hook(panic_info);
    }));

    let (dna, grid, orca_mode, custom_runes, alchemy_book) = if let Some(input_path) = &cli.input {
        use std::io::Read;
        let mut f = fs::File::open(input_path)
            .map_err(|e| anyhow::anyhow!("Failed to open input file '{}': {}", input_path, e))?;
        let mut unparsed_file = String::new();
        let limit = 10 * 1024 * 1024;
        let bytes_read = f
            .by_ref()
            .take(limit + 1)
            .read_to_string(&mut unparsed_file)?;
        if bytes_read > limit as usize {
            return Err(anyhow::anyhow!(
                "File is too large! Maximum allowed size is 10MB."
            ));
        }
        let path = Path::new(input_path);
        let extension = path
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("");

        if extension == "prl" || extension == "pro" {
            let prog = prologue_compiler::compile(&unparsed_file, path.parent())?;
            (
                prog.dna,
                prog.grid,
                prog.orca_mode,
                prog.custom_runes,
                prog.alchemy_book,
            )
        } else if extension == "hlx" || extension == "helix" {
            (
                chimera_lang::helix_compiler::compile(&unparsed_file)?,
                None,
                None,
                HashMap::new(),
                Vec::new(),
            )
        } else if extension == "plge" || extension == "prolouge" {
            (
                chimera_lang::prolouge_compiler::compile(&unparsed_file)?,
                None,
                None,
                HashMap::new(),
                Vec::new(),
            )
        } else if extension == "tap" || extension == "tapestry" {
            (
                chimera_lang::tapestry_compiler::compile(&unparsed_file)?,
                None,
                None,
                HashMap::new(),
                Vec::new(),
            )
        } else if extension == "plg" {
            (
                prologue_esolang_compiler::compile(&unparsed_file)?,
                None,
                None,
                HashMap::new(),
                Vec::new(),
            )
        } else if extension == "score" {
            #[cfg(feature = "resonance")]
            {
                (
                    chimera_lang::acoustic_compiler::compile(&unparsed_file)?,
                    None,
                    None,
                    HashMap::new(),
                    Vec::new(),
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
                Vec::new(),
            )
        } else if extension == "lisp" || extension == "cl" {
            (
                chimera_lang::lisp::compile(&unparsed_file)?,
                None,
                None,
                HashMap::new(),
                Vec::new(),
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
                Vec::new(),
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
            Vec::new(),
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
        vm.prologue_state.alchemy_book = alchemy_book;
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

    if cli.headless || cli.json {
        for _ in 0..cli.ticks {
            if vm.halted {
                break;
            }
            vm.step();
        }

        if cli.json {
            #[derive(serde::Serialize)]
            struct CliOutput {
                stack: Vec<String>,
                log: Vec<String>,
            }

            let out = CliOutput {
                stack: vm.stack.iter().rev().map(|v| format!("{}", v)).collect(),
                log: vm.output.clone(),
            };

            println!("{}", serde_json::to_string_pretty(&out).unwrap());
            return Ok(());
        }

        use crossterm::style::{Color, Stylize};

        println!("\n{}", "✨ Execution Complete ✨".with(Color::Green).bold());
        println!(
            "{}\n",
            "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".with(Color::DarkGrey)
        );

        println!("🖥️  {}", "VM State Dashboard:".with(Color::Cyan).bold());
        println!("{}\n", vm);

        let mut table = comfy_table::Table::new();
        table
            .load_preset(comfy_table::presets::UTF8_FULL)
            .apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS)
            .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
            .set_header(vec!["Index", "Type", "Value"]);

        for (i, val) in vm.stack.iter().rev().enumerate() {
            let (type_str, type_color) = match val {
                Value::Int(_) => ("Integer", comfy_table::Color::Blue),
                Value::Str(_) => ("String", comfy_table::Color::Cyan),
                Value::Junction(_, _) => ("Junction", comfy_table::Color::Magenta),
                Value::Superposition(_) => ("Superposition", comfy_table::Color::Yellow),
                Value::Symbol(_) => ("Symbol", comfy_table::Color::Magenta),
                Value::Color(_, _, _) => ("Color", comfy_table::Color::Cyan), // 🎨 Mosaic: No raw green text
            };

            let val_str = format_oracle_result(val);

            let mut val_cell = comfy_table::Cell::new(&val_str);

            // 🎨 Mosaic: Colorize 'True' as Green and 'False' as Yellow
            if val_str == "1" || val_str.eq_ignore_ascii_case("true") {
                val_cell = val_cell.fg(comfy_table::Color::Green);
            } else if val_str == "0" || val_str.eq_ignore_ascii_case("false") {
                val_cell = val_cell.fg(comfy_table::Color::Yellow);
            } else if matches!(val, Value::Str(_)) {
                val_cell = val_cell.fg(comfy_table::Color::Magenta);
            }

            table.add_row(vec![
                comfy_table::Cell::new(i),
                comfy_table::Cell::new(type_str).fg(type_color),
                val_cell,
            ]);
        }

        println!(
            "🥞 {}",
            "Final Stack State (Top -> Bottom):"
                .with(Color::Cyan)
                .bold()
        );
        println!("{table}\n");

        println!("📜 {}", "Output Log:".with(Color::Cyan).bold());
        for line in vm.output {
            if line.contains("Error")
                || line.contains("Unknown")
                || line.contains("Warning")
                || line.contains("Failed")
            {
                println!("  ❌ {}", line.with(Color::Red).bold());
            } else if line.contains("Success")
                || line.contains("Started")
                || line.contains("Executed")
            {
                println!("  ✅ {}", line.with(Color::Green));
            } else {
                println!("  ℹ️ {}", line.with(Color::DarkGrey));
            }
        }
        println!();
    } else {
        let input_path = cli.input.as_ref().map(|s| Path::new(s).to_path_buf());

        #[cfg(feature = "nova")]
        run_tui(vm, Some(ViewMode::Prologue), input_path)?;

        #[cfg(not(feature = "nova"))]
        run_tui(vm, None, input_path)?;
    }

    Ok(())
}
