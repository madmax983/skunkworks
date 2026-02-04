pub mod mapping;
pub mod synth;
pub mod ui;

use anyhow::Context;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{env, fs, io, time::Duration};
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filepath = if args.len() > 1 {
        &args[1]
    } else {
        "experiments/code-concerto/src/main.rs"
    };

    println!("🎵 Code Concerto 🎵");
    println!("Reading {}...", filepath);

    // Check if file exists, fallback to local if running from crate root
    let content = match fs::read_to_string(filepath) {
        Ok(c) => c,
        Err(_) => {
            // Try relative to crate root if likely running from repo root
            let fallback = format!("experiments/code-concerto/{}", filepath);
             match fs::read_to_string(&fallback) {
                 Ok(c) => c,
                 Err(_) => {
                      // Try just the filename if it was "src/main.rs" and we are inside the crate
                      if filepath == "experiments/code-concerto/src/main.rs" {
                          fs::read_to_string("src/main.rs").context("Could not find source file")?
                      } else {
                          return Err(anyhow::anyhow!("Could not open file: {}", filepath));
                      }
                 }
             }
        }
    };

    println!("Parsing AST...");
    let events = mapping::CodeConcerto::generate_from_file(&content);
    println!("Generated {} musical events.", events.len());

    println!("Synthesizing audio to 'concerto.wav'...");
    let synth = synth::Synthesizer::new();
    synth.write_wav(&events, "concerto.wav").context("Failed to write WAV file")?;
    println!("WAV file generated successfully!");

    println!("Starting TUI visualization in 2 seconds...");
    std::thread::sleep(Duration::from_secs(2));

    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run App
    let app = ui::App::new(content, events);
    let res = ui::run_app(&mut terminal, app);

    // Restore Terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
