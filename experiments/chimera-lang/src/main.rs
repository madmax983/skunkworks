use anyhow::Result;
use clap::Parser as ClapParser;
use pest::Parser;
use std::fs;

use chimera_lang::{ast::Dna, compiler, tui::run_tui, vm::ChimeraVM, ChimeraParser, Rule};
use std::path::Path;

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
        compiler::compile(&unparsed_file)?
    } else {
        let dna_pair = ChimeraParser::parse(Rule::dna, &unparsed_file)?
            .next()
            .ok_or_else(|| anyhow::anyhow!("No DNA found"))?;
        Dna::from_pair(dna_pair)
    };

    let mut vm = ChimeraVM::new(dna);

    #[cfg(feature = "resonance")]
    let _stream = (|| {
        use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
        use crossbeam_channel::unbounded;
        use resonance_audio::audio::{AudioCommand, AudioModel};

        let host = cpal::default_host();
        let device = host.default_output_device()?;
        let config = device.default_output_config().ok()?;

        if config.sample_format() != cpal::SampleFormat::F32 {
            eprintln!("Unsupported sample format: {:?}", config.sample_format());
            return None;
        }

        let channels = config.channels() as usize;
        let (cmd_tx, cmd_rx) = unbounded::<AudioCommand>();
        let (snap_tx, _snap_rx) = unbounded::<Vec<f32>>();

        let mut model = AudioModel::new(16, 16, cmd_rx, snap_tx);

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = device
            .build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let frame_count = data.len() / channels;
                    let mut mono_buf = vec![0.0; frame_count];
                    model.process(&mut mono_buf);

                    for (i, frame) in mono_buf.iter().enumerate() {
                        for ch in 0..channels {
                            data[i * channels + ch] = *frame;
                        }
                    }
                },
                err_fn,
                None,
            )
            .ok()?;

        stream.play().ok()?;
        vm.set_audio_tx(cmd_tx);
        Some(stream)
    })();

    if cli.headless {
        while !vm.halted {
            vm.step();
        }

        println!("Execution complete.");
        println!("Final Stack: {:?}", vm.stack);
        println!("Output Log:");
        for line in vm.output {
            println!("  {}", line);
        }
    } else {
        run_tui(vm)?;
    }

    Ok(())
}
